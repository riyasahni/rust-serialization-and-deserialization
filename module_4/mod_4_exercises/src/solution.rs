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
    //let c2 = c.clone().write().unwrap();
    let cloned2= Arc::clone(&c);
    for _i in [50..100] {
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
