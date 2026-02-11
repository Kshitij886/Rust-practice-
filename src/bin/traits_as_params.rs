#![allow(unused)]

use std::fmt::format;

pub trait Summary {
    fn sumarize(&self) -> String;
}
struct User {
    name: String,
    age: u32,
}
impl Summary for User {
    fn sumarize(&self) -> String {
        return format!("Name: {} and age is {}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: "KShtij khatri".to_string(),
        age: 32,
    };
    notify(&user);
}

fn notify(u: &impl Summary) {
    println!("{}", u.sumarize());
}
