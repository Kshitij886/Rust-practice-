#![allow(unused)]
use std::collections::HashMap;

// hashmap methods
// insert => to insert the value to the hashmaps 
// get => to get the value 
// remove 
// clear

fn main() {
    let mut user: HashMap<String, i32> = HashMap::<String, i32>::new();

    user.insert(("Kshitij".to_string()), 19);
    user.insert(("Simran".to_string()), 22);

    let first_user = user.get("Kshitij");
    match first_user {
        Some(age) => {
            println!("Age: {:?}", user);
        },
        None =>  {
            println!("No value");
        }
        
    }
}