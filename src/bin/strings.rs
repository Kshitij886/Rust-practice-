#![allow(unused)]

fn main() {
    // removing the string
    let mut name = String::from("Kshitij");
    println!("{}", name);
    name.push_str(" khatri");
    println!("{}", name);
    name.replace_range(8..name.len(), "");
    println!("{}", name);
}

// output
// Compiling rust-projects v0.1.0 (C:\Users\Kshitij\rust-projects)
// Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
// Running `target\debug\strings.exe`
// Kshitij
// Kshitij khatri
// Kshitij

// Strings are which provided by rust's standered library rather than coded into the core language,
// its is grawable, mutable, owned and utf-8 string type.
//
