/*
TASK 1
*******
Ownership & Borrowing:
Step 1: Write a function that increments a number by 1.
        Make sure to create a mutable variable that you can reference
        outside of the scope of ownerhsip.
*/

pub fn plus_one(n: &mut i32) {}


/*
TASK 2
*******
Struct and Implement

Create a Rectangle struct. Then, implement the following functions:
    1. Checks if the Rectangle is a square, and returns a boolean
    2. Calculates the area of the Rectangle.
*/

pub struct Rectangle {}

impl Rectangle {
    pub fn is_square(&self) -> bool {}

    pub fn calc_area(&self) -> u8 {}
}

/*
TASK 3
*******
Match - Create a function with a match pattern that is able to sort coins.

*/

pub enum Coin {}

pub fn coin_value(coin: Coin) -> u8 {
    0
}
