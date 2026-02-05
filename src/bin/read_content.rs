// Result enum

use std::{ fs};
use std::io::Error;
fn main() {
    let path = String::from("practice.txt");
    match extract_content(&path) {
        Ok(ans) => {
            println!("{}", ans);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn extract_content(path : &String) -> Result<String, Error> {
    match fs::read_to_string(path) {
        Ok(content) => {
            return Ok(content);
        },
        Err(e) => {
            println!("Error {}", e);
            return Err(e);
        }
    }
}