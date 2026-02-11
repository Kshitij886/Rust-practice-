#![allow(unused)]
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

fn main() {
    with_awating();
    thread::spawn(|| {
        for i in 1..10 {
            println!("thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

fn with_awating() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spwned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..10 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
}
