// WAF that takes a vector of tuples (each tuple containing key and a value)
// and returns a hashmap where the keys are the unique keys from the input tuples
// and the values are vectors of all corresponding values associated with each key
#![allow(unused)]
use std::{collections::HashMap, hash::Hash};
fn main() {
    let vec = vec![("Simran".to_string(), 13),("kshitij".to_string(), 12), ("Simran".to_string(), 13)];
    println!("Vec : {:?}",vec);
    let new_hashmap = make_hashmaps(vec);
    println!("new Hashmap: {:?}", new_hashmap);
}

fn make_hashmaps(vec : Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut users = HashMap::<String, i32>::new();
    for (key, value) in vec {
        users.insert(key, value);
    }
    return users;
}