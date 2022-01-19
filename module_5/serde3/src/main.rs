use solution::{University,
               serialize_string_to_json,
               deserialize_string_from_json,
               serialize_json_to_cbor,
               deserialize_json_from_cbor};


fn main() {
    let data = r#"
        {
            "name": "University of Chicago",
            "undergraduate_enrollment": 7559,
            "graduate_enrollment": 10893,
            "schools": [
                "Biological Sciences Division",
                "Chicago Booth School of Business",
                "Crown Family School of Social Work, Policy, and Practice",
                "Divinity School",
                "Graham School of Continuing Liberal and Professional Studies",
                "Harris School of Public Policy",
                "Humanities Division",
                "Law School",
                "Physical Sciences Division",
                "Pritzker School of Medicine",
                "Pritzker School of Molecular Engineering",
                "Social Sciences Division"
            ],
            "acceptance_rate": 0.07
        }"#;

    // convert string to json
    let uchicago: University = serialize_string_to_json(data);


    // convert json to string
    let serialized = deserialize_string_from_json(&uchicago);
    println!("serialized = {}", serialized);

    let filename = "uchicago.cbor";

    serialize_json_to_cbor(&uchicago, filename);

    let uchicago_from_cbor: University = deserialize_json_from_cbor(filename);
    println!("{:?}", uchicago_from_cbor);
}