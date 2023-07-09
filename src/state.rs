use std::{
    fs::{self, File},
    io::Read,
};

use serde_json::{json, Map, Value};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();

    json.as_object().unwrap().clone()
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);

    fs::write(file_name, new_data.to_string()).expect("Unable to write file");
}
