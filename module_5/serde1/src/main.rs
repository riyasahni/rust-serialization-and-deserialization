use std::io::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use solution::{serialize_to_string, serialize_to_bytes, deserialize_from_bytes};

fn write_bytes_to_file(bytes: [u8; 4], filename: &str) -> Result<(), Error> {
    // Create a File; see Rust doc for std::fs::File
    let mut buffer = File::create(filename)?;
    // Write bytes to that file
    buffer.write_all(&bytes)?;
    Ok(())
}

fn read_bytes_from_file(filename: &str) -> [u8; 4] {
    let f = File::open(filename).expect("could not open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer).expect("error while reading file");

    // Transform Vec (much preferred way of handling collection of values) into array (for this example)
    let array = vec_to_array(buffer);
    array
}

fn write_string_to_file(string: &str, filename: &str) {
    let f = File::create(filename).expect("error creating file");
    let mut f = BufWriter::new(f);
    f.write_all(string.as_bytes());
}

fn read_string_from_file(filename: &str) -> String {
    let mut data = String::new();
    let f = File::open(filename).expect("error while opening file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data);
    data
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn main() {
    println!("Basic Integer serialization, deserialization");
    // Change this variable to serialize data (true) or deserialize it (false)
    let serialize = true;
    let bytes_filename = "test.bytes";
    let string_filename = "test.txt";
    let integer: u32 = 33;
    if serialize {
        // We obtain a human-readable representation of the data (integer) to store
        let integer_in_string = serialize_to_string(integer);
        // Then we store the string into a file
        write_string_to_file(&integer_in_string, &string_filename);

        // We obtain a byte representation of the data (integer) to store
        let integer_in_bytes = serialize_to_bytes(integer);
        // Then we store the byte representation on a file on disk
        write_bytes_to_file(integer_in_bytes, &bytes_filename);
    }
    else {
        // We read string from file (we can deserialize it directly into a rust string)
        let data = read_string_from_file(&string_filename);
        println!("The (string) deserialized integer is: {}", data);

        // We read bytes from a file
        let read_bytes = read_bytes_from_file(&bytes_filename);
        let deserialized_integer = deserialize_from_bytes(read_bytes);
        println!("The (bytes) deserialized integer is: {}", deserialized_integer);
    }
}
