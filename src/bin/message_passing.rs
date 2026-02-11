// it leads transfer data from one to other.
// or communicate with it
// introducing Channels
// we use std::sync::mpsc => mpsc full form multiple producer sigle consumer

#![allow(unused)]

use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

fn main() {
    // creating a channel
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Kshitij");
        tx.send(val);
    });
    let reciver = rx.recv();
    match reciver {
        Ok(res) => {
            println!("hello {}", res);
        }
        Err(e) => {
            println!("Error {e}");
        }
    }
}
