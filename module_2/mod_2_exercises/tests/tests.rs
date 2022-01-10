use solution::{plus_one, Rectangle, Coin, coin_value};

#[test]
fn test_incr() {
    let mut n: i32 = 9;
    plus_one(&mut n);
    assert_eq!(n, 10);
}

#[test]
fn test_square_struct() {
    let r = Rectangle {
        width: 50,
        height: 25,
    };
    assert_ne!(r.is_square(), true);
}

#[test]
fn test_square_struct2() {
    let r = crate::Rectangle {
        width: 25,
        height: 25,
    };
    assert_eq!(r.is_square(), true);
}

#[test]
fn test_area_struct() {
    let r = Rectangle {
        width: 4,
        height: 8,
    };
    assert_eq!(r.calc_area(), 32);
}

#[test]
fn test_coins() {
    assert_eq!(coin_value(Coin::Nickel), 5);
}


