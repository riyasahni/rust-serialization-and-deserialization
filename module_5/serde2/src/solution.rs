use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Error;

pub fn serialize_data_to_disk(data: Vec<i32>, filename: &str) -> Result<(), Error> {

    //iterate through vector of ints 
    //serialize ints to bytes
    
    let mut buffer = File::create(filename)?;
      for i in data.iter(){
        let res = &i.to_be_bytes();
        buffer.write_all(res)?;
    }
  // Write bytes to that file
  Ok(())

}

pub fn deserialize_data_from_disk(filename: &str) -> Vec<i32> {
    
    let f = File::open(filename).expect("could not open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer).expect("error while reading file");

    let mut vector: Vec<i32> = Vec::new();
    
    let mut count = 0;

    while(count<buffer.len()) {
        let new_int = i32::from_be_bytes([buffer[count],buffer[count+1], buffer[count+2], buffer[count+3]]);
        vector.push(new_int);
        count+=4;
    }
  vector


}
