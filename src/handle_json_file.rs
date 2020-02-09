use serde_json::{Result, Value};

extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::io::{Read, BufReader, BufRead};

use std::fs::File;
use std::io::Write;
use regex::bytes::Regex;
use std::io;
use std::time::Instant;
use std::path::Path;

pub fn write_json_file() -> std::io::Result<()> {
    let str_json = r#"
    {
      "@timestamp": "2019-12-11T11:41:56+08:00",
      "ampere": 0.0,
      "cameo_projects": [
        "561.taichung_500_iot"
      ],
      "cameo_tags": [
        "台中",
        "rest"
      ],
      "co": 0.927191601564175,
      "deviceId": "6201492301",
      "devstat": 0.0,
      "humidity": 66.78,
      "humidity_main": 57.39,
      "lat": 24.222725,
      "location": {
        "lat": 24.222725,
        "lon": 120.52705
      },
      "lon": 120.52705,
      "name": "TW010108A0100347",
      "no2": 14.636371996508107,
      "noise": null,
      "o3": 2.12659425010402,
      "pm1": 17.0,
      "pm10": 28.0,
      "pm2_5": 19.64,
      "temperature": 25.08,
      "temperature_main": 27.22,
      "time": "2019-12-11 11:41:56.000",
      "voc": 485.0,
      "volt": 0.0
    },
    "#;

    for _i in 1..2 {
        write_file(str_json, format!("2020_1_{}_R001_臺中五百點.json", _i).as_ref()).expect("err handle_json_file 42");
    }
    Ok(())
}

fn write_file(str_json: &str, str_region_name: &str) -> std::io::Result<()> {
    let mut file = File::create(str_region_name)?;
    let content = get_content(str_json);
    file_write_all(&mut file, content).expect("err file_write_all 52");
    Ok(())
}

fn file_write_all(file: &mut File, content: String) -> std::io::Result<()> {
    file.write_all(content.as_ref())?;
    Ok(())
}

fn get_content(str_json: &str) -> String {
    let mut content: String = "[".parse().unwrap();
    for _i in 0..60 * 24 * 500 {
        content += str_json;
    }
    content += "{}]";
    content
}

pub(crate) fn read_file_rustc_serialize() {
    let mut file = File::open("2020_1_1_R001_臺中五百點.json").unwrap();
    let mut str_json = String::new();
    file.read_to_string(&mut str_json).unwrap();
    let i = str_json.len();
    println!("i:{}", i);
    let json_obj = Json::from_str(&str_json).unwrap();
    //println!("{}", json_obj.find_path(&["Address", "Street"]).unwrap());
}

pub(crate) fn read_file_serde() {
    let mut file = File::open("2020_1_1_R001_臺中五百點.json").unwrap();
    let mut str_json = String::new();
    file.read_to_string(&mut str_json).unwrap();
    let i = str_json.len();
    println!("i:{}", i);
    let v: Value = serde_json::from_str(str_json.as_ref()).expect("err 91");
}

pub(crate) fn regex() {
    let mut file = File::open("2020_1_1_R001_臺中五百點.json").unwrap();
    let mut str_json = String::new();
    file.read_to_string(&mut str_json).unwrap();
    let i = str_json.len();
    println!("i:{}", i);

    let re = Regex::new(r"(pm2_5)").unwrap();
    let mut i = 0;
    for cap in re.captures_iter(str_json.as_ref()) {
        i += 1;
    }
    println!("pm25:{:?}", i);
}

pub fn readlines2() {
    let mut int_cnt = 0;
    if let Ok(lines) = read_lines("2020_1_1_R001_臺中五百點.json") {
        for line in lines {
            if let Ok(str_line) = line {
                if str_line.contains("pm2_5") {
                    int_cnt += 1;
                }
            }
        }
    }
    println!("int_cnt: {}", int_cnt);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn origin_to_turbo() {
    let mut file_turbo = File::create("turbo.csv").expect("err origin_to_turbo");
    if let Ok(lines) = read_lines("2020_1_1_R001_臺中五百點.json") {
        for line in lines {
            if let Ok(str_line) = line {
                if str_line.contains("\"deviceId\"") {
                    file_turbo.write(str_line.as_ref()).expect("err file_turbo.write 001");
                }
                if str_line.contains("\"time\"") {
                    file_turbo.write(str_line.as_ref()).expect("err file_turbo.write 002");
                }
                if str_line.contains("\"voc\"") {
                    file_turbo.write(str_line.as_ref()).expect("err file_turbo.write 003");
                }
            }
        }
    }
    file_turbo.flush();
}

pub fn origin_to_turbo_memory() {
    let time = Instant::now();
    let mut file_json = File::open("2020_1_1_R001_臺中五百點.json").unwrap();
    let mut str_json = String::new();
    file_json.read_to_string(&mut str_json).unwrap();
    println!("001 全部JSON讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_json.split("\n");
    println!("002 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut str_turbo: String = "".to_string();
    let str_comma=",".parse().unwrap();
    let str_break_line="\n".parse().unwrap();
    for str_line in split {
        if str_line.contains("deviceId") {
            let tokens:Vec<&str>= str_line.split("\"").collect();
            str_turbo += tokens[3];
            str_turbo.push(str_comma);
        }
        if str_line.contains("\"time\"") {
            let tokens:Vec<&str>= str_line.split("\"").collect();
            str_turbo += tokens[3];
            str_turbo.push(str_comma);
        }
        if str_line.contains("voc") {
            let tokens:Vec<&str>= str_line.split("\"").collect();
            let int_len=tokens[2].len();
            str_turbo += tokens[2][2..int_len-1].as_ref();
            str_turbo.push(str_break_line);
        }
    }
    println!("003 一行一行讀出來進行三欄位切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut file_turbo = File::create("turbo.csv").expect("err 174");
    file_write_all(&mut file_turbo, str_turbo).expect("err 175");
    println!("004 最後寫入檔案 turbo.csv: {:?}", time.elapsed());
}