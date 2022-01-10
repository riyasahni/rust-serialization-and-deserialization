use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Error;
use rand;
use rand::Rng;

fn serialize_data_to_disk(data: Vec<i32>, filename: &str) -> Result<(), Error> {
    // TODO: to implement by students
    // Create a File; see Rust doc for std::fs::File
    let mut buffer = File::create(filename)?;

    // First write vector size
    let vector_size: i32 = data.len() as i32;
    // Write bytes to that file
    buffer.write_all(&vector_size.to_be_bytes())?;
    // Then write data
    let iter = data.iter();
    for i in iter {
        buffer.write_all(&i.to_be_bytes())?;
    }
    Ok(())
}

fn deserialize_data_from_disk(filename: &str) -> Vec<i32> {
    // TODO: to implement by students.
    let f = File::open(filename).expect("could not open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    let mut four_bytes:[u8;4] = [0,0,0,0];
    // Read vector size
    reader.read(&mut four_bytes);
    let size = i32::from_be_bytes(four_bytes);
    // Now read each of the integers written to disk
    for i in 1..size {
        reader.read(&mut four_bytes);
        let data_item = i32::from_be_bytes(four_bytes);
        buffer.push(data_item);
    }
    buffer

}

fn main() {
    println!("Serializing and Deserializing vectors");
    // Change this variable to serialize data (true) or deserialize it (false)
    let serialize = false;
    let filename = "data.bin";

    if serialize {
        let mut rng = rand::thread_rng();
        let n1: u32 = rng.gen_range(1500..10000);
        let mut counter = 0;
        let mut data = Vec::new();

        for i in 1000..n1 {
            data.push(counter);
            counter += 1;
        }
        serialize_data_to_disk(data, &filename);
    }
    else{
        let data = deserialize_data_from_disk(&filename);
        // TODO: to fill in by students
        println!("The size of the array is: {}", data.len());
        println!("This is the data:");
        for i in data.iter() {
            println!("Element: {}", i);
        }
        println!("The size of the array is (again): {}", data.len());
    }

}
