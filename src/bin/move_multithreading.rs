#![allow(unused)]
use std::{thread, time::Duration};

fn main() {
    let vec = vec![1, 2, 3];
    let handle = thread::spawn(move || println!("here is a vector : {:?}", vec));
    handle.join().unwrap();
}

// why should use move ?
// EG:

// fn use_case() {
//     let x = 1;
//     {
//         let vec = vec![1, 2, 3];
//         let handle = thread::spawn(|| {
//             println!("hello {:?}", vec);
//             thread::sleep(Duration::from_millis(12));
//         });
//     }
//     println!("{}", x);
// }

// since spawned thread is running inside the new scope but we dont know that when the println macro will run if it runs after completing the spawned thread work
// then it will be fine but if not the vec should go out of scope and deallocated from the memory so this will leads to dangling pointer

// what move does ?
// => move is a closure which more the ownership of the value to the new thread
