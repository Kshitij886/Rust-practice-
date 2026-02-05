#![allow(unused)]
use chrono::{Utc, Local};


fn main() {
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);
    let formated = now.format("%Y-%h-%d %H:%M:%S");
    println!("Formatted date and time: {}", formated);

    let local = Local::now();
    println!("Current date and time in local is : {}", local);

} 