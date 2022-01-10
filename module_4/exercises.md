## Setup instructions
Complete the following exercises and put all code in ```solution.rs```. Function and struct declarations are already provided to you, so put the code needed to complete the exercises there. To test, can either write your own tests or use ```main.rs```, but neither your tests or anything you write in ```main.rs``` will be used for grading: only what's in ```solution.rs```.

1. Define a trait called *Geometry* that has two methods: *get_area* and *get_name*. Then implement that trait on two types - *Rectangle* and *Circle*. *get_area* should return a float (use f64). *get_name* should return the name of the shape as a String. You can import PI via *use std::f64::consts::PI*. 

2. In this exercise, you will need to implement a two-thread counter that counts from 0 to 100. Below is the skeleton of the counter program. Read the comments and complete as needed.

```rust

use std::thread;

struct Counter {
    count: i32
}

// define a function incr() to increment count in Counter by 1
fn incr() {}

fn main() {
    // declare a counter

    // spawn a thread here to call incr() 50 times
    let handle = thread::spawn(move|| {
        for _i in [??] {
            println!("thread spawned count {}", [??]);
        }
    });
   
    // in the main thread, call incr() 50 times
    for _i in [??] {
        println!("thread main count {}", [??]);
    }
    handle.join().unwrap();
}
```

3. Modify the following code block to use RwLocks. Run both blocks, one with standard mutexes and the other with RwLocks. What differences do you observe in their behavior/output? Does this match your understanding of how Read/Write locks work?

```rust
use std::sync::{Arc, RwLock, Mutex};
use std::thread;

fn main() {
    let lock = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(10);

    for _i in 0..10 {
        let reader_lock = lock.clone();
        let reader = thread::spawn(move || {
            for _j in 0..20 {
                let r = reader_lock.lock().unwrap();
                println!("Read value as {}", *r);
            }
        });
        handles.push(reader)
    }

    for _j in 0..20 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("Incremented value by 1 to {}", *val);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```
