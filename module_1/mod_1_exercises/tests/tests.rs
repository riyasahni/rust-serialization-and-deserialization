use solution::{hello, is_leap_year};


#[test]
fn check_hello() {
    assert_eq!(hello(), "Hello World!");
}

#[test]
fn check_leap_year1() {
    assert!(is_leap_year(2020));
}

#[test]
fn check_leap_year2() {
    assert_eq!(is_leap_year(1557), false);
}

#[test]
fn check_leap_year3() {
    assert_eq!(is_leap_year(1900), false);
}

