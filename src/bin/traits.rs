#![allow(unused)]

// define a trait

use std::iter::Sum;

trait Summary {
    fn summarize(&self) -> String;
}
struct User {
    name: String,
    age: u32,
}
impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("kshiitj"),
        age: 21,
    };
    println!("{}", user.summarize());
}

// a trait defines a functionality a particular type has and can share with other types. We can use traits to define shared behaviour in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// default trait implementation

pub trait Summarize {
    fn sumarize(&self) -> String {
        return String::from("Sumarize");
    }
}

struct Users {
    name: String,
    age: u32,
}

impl Summarize for Users {}
