#![allow(unused)]

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(3);
    vec.push(4);
    println!("{:?}", vec);
    match even(&vec) {
        Ok(v) => {
            println!("{:?}", v);
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}

fn even(nums: &Vec<i32>) -> Result<Vec<i32>, &'static str> {
    let mut evens = Vec::<i32>::new();
    for i in nums {
        if i % 2 == 0 {
            evens.push(*i);
            println!("{}", i);
        } else {
            continue;
        }
    }
    return Ok(evens);
}
