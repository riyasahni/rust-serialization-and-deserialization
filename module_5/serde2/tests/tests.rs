use solution::{serialize_data_to_disk, deserialize_data_from_disk};

#[test]
fn test_serialize_deserialize_data_to_disk() {
    let n1: u32 = 100000;
    let mut counter = 0;
    let mut data = Vec::new();
    let filename = "data_test.bin";

    for i in 0..n1 {
        data.push(counter);
        counter += 1;
    }
    serialize_data_to_disk(data, &filename);

    let data = deserialize_data_from_disk(&filename);

    assert_eq!(n1 as usize, data.len());
}
