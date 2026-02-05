#![allow(unused)]
fn main() {
    let a = String::from("khatri");
    match find_index_a(&a) {
        Some(a) => {
            println!("Index of string is {}", a);
        },
        None => {
            println!("This string do not contain letter a ");
        }
    }
}

fn find_index_a(name: &String) -> Option<u32> {
    let mut index = 0;
    for c in name.chars() {
        index = index + 1;
        if c == 'a' {
            return Some(index);
        }
    }
    return None
}