#![allow(unused)]
fn main() {
    let mut  s1  = String::from("hwllo");
    let s2 = s1;
    // println!("{}", s1); 
    // Since the variable can only have one owner but here first 
    // s1 is the owner but when we did s2 = s1 the owner moved from s1 to s2 so the value "hello" 
    // has one owner which is s2
    // why this ? 
    // because it can leads to dangling pointer.
    // since rust remove the value when its owner goes out of scope so when s2 goes out of scope the 
    // value get removed from heap but s1 is still pointing to that location but there is no data ie: dangling pointer 
    s1 = String::from("world");
    println!("{}", s1);
    // now this is right because the value "world" has only one owner ie : s1
}