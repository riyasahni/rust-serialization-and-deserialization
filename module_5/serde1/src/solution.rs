use std::io::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::mem;

// Serializes an integer into a string
// 1. what's the difference between casting into a string and serializing into a string?
pub fn serialize_to_string(data: u32) -> String {
    let str = data.to_string();
    str
}

/// Serializes an integer into bytes
pub fn serialize_to_bytes(data: u32) -> [u8; 4] {
    let byte = data.to_be_bytes();
    byte
}

/// Reads the contents of a file and deserializes them into an integer
pub fn deserialize_from_bytes(bytes:  [u8; 4]) -> u32 {
    let num = u32::from_be_bytes(bytes);
    num

}
