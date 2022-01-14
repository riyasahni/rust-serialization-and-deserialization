pub fn hello() -> &'static str {
    "Hello World!"
}

pub fn is_leap_year(yr: i32) -> bool {
    if {yr % 4 == 0 && yr % 100 != 0} || yr % 400 == 0 {
        return true
    } else {
        return false
    }
}
