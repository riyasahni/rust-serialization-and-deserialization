# Module 1
- [Installation](#installation)
- [Basic Data Types](#basic-data-types)
- [Control Flow](#control-flow)
- [Common Data Structures](#common-data-structures)
- [Cargo](#cargo)
- [Debugging](#debugging-rust-programs)
- [Module 1 Exercises](#module-1-exercises)

## Installation

Installing Rust is easy in most platforms due to the environment, ```rustup``` it includes. **rustup** will help you check for updates and update your Rust environment when necessary. If you install Rust 1.57 or greater at the beginning of the quarter, we do not anticipate you will need to use rustup the rest of the quarter.

To install Rust, we downloand it through ```rustup```.

### Install ```rustup``` on Linux or macOS
Enter the following command in terminal:

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

A successful installation will return: 

```
Rust is installed now. Great!
```

### Update existing Rust environment

You can run:

```
rustc --version
```

to both check if you already have Rust installed, and if so, which version. If you don't have it installed, go above and follow the instructions to install it. If you already have a Rust environment installed, then you can update the version by doing:

```
rustup update
```

### Install ```rustup``` on Windows
For Windows, go to https://forge.rust-lang.org/infra/other-installation-methods.html and follow the instructions on how to install Rust for Windows.

## Basic Data Types:
[Link to Rust documentation for Basic Data Structures](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

### Variables

Rust contains many of the same variables used in other programming languages.
These variables include:
- integer: ```int``` ```(u8,i8,u16,i16...u64/usize)```
- floating-point ```f32,f64```
- boolean ```bool```
- character ```char```
- tuple ```tup```
- array (not frequently used though, Vector is much more common)   

By default, variables are immutable, which means that once you assign a value you cannot modify (mutate) it anymore. However, you have the option to make your variables mutable by adding ```mut``` in front of the variable name. 

When defining a variable, sometimes the compiler can infer the type.

`let x = 14;`

Other times it cannot and requires you provide the type

`let x: u8 = 14;`

Read more in the [Rust Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

### Functions

Functions in Rust are defined as:

```rust
fn main() {
    //expression;
}
```

Functions are private by default. To make them public, add ```pub``` before ```fn```.

```rust
pub fn main() {
    //expression;
}
```

Functions can have a return type that is indicated with -> after the name. For example `fn square(n: u64) -> u64`

The return can be explicitly given with `return [expression or variable];` or it can be the last line of a function with no semicolon:

```rust
fn square(n: u64) -> u64 {
    return n * n;
}
```

or

```rust
fn square(n: u64) -> u64 {
    n * n
}
```

Read more in the [Rust Book](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Control Flow

### ```if``` Expressions:

- An ```if``` expression allows you to run code depending on conditions. Optionally, we can include an ```else``` expression, which will execute the code if the condition evaluates to false. 
- You can handle multiple conditions by using an ```else if``` expression
- ```if``` expressions can also be used in a ```let``` statement to assign a variable dependent on conditions
- Example:
```rust
fn main() {
    let number = 10;
    
    if number % 5 == 0 {
        println!("number is divisble by 5");
    } else {
        println!("number is not divisble by 5");
    }
}
```

### ``` loop```

- ```loop``` allows you to execute a block of code repeatedly forever, or until you explicitly tell it to stop. 
- You can use the ```break``` keyword within the loop to tell the program when to stop executing the loop
    - If you want to have the ```loop``` return a value, you can add the value you want returned after the ```break``` expression
- Example:
```rust
fn main() {
    let mut counter = 0;

    loop { 
        counter += 1
        if counter == 10 {
            break counter;       // will break the loop and return counter value
        }
    }
}
```

### ``` while``` conditional loop

- A ```while``` loop allows you to run the loop while the condition is true, and when the condition ceases to be true, the loop will break
- Example:
```rust
fn main() {
    let mut number = 5;
    while number > 0 {
        println!(number);
        number -= 1
    }
}
```

### ``` for``` loop

- A ```for``` loop executes code for each item in a collection

## Common Data Structures

Rust has many useful data structures built into the language. Two common data structures you are likely to use / encounter are vectors and hashmaps. A vector (vec) is a growable list/array. A hashmap is an associative map that lets you insert and look up structs/variables by a key.

### Vec

Vectors allow you to insert elements into them (push), iterate through elements in the vector, get an element from the vector, and remove the last element (pop). See the [API](https://doc.rust-lang.org/std/vec/struct.Vec.html) for the full set of functions.

```rust
let mut v: Vec<i32> = Vec::new();    
v.push(5);
v.push(6);
v.push(7);
v.push(8);

//Use &v to borrow the vector -- we'll see in Module 2 what 'borrow' means in Rust
for x in &v {
    println!("{}", x);
}

// assert_eq! checks two values are equal. We will cover Some later.
assert_eq!(vec.pop(), Some(8));
// Check the length of the vec
assert_eq!(vec.len(), 3);
```

A macro, vec!, exists for quickly building a vector
`let mut v = vec![1, 2, 3];`

### Hashmap 

Read the documentation on hashmaps when you need to use them
https://doc.rust-lang.org/book/ch08-03-hash-maps.html

## Cargo
[Link to Rust documentation for cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

Cargo is Rust's build system and package manager. It allows you to:

- Build your software
- Declare software dependencies your project needs
- Download those dependencies automatically

If you come from Python are used to rely on pip/conda or other environments to manage your dependencies, you'll find Cargo extremely useful. If you come from a JVM-based language such as Java or Scala, you'll find Cargo does much of what Maven/Gradle/Ant does for JVM languages, an all-in-one environment. If you come from C or C++ you may enjoy not having to deal with Makefile anymore :)

To make sure you have cargo installed, enter the following command into the terminal:
``` cargo --version ```

### Creating a Project (crate) with Cargo

To create a project with cargo, run the following commands, which create a `crate`. A crate is a package of code that can either be a binary (executable) or a library.

```
cargo new hello_cargo
cd hello_cargo
```

In this example, we will be working with a new (binary) crate named hello_cargo. When you cd into the directory,
you should see that Cargo has generated two files:
- Cargo.toml
- main.rs file inside the src directory

Open Cargo.toml in a text editor. You should see the configuration information for the package, and the information that Cargo needs to compile your program. Make sure your name and email are correct.

```
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Name <email@uchicago.edu>"]
edition = year

[dependencies]
```

Next, open src/main.rs. You should see that Cargo has generated a Hello World program for you. 

### Building and Running a Cargo Project

 - To build your cargo project run: ``` cargo build ```
 - To build your cargo and run tests (and compile tests) run ```cargo test```
 - If you crate is a binary you can run it (and compile it) with: ``` cargo run ```
 - If you just want to check your code to see if it compiles, but don't want to run it, you can run ``` cargo check ```
 - To build your code with a higher level of optimization run ``` cargo build --release ``` This will take longer but result in 'faster' code. 

### Building a crate in a workspace

For our project, we are using a workspace, which is a collection of related crates. When code repositories grow large, workspaces are a great way of organizing code to make reading and modifying it easier. If you want to execute the build, test, run, etc. for a particular crate you add `-p <crate_name>` after the command. For example: `cargo test -p memstore` would just compile and execute the tests in the memstore crate (you'll soon what that is in the context of CrustyDB). If memstore depends on other crates (external or in the workspace), it would build them first.

Read more about Cargo in the [Rust Book](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html).

## Debugging Rust Programs

Debugging is a crucial skill you should learn (if you don't know yet) in order
to become a more effective software developer. If you write software, your
software will contain bugs. Debugging is the process of finding those bugs,
which is necessary if you want to fix them.

### Debuggers

There are tools to help you debug software called debuggers. You may
have already heard about these. For example, in the C, C++ world, gdb and lldb
are two popular debuggers. gdb is used to debug programs that have been compiled
with gcc, while lldb is used to debug programs compiled with the LLVM toolkit.
What this means in practice, in 2020, is that if you work on a Linux platform,
you'll likely be using gdb. If you work on a Mac OS platform, you'll likely be
using lldb. If you work on a windows platform, then you may be using either one,
depending on your configuration.

### Debuggers in the IDE

A popular way of writing software is via IDEs (which we recommend you use to
develop crustyDB). IDEs for most languages come with a debugger preconfigured.
The situation for Rust is a little different. Only relatively recently IDE
developers have started incorporating debuggers for Rust, and the support is
still sparse. If you use Visual Studio Code (a lightweight and open source IDE),
you will be able to use a Rust debugger (based on gdb or lldb depending on the
underlying platform). You can easily find instructions online on how to set this
up.

Most other free IDEs do not have good support for the Rust debugger yet.

#### CLion

JetBrain's [CLion](https://www.jetbrains.com/clion/) IDE looks to have a solid Rust debugger with the Rust extension.
However CLion is not free, but it does offer academic licenses. [Apply here](https://www.jetbrains.com/community/education/#students)
if you want to access the tool (some restrictions on what you can use the tool for).
[Here are instructions on set up and using](https://blog.jetbrains.com/clion/2019/10/debugging-rust-code-in-clion/)
which worked for me out of the box on Ubuntu (with installing the Rust plugin).
One of our TAs uses CLion to debug Rust on OSX. The link also contains instructions for 
debugging on Windows, but it has not been tested by us.

#### VSCode

We have had some mixed success with using VSCode for debugging Rust
(although it is a great Rust IDE with the right extensions).  Using the
extensions Rust and CodeLLDB on Ubuntu has gotten debugging working on a set up.
We included the launch.json for running tests in a package.

### Alternative ways of debugging programs

You are already familiar with printing the values of variables in your programs
in order to understand program behavior and detect problems, i.e., in order to
debug your programs. Rust has its own println!() macro (and Crusty uses a
logging library). Rust also has a dbg!()
macro in its standard library, which will simply format the argument so its
printable along with the line where it's found.  A real debugger will give you
much more information, presented better, and in context, so it's a much more
powerful way of debugging programs, and the recommended way. However, in some
instances, the macros above may come in handy, especially as Rust's debuggers
support matures.

## Module 1 Exercises:

Clone this repository in a computer with the Rust environment installed and go to module_1_exercises to access the code necessary to complete this module. You will find the repo has a Cargo.toml file, which defines the project. In this case, the Cargo.toml file is very simple (take a look!) but once you transition to working on crusty it'll become a bit more complex and you'll have to be familiar with its syntax to be productive.

Place all the structs and functions we ask you to write in a file called ```solutions.rs``` within src so that our autograder can process your submission.

- Here are some exercises to help you practice the Rust programming concepts introduced in Module 1:
    1. Create a Hello World function (call it hello()) that returns "Hello World!"
    2. Create a function ```is_leap_year(year: i32) -> bool``` that takes an int and determines if that year is a leap year. 
- Remember, to test these exercises, run ``` cargo build ```
- You can also complete [the programming of a guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) in the Rust documentation.
