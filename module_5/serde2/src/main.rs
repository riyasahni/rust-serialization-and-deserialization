use rand;
use rand::Rng;

use solution::{serialize_data_to_disk, deserialize_data_from_disk};


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
        println!("The size of the array is: {}", data.len());
        println!("This is the data:");
        for i in data.iter() {
            println!("Element: {}", i);
        }
        println!("The size of the array is (again): {}", data.len());
    }

}
