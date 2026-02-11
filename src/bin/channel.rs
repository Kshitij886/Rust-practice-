// can you write the code that finds the sum from 1 to 10^8?
#![allow(unused)]

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<i64>();
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans = 0;
            for j in i * 10000000..((i + 1) * 10000000) {
                ans = ans + j;
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);
    let mut ans = 0;
    for val in rx {
        ans = val + ans;
    }
    println!("ans is {}", ans);
}
