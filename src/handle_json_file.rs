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
    let str_comma = ",".parse().unwrap();
    let str_break_line = "\n".parse().unwrap();
    for str_line in split {
        if str_line.contains("deviceId") {
            let tokens: Vec<&str> = str_line.split("\"").collect();
            str_turbo += tokens[3];
            str_turbo.push(str_comma);
        }
        if str_line.contains("\"time\"") {
            let tokens: Vec<&str> = str_line.split("\"").collect();
            str_turbo += tokens[3];
            str_turbo.push(str_comma);
        }
        if str_line.contains("voc") {
            let tokens: Vec<&str> = str_line.split("\"").collect();
            let int_len = tokens[2].len();
            str_turbo += tokens[2][2..int_len - 1].as_ref();
            str_turbo.push(str_break_line);
        }
    }
    println!("003 一行一行讀出來進行三欄位切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut file_turbo = File::create("turbo.csv").expect("err 174");
    file_write_all(&mut file_turbo, str_turbo).expect("err 175");
    println!("004 最後寫入檔案 turbo.csv: {:?}", time.elapsed());
}

pub fn origin_to_turbo_regex() {
    let time = Instant::now();
    let mut file_json = File::open("2020_1_1_R001_臺中五百點.json").unwrap();
    let mut str_json = String::new();
    file_json.read_to_string(&mut str_json).unwrap();
    println!("001 全部JSON讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    //20200209 17:54 製作到一半太痛苦了先放棄一下
    let re = Regex::new("(\"name\":)([^,]+),").unwrap();
    let mut int_cnt = 0;
    for caps in re.captures_iter(str_json.as_ref()) {
        int_cnt += 1;
        //let str1=&cap[1];
    }
    println!("002 regex: {:?}, int_cnt:{}", time.elapsed(), int_cnt);
    /*
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    for caps in re.captures_iter(text) {
        println!("Movie: {:?}, Released: {:?}",
                 &caps["title"], &caps["year"]);
    }
    */
}

pub fn origin_to_turbo_index() {
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
    for str_line in split {
        let int_index_name = str_line.find("\"name\": \"");
        let int_index_time = str_line.find("\"time\": \"");
        let int_index_voc = str_line.find("\"voc\": \"");
        if int_index_name > Some(0) { str_turbo += str_line; }
        if int_index_time > Some(0) { str_turbo += str_line; }
        if int_index_voc > Some(0) { str_turbo += str_line; }
    }
    println!("003 運用 str_line.find 進行索引切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut file_turbo = File::create("turbo.csv").expect("err 174");
    file_write_all(&mut file_turbo, str_turbo).expect("err 175");
    println!("004 最後寫入檔案 turbo.csv: {:?}", time.elapsed());
}

pub fn read_csv() {
    let time = Instant::now();
    let mut file_csv = File::open("turbo.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 read_csv 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_csv.lines();
    println!("002 CSV 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut str_turbo: String = "".to_string();
    let int_cnt = 0;

    for str_line in split {
        let mut str_comma = str_line.split(",");
        let str_device_id: String = str_comma.next().unwrap().to_string();
        let str_date = str_comma.next().unwrap().to_string();
        let int_voc: f32 = str_comma.next().unwrap().to_string().parse().unwrap();
    }
    println!("003 CSV split_comma 區分 字串 日期 浮點數VOC，一行一行讀出來進行三欄位切割: {:?}", time.elapsed());
}

pub fn read_csv_fixed_width() {
    let time = Instant::now();
    let mut file_csv = File::open("turbo.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 read_csv 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_csv.lines();
    println!("002 CSV 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut str_turbo: String = "".to_string();
    let int_cnt = 0;

    let mut str_device_id: String = "".to_string();
    let mut str_date: String = "".to_string();
    let mut str_voc: String = "".to_string();

    for str_line in split {
        str_device_id = (&str_line[..10]).parse().unwrap();
        str_date = (&str_line[11..34]).parse().unwrap();
        str_voc = (&str_line[35..38]).parse().unwrap();
    }
    println!("003 CSV 固定寬度 fixed width 區分 字串 日期 浮點數VOC，一行一行讀出來進行三欄位切割: {:?},{},{},{}", time.elapsed(), str_device_id, str_date, str_voc);
}

pub fn read_csv_fixed_width_turbo() {
    let time = Instant::now();
    let mut file_csv = File::open("turbo.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 read_csv 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_csv.lines();
    println!("002 CSV 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();
    let mut str_turbo: String = "".to_string();
    let int_cnt = 0;

    let mut str_device_id: String = "".to_string();
    let mut str_date: String = "".to_string();
    let mut i32_voc: i32 = 0;
    for str_line in split {
        str_device_id = (&str_line[..10]).parse().unwrap();
        str_date = (&str_line[11..34]).parse().unwrap();
        i32_voc = (&str_line[35..38]).parse().unwrap();
    }
    println!("003 CSV 固定寬度 fixed width 區分 字串 日期 浮點數VOC，一行一行讀出來進行三欄位切割: {:?},{},{},{}", time.elapsed(), str_device_id, str_date, i32_voc);
}

pub fn read_csv_voc() {
    let time = Instant::now();
    let mut file_csv = File::open("data/csv/PK2CKTCE4F2GGU0FCS/2020-01_voc.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 read_csv_voc 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_csv.lines();
    println!("002 CSV 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();

    for str_line in split {
        let mut str_comma = str_line.split(",");
        let str_device_id: String = str_comma.next().unwrap().to_string();
        let str_date: String = str_comma.next().unwrap().to_string();
        let str_voc: String = str_comma.next().unwrap().to_string();
    }
    println!("003 CSV split_comma 區分 字串 日期 VOC，一行一行讀出來進行三欄位切割: {:?}", time.elapsed());
}

pub fn read_csv_vec() {
    let time = Instant::now();
    let mut file_csv = File::open("data/csv/PK2CKTCE4F2GGU0FCS/2020-01_voc.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 read_csv_voc 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_csv.lines();
    println!("002 CSV 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();

    let mut vec=Vec::new();//with_capacity(800*1024*1024);
    for str_line in split {
        let mut str_comma = str_line.split(",");
        vec.push(str_comma.next().unwrap().to_string());
        vec.push(str_comma.next().unwrap().to_string());
        vec.push(str_comma.next().unwrap().to_string());
    }
    println!("003 read_csv_vec 區分 字串 日期 VOC，一行一行讀出來進行三欄位切割: {:?}", time.elapsed());
}

pub fn read_csv_vec_turbo() {
    let time = Instant::now();
    let mut file_csv = File::open("data/csv/PK2CKTCE4F2GGU0FCS/2020-01_voc.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 read_csv_voc 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());

    let time = Instant::now();
    let split = str_csv.lines();
    println!("002 CSV 斷行切割: {:?}", time.elapsed());

    let time = Instant::now();

    let mut vec=Vec::new();//with_capacity(800*1024*1024);
    for str_line in split {
        //vec.push(str_line);
        vec.push(&str_line[0..str_line.len()-1]);
        vec.push(&str_line[0..str_line.len()-2]);
        vec.push(&str_line[0..str_line.len()-3]);
        //println!("{:?}",str_line);
    }
    println!("003, read_csv_vec_turbo, vec.push(str_line), {:?}", time.elapsed());
}

pub fn read_csv_vec_fix() {
    let time = Instant::now();
    let mut file_csv = File::open("data/csv/PK2CKTCE4F2GGU0FCS/2020-01_voc.csv").unwrap();
    let mut str_csv = String::new();
    file_csv.read_to_string(&mut str_csv).unwrap();
    println!("001 全部 CSV 讀取到記憶體當中: {:?}", time.elapsed());
    let mut vec=Vec::new();//with_capacity(800*1024*1024);
    let time = Instant::now();
    let mut i =0;
    loop{
        i+=13;
        if i+13>=str_csv.len(){
            break;
        }
        vec.push(&str_csv[i..i+10]);
    }
    println!("003, read_csv_vec_fix, vec.push(str_line), {:?}", time.elapsed());
}