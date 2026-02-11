#![allow(unused)]

fn main() {
    let ans;
    let a = 43;
    {
        let b = 32;
        ans = longer(&a, &b);
        println!("{} ", ans);
    }
    // println!("{} ", ans); // error : `b` does not live long enough
    // borrowed value does not live long enough
}

// question
// write a function that takes two strings as an input and returns the bigger amongst them

// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() { a } else { b }
// }

// this got a problem
// we are passing the refrence of the string to the function and the compiler doesnot know how much two string are going to be valib so that we have to specify the lifetime
// means the variable ans should be valid for how much lines, lets say b is the largest then ans will get the refrence of b but b value deallocated after the owner ended
// so if we access the ans again because ans has not ended yet, it leads to dangiling pointer. so to solve this we have to specify the lifetime of ans variabloe

// solution
// introduction of generic lifetimes

fn longer<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b { a } else { b }
}

// it doesnot means that the lifetime of all variable is same but the intersection of those two is the lifetime of return value
