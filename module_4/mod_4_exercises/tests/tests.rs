use solution::{Geometry, Rectangle, Circle};
use std::f64::consts::PI;

#[test]
fn check_rect_area1() {
    let x = Rectangle {length: 3.0, width: 1.0};
    assert_eq!(x.get_area(), 3.0);
}

#[test]
fn check_rect_area2() {
    let x = Rectangle {length: 3.5, width: 2.2};
    assert_eq!(x.get_area(), 7.7);
}

#[test]
fn check_circle_area1() {
    let x = Circle {radius: 1.0 };
    let expected = PI;
    assert_eq!(x.get_area(), expected);
}

#[test]
fn check_circle_area2() {
    let x = Circle {radius: 2.5 };
    let expected = 2.5 * 2.5 * PI;
    assert_eq!(x.get_area(), expected);
}
