//use core::num;
use std::f64::consts::PI;
//use std::net::Ipv6MulticastScope;
//use std::fmt::rt::v1::Count;
//use std::fmt::write;
use std::thread;
use std::sync::{Arc, RwLock, Mutex};

pub trait Geometry {
    fn get_area(&self) -> f64; // return area of shape
    fn get_name(&self) -> String; // return name of shape
}

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Geometry for Rectangle {
    fn get_area(&self) -> f64 {
        self.length*self.width
    }
    fn get_name(&self) -> String {
        let name = "Rectangle";
        name.to_string()
    }
}


pub struct Circle {
    pub radius: f64,
}

impl Geometry for Circle {
    fn get_area(&self) -> f64 {
        PI*self.radius*self.radius
    }
    fn get_name(&self) -> String {
        let name = "Circle";
        name.to_string()
    }
}

struct Counter {
    count: i32
}

// define a function incr() to increment count in Counter by 1

trait Increment {
fn incr (&mut self);
}

impl Increment for Counter {

    fn incr(&mut self) {
        self.count += 1;
    }
}

pub fn counter() {
    // declare a counter
    let c = Arc::new(Mutex::new(Counter { count: 0 }));
    // spawn a thread here to call incr() 50 times
    let cloned1 = Arc::clone(&c);
    let handle = thread::spawn(move|| {
        for _i in [0..50] {
            let mut num = cloned1.lock().unwrap();
            num.incr();
            println!("thread spawned count {:#?}", _i);
        }
    });

    // in the main thread, call incr() 50 times
    let cloned2= Arc::clone(&c);
    for _i in [0..50] {
        let mut num2 = cloned2.lock().unwrap();
        num2.incr();
        println!("thread main count {:#?}", _i);
    }
    handle.join().unwrap();
}


/*
 *  Modify the following function to use RwLocks. Run both blocks, one with standard mutexes
 *  and the other with RwLocks. What differences do you observe in their behavior/output? Does
 *  this match your understanding of how Read/Write locks work?
 */
pub fn read_write() {
   // let lock = Arc::new(Mutex::new(0));
   let lock = Arc::new(RwLock::new(0));
    let mut handles = Vec::with_capacity(10);

    for _i in 0..10 {
        let reader_lock = lock.clone();
        let reader = thread::spawn(move || {
            for _j in 0..20 {
                let r = reader_lock.read().unwrap();
                println!("Read value as {}", *r);
            }
        });
        handles.push(reader)
    }

    for _j in 0..20 {
        let mut val = lock.write().unwrap();
        *val += 1;
        println!("Incremented value by 1 to {}", *val);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
//With the Mutex lock, the reading happens after the writing. First, I notice that the value is
//incremented from 1 to .. 20, and then "Read value as 20" happens. But after I switch to RwLocks,
//the reading happens before the writing. First, "Read value as 0" happens, and then the value is
//incremented from 1 to .. 20. This makes sense, since from my understanding of Read/Write locks,
//any number of readers can acquire the lock so long as there is no writer. But the Mutex lock 
//doesn't distinguish between readers and writers, so it blocks threads that wait for the block to
//become available. This is why I think the value is incremented before it is read when we use Mutex.
