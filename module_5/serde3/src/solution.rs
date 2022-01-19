use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;


#[derive(Debug, Serialize, Deserialize)]
pub struct University {

}

pub fn serialize_string_to_json(string_data: &str) -> University {
    
}

pub fn deserialize_string_from_json(json_data: &University) -> String {
    
}

pub fn serialize_json_to_cbor(json_data: &University, filename: &str) {
    
}

pub fn deserialize_json_from_cbor(filename: &str) -> University {
    
}

