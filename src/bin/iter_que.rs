#![allow(unused)]

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let vec_iter = vec.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);
    let mut new_vec = Vec::<u32>::new();
    for i in vec_iter {
        new_vec.push(i);
    }
    println!("{:?}", new_vec);
}

// write a logic to first filter all odd values then double each values and create a new vector
