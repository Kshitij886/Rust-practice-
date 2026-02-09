#![allow(unused)]

use std::collections::btree_map::Values;

fn main() {
    let v1 = vec![1, 2, 3];
    // the iterators are lazy, meaning they have no effect untill you call methods that consume the iterator to use it up.
    // for eg : in below it just defiles a new variable
    let v2 = v1.iter();
    // the iterator is stored in the v2 variable.
    mutate_var();
    iter1();
}

// writing a code like

// let v1=  vec![1,2,3];
// for i in v1 {
// }

// its same as
// let v1 =  vec![1,2,3];
// let v1_iter = v1.iter();
// for i in v1_iter {}

// when we use first way, underthe hood rust is using iter() to iterate

// and iter() method in rust provides a way to iterate over the elements of a collection by borrowing them.

// and we cannot mutate the variables sinxe we have an immutable refrence to the internal elements.

// to mutate the value we have to

fn mutate_var() {
    let mut v1 = vec![1, 2, 3];
    println!("vec before mut: {:?}", v1);
    let v1_iter = v1.iter_mut();
    for i in v1_iter {
        *i = *i + 1;
    }
    println!("vec before mut: {:?}", v1);
}
// another way to iterate

fn iter1() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    while let Some(val) = v1_iter.next() {
        println!("value : {}", val)
    }
}

fn iter2() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();

    // this gives us the mutable ref of the elements of collection
    let first_num = v1_iter.next();    
    let second_num = v1_iter.next();
    let third_num = v1_iter.next();
    println!("{:?}", v1);
}

//iter function 

// two types 
//  consuming adapters 
// -> methods that call next() are consuming adapters because call them uses up the iterator.
// iterator adapters 
// -> methods defined on the iterator trailt that dont consume the iterator. Instead they produce different iterators by changing some aspect of the original iterator. EG: map, filter

fn is_even() {
    let mut vec1 = vec![1,2,3,4,5,6,7];
    let vec_iter = vec1.iter();
    let vec_iter2 = vec_iter.filter(|x| *x%2==0);
    println!("{:?}", vec_iter2);
}