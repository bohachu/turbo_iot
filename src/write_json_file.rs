use std::fs::File;
use std::io::Write;

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
    }
    "#;

    for _i in 1..2 {
        write_file(str_json, format!("2020_1_{}_R001_臺中五百點.json", _i).as_ref()).expect("err write_json_file.rs 42");
    }
    Ok(())
}

fn write_file(str_json: &str, str_region_name: &str) -> std::io::Result<()> {
    let mut file = File::create(str_region_name)?;
    write!(file, "[")?;
    let content = get_content(str_json);
    file_write_all(&mut file, content).expect("err file_write_all 52");
    Ok(())
}

fn file_write_all(file: &mut File, content: String) -> std::io::Result<()> {
    file.write_all(content.as_ref())?;
    Ok(())
}

fn get_content(str_json: &str) -> String {
    let mut content: String = "".parse().unwrap();
    for _i in 0..60 * 24 * 500 {
        content = content + str_json;
    }
    content
}
