use std::time::SystemTime;

pub fn speed_test_rust3() {
    let mut str1 = String::from("");
    let mut int_count = 0;
    loop {
        int_count += 1;
        if int_count > 123456789 {
            break;
        }
        str1.push('{');
        str1.push('"');
        str1.push('p');
        str1.push('m');
        str1.push('2');
        str1.push('5');
        str1.push('"');
        str1.push(':');
        str1.push('1');
        str1.push('0');
        str1.push('0');
        str1.push('}');
    }
}

pub fn speed_test_rust2() {
    let mut str1 = String::from("");
    let mut int_count = 0;
    loop {
        int_count += 1;
        if int_count > 123456789 {
            break;
        }
        str1.push_str("{\"pm25\":100}");
    }
}

pub fn speed_test_rust1() {
    let mut str1 = String::from("");
    for _n in 0..123456789 {
        str1 += "{\"pm25\":100}";
    }
}

fn main() {
    println!("Hello, world111:{:?}",SystemTime::now());
    speed_test_rust3();
    println!("Hello, world222:{:?}",SystemTime::now());
}




