use serde::{Serialize, Deserialize};
use std::{fs::File, collections::btree_set::Union};


#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    pub name: String,
    pub undergraduate_enrollment: u8,
    pub graduate_enrollment: u8,
    pub schools: Vec<String>,
    pub acceptance_rate: f64,
}

pub fn serialize_string_to_json(string_data: &str) -> University {
    let serialized = serde_json::from_str(string_data).unwrap();
    serialized
}

pub fn deserialize_string_from_json(json_data: &University) -> String {

    let deserialized = serde_json::to_string(&json_data).unwrap();
    deserialized

}

pub fn serialize_json_to_cbor(json_data: &University, filename: &str) {
    let file = File::create(filename).unwrap();
    //let metadata = file.metadata().unwrap();
    let serialized = serde_cbor::to_writer(file, &json_data).unwrap();
    serialized

}

pub fn deserialize_json_from_cbor(filename: &str) -> University {
    let file = File::open(filename).unwrap();
    let deserialized = serde_cbor::from_reader(file).unwrap();
    deserialized
}

