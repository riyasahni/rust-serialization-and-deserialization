# Module 2
- [Ownership](#ownership)
- [Structures](#structuresstructs)
- [Implementation & Methods](#implementation-and-methods-with-structs)
- [Derive](#derive)
- [Algebraic Data Types](#algebraic-data-types)
- [Module 2 Exercises](#module-2-exercises)

## Ownership
[Link to Rust documentation on ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

### What is ownership?

**Primer on memory allocation:** Programs need to allocate memory to do computation. In C environments you will be familiar with the malloc (memory allocation) function to achieve that goal. If a program allocates memory and never 'frees' it after using it, it will soon exhaust the machine's memory leading to a crash. In C environments developers are in charge of allocating memory and freeing it when no longer necessary. This gives developers full control at the expense of increased complexity. Reducing that complexity is not just a matter of convenience, but of security and safety as well. In response to the complexity of manual memory allocation, some language runtimes incorporate a garbage collector. With a garbage collector, developers no longer need to allocate and free memory explicitly. Instead, memory is allocated on demand as you declare variables, and the garbage collector will search for references to memory that is no longer used and free them transparently to the developer. This increased convenience comes at the cost of performance.

Rust aims to help developers write safe programs without using a garbage collector. Striking a balance between convenience and performance. Developers only need to learn a few additional concepts to take full advantage of Rust's powerful model. The concept of ownership is at the center of it all.

To recap: Ownership is one of the most important and unique features of Rust; it allows Rust to be memory-safe and efficient without needing a garbage collector. 

In Rust, whether the value is on the stack or the heap is important to understand how Rust behaves as a language. This is where ownership comes in. 
- Ownership Rules:
    - Each value in Rust has a variable that's called its owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped

For example, when you assign a value to a variable, that variable becomes the value's sole owner.

```rust
fn main() {
    let x = String::from("hello");
}
```

Now, let's say we define a new owner of x's values:

```rust
fn main() {
    let x = String::from("hello");
    let y = x;
}
```

This reassignment of ownership is known as a move. A move causes the former assignee to become uninitialized, and therefore cannot be usable in the future.

The last rule of ownership deals with scope. When a variable goes out of scope, its associated value is dropped. For as long as a variable remains in scope, the value it owns will never be dropped. For example:

```rust
fn main() {
    {
        let x = 24;
    }
    println!("x: {}", x);  //ERROR: x is no longer in scope
}
```

### References and borrowing

References and borrowing allows developer to use variables without taking ownership.

#### Borrowing

To borrow a value without taking ownership of a value, you can add a ```&``` to the value. For example: (FYI vec! is a macro/function to create a vector which is a growable array/list)

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    // Borrow the numbers by using &numbers
    let borrowed_numbers = &numbers; 
    // Now, you are able to use borrowed_numbers with no error
    for number in borrowed_numbers.iter() {
        println!("Number: {}", number);
    }

}
```

#### References

Rust lets us pass values to functions by reference, or by copying the value itself.

1. If a value implements a Copy and is not borrowed, it will be passed by value (copied)
2. If a value implements a Copy and is borrowed, it will be passed by reference
3. If a value does not implement a Copy, it must be borrowed to be passed by reference

For example, consider different ways of checking whether a number is negative or a vector empty using references and copies:

```rust
fn pass_number_by_reference(number_arg: &i8) -> bool {
    number_arg.is_negative()
}

fn pass_number_by_value(number_arg: i8) -> bool {
    number_arg.is_negative()
}

fn pass_vec_by_reference(vec: &Vec<i8>) -> bool {
    vec.is_empty()
}

fn main() {
    // numbers implement Copy, and so can be passed by value or reference
    let number = 42;

    // pass by reference
    // does not move number because of borrow
    let is_negative_by_ref = pass_number_by_reference(&number);
    
    // pass by value
    // the `number` is copied to `number_arg`
    // so `number` and `number_arg` are two independent variable at two places in memory.
    let is_negative_by_value = pass_number_by_value(number);

    // copy not implemented - must be passed by reference
    let vec = vec![];

    // does not move vec
    let is_empty = pass_vec_by_reference(&vec);
}
```

As you can see, the comments above the functions are a good description of the relationship between ownership, referencing, and borrowing. 

### The Slice Type

Rust also allows you to reference values using slicing. Slicing applies to collection types such as string, or list. From the Rust book "*Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.*"

For example:
 
```rust
let s = String::from("hello");

// You can slice from a range
let slice = &s[0..2];

// Or you can slice the whole string
let slice = &s[..];
```

### Lifetimes

Any reference has a lifetime that indicates how long the reference is valid. Most of the time this will be inferred by the rust compiler, but it can be explicitly given. We have avoided using lifetimes when possible in our project to simplify the code. This results in using smart pointers more than is ideal. You can read more about lifetimes [here](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), so you are familiar with the idea by the time you encounter them in Crusty's codebase.

## Structures/Structs

[Link to Rust documentation for Struc & Impl ](https://doc.rust-lang.org/book/ch05-00-structs.html)

A ```struct```, or structure, is a data type in Rust that is composed of data items of different types, including other structures. A structure defines its data as a key-value pair.

### Declaring and Instantiating a ```struct```

To declare a structure, we use the keyword `struct` followed by the name of the structure. Then, inside the curly brackets, we define the names and types of the pieces of data. For example:

```rust
struct name_of_struct {
    field1: data_type
    field2: data_type,
    field3: data_type,
}
```

After declaring a ```struct```, we create an instance of that struct and specify concrete values for the fields and values. For example:

```rust
// declaring an employee structure
struct Employee {
    name: String,
    company: String, 
    age: u32,
    active: Boolean
}
//creating an instance of the struct
fn main() {
    let emp1 = Employee {
        name: String::from("Some Name"),
        company: String::from("Some Company"),
        age: 27,
        active: True
    };
}
```

In an instance, we define the ```key: value``` pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. 

There is a common shorthand notation when initializing a struct that you can pass a variable with the same name as the member of the struct. 

```rust
let age = 27;
let active = true;
let emp1 = Employee {
        name: String::from("Some Name"),
        company: String::from("Some Company"),
        age,
        active
    };
```

To get a specified value from a struct, we can use dot notation. For example, if we wanted the employee's name, we would call
``` emp1.name ``` whenever we wanted to use this value.

To be able to modify an instance, the instance variable must be marked as mutable. A reminder on how to make your variables mutable is by adding ```mut``` in front of the variable name. If we want to modify a specific ``` key: value ``` pair, we would again use dot notation. For example, if we wanted to modify the company name of emp1, we would call:

``` 
emp1.company = String::from("Different Company");
```

after making the appropriate value mutable.

## Implementation and Methods with Structs

The ```impl``` keyword is primarily used to define implementations on types. Both functions and consts can be defined in an implementation. For example, to implement a rectangle struct, we could do:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

This example allows us to call rectangle.area on an instance of the struct.

```rust
let rec = Rectangle{ 
    width=10,
    height=20,
};
println!("Rectangle Area: {}", rec.area());
```

### Methods

Methods are functions that are associated with a given type. Usually, the type is a ```struct```, and so you can think of methods as the functions you define to operate with the data represented by a struct. The first argument of methods will usually be a reference to the type on which they operate, via ```self, &self, or &mut self```. For example, if the method is defined over a struct, then self is the instance of the struct that the method is called on.

#### Defining Methods:

An example of how to define a method using the Rectangle struct is below.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
In this example, area is the function called in main, and used on ```rect1```, which we know is a Rectangle. Using & in front of our methods allows us to not take ownership, and rather borrow ```self``` immutably. 

Methods in an impl that do not take self as an argument are typically used for constructors, and return `Self` as a type. You invoke these functions by using the stuct name::function name (e.g. `Rectangle::new()`)

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new_square_rect(n: u32) -> Self {
        Rectangle {
            width: n,
            height: n
        }
    }
}

let r = Rectangle::new_square_rect(4);
```

## Derive 

[Derive](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) is an annotation you can put on a struct that will automatically generate functions for the impl of the struct. Common derives will be debug (generate a string when called with {:?} in a string format), eq/partialEq (check equality), clone (create a deep copy), copy (stack only copy), hash (generate a hash code), and Ord/PartialOrd (comparing ordering).

If we wanted to generate equality checks and debug on our rectangle struct we would add

```
#[Derive(Debug, Eq, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

and if r1 and r2 were Rectangles we could invoke code like:

```
if r1 == r2 {
    println!("Rectangles are the same {:?}", r1);
}
```

## Algebraic Data Types

### Enums
An ```enum``` is a data type that allows you to define a type by enumerating its possible variants. The options of an enum can optionally hold variables (primitive types, structs, or a tuple of values).  

Consider we have a message type that can either be a hello, goodbye, body, or wait. The body type has a string associated with it, and wait has an integer that specifies how long to wait.

```rust
enum Message {
    Hello,
    Goodbye,
    Body(String),
    Wait(String,u32)
}
```

Therefore when dealing with a message enum/variable you know it *must* be one of these four options, and if it is a body it must have a string and if it is a wait, it must have a u32.

```rust
let m1 = Message::Hello;
let m2 = Message::Body(String::from("Meet me at Medici"));
let m3 = Message::Wait(String::from("Seconds",10);
let m4 = Message::Goodbye;
```

### Match

If you are not used to languages with a pattern-matching feature, you'll love the transition to Rust. Match expressions allow you to compare a value against a series of patterns. 

For example, this match expression would print "three"

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

Rust also has a pattern we can use to match any value. The ```_``` pattern will match all possible cases that aren't specified before it. For example:

```rust
let x = 52;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

This would print "something else" because 52 does not match the other patterns listed.

Matching is common with enums as it ensures that all possible matches are accounted for (or at least a wildcard captures cases). Using our message example a match function like the following will cause an error as we do not match Message::{Goodbye,Wait,Body}

```rust
fn print_msg(m: Message) {
    match m {
        Message::Hello => {
            println!("Hi");
        }
    }
}
```

Instead we would want to match all Messages as follows. A few things to note, the code after the arm (=>) can be a code block { } or a single line, but both are followed by a , that separates the matches.  Also note that on the match of enum types that have store values, the values are available in a variable named after the type (e.g. `Message::Body(msg)` extracts the String which is available in the variable `msg` for the code block.).

```rust
fn print_msg(m: Message) {
    match m {
        Message::Hello => {
            println!("Hi");
        },
        Message::Goodbye => println!("Bye"),
        Message::Body(msg) => {
            println!("{}",msg);
        },
        Message::Wait(time,len) => println!("Waiting for {} {}",len,time)
    }
}
```

In the next module we will explore two common enums Option and Result.

### if let

Sometimes you want to only check a single condition, in this case writing an `if let` can be concise.

```rust
if let Message::Wait(time,len) = m3 {
    println!("Waiting for {} {}",len,time);
}
```

Matches are a powerful tool. You can use them when assigning variables or for returning from a function.

Read more about enums and matching in the [Rust Book](https://doc.rust-lang.org/book/ch06-00-enums.html)



## [Module 2 Exercises]
As a quick note: you may need to make several fields/functions public via the ```pub``` keyword to allow it to be seen by the autograder

1. Ownership task. Define a function ```plus_one``` which takes in a reference to a mut i32 and increments it. 
2. Struct & Impl:
    Create a ```Rectangle``` struct with a width and height. Next, implement the following two functions:
        1. ```is_square``` Checks if the rectangle is a square
        2. ```calc_area``` Calculates the area of the rectangle
3. Match:
    1. Define an enum Coin with values for Penny, Nickel, Dime, and Quarter
    2. Write a function ```coin_value``` which takes in a Coin, performs a pattern match, and then returns the value of that Coin in cents (i.e. if the input is a Penny the function returns 1).
