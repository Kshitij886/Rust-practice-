#![allow(unused)]

use std::collections::{HashMap, hash_map::Values};

fn main() {
    let mut vec = Vec::<(String, u32)>::new();
    vec.push(("kshitij".to_string(), 20));
    vec.push(("Simran".to_string(), 19));
    vec.push(("Rajendra".to_string(), 22));
    vec.push(("Daya".to_string(), 18));
    let mut hash_map = HashMap::<String, u32>::new();
    for (key, value) in vec.into_iter() {
        hash_map.insert(key, value);
    }
    for (k, v) in hash_map.iter_mut() {
        match k == "kshitij" {
            true => {
                *v = 34;
            }
            false => {
                println!("false");
            }
        }
    }
    println!("New vec : {:?}", hash_map);
}
