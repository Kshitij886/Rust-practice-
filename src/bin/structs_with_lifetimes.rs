#![allow(unused)]

// untill now we haven't used refrences in structs

use std::fmt::Display;

// just add a generic lifetime annotation
struct User<'a> {
    name: &'a str,
}
fn main() {
    let first_name = String::from("KShitij khatri");
    let user1 = User { name: &first_name };
    println!("{}", user1.name);
    let ans = longest_with_an_announcement("hello", "khana", String::from("kshitij"));
    println!("{ans}");
}

// fn some_name() {}

fn longest_with_an_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement : {ann}");
    if a.len() > b.len() { a } else { b }
}
