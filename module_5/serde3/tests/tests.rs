use solution::{University,
               serialize_string_to_json,
               deserialize_string_from_json,
               serialize_json_to_cbor,
               deserialize_json_from_cbor};

#[test]
fn test_serialize_deserialize_string_json() {
    let data = r#"
        {
            "name": "University of Chicago",
            "undergraduate_enrollment": 10,
            "graduate_enrollment": 10,
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

    let uchicago: University = serialize_string_to_json(data);
    assert_eq!(uchicago.undergraduate_enrollment, 10);
    assert_eq!(uchicago.schools.len(), 12);
}

#[test]
fn test_serialize_deserialize_json_cbor() {
    let data = r#"
        {
            "name": "University of Chicago",
            "undergraduate_enrollment": 50,
            "graduate_enrollment": 50,
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

    let uchicago: University = serialize_string_to_json(data);
    let filename = "uchicago_test.cbor";
    serialize_json_to_cbor(&uchicago, filename);
    let uchicago_from_cbor: University = deserialize_json_from_cbor(filename);

    assert_eq!(uchicago_from_cbor.graduate_enrollment, 50);
    assert_eq!(uchicago_from_cbor.acceptance_rate, 0.07);
}
