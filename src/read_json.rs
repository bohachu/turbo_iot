extern crate serde;
extern crate serde_json;

pub fn read_json() {
    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let json_test: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");

    println!("json_test:{}",json_test);

}