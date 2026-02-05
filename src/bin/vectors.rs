#![allow(unused)]

fn main() {
    let mut vec = vec![1,2,3,4,5,6];
    println!("{:?}", vec);
    match even_values(&vec) {
        Ok(v) => {
            println!("{:?}", v);
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}

fn even_values(nums: &Vec<i32>) -> Result<Vec<i32>, &'static str> {
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
