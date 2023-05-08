#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;

fn main() {
    // println!("Hey what is your fucking name????");
    // // This var can be change
    // let mut name = String::new();
    // let greeting = "Yo YO";
    // io::stdin().read_line(&mut name)
    //     .expect("Bro go to hell");
    // println!("Hello {}! {}", name.trim_end(), greeting);

    // =========== Variables ============= //
    // // constants name must be a capital litters
    // const PI: f32 = 3.14;
    // const ONE_MIL: u32 = 1_000_000;
    // let age = "18";
    // let mut age: u32 = age.trim().parse()
    //     .expect("age must be number");
    // age+=1;
    // println!("Your new age is {}, i want ${}", age, ONE_MIL)
    
    // =========== Random package ============ //
    // let random_num = rand::thread_rng().gen_range(1..100);
    // println!("rand in {}", random_num);

    // ============== Match =================== //
    // let my_age = 17;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("can't vote child"),
    //     Ordering::Greater => println!("can vote"),
    //     Ordering::Equal => println!("can vote too"),
    // };

    // ============== arrays =================== //
    // // Must be same type
    // let arr = [1,2,3,4];
    // println!("arr first element: {}", arr[0]);
    // println!("arr length: {}", arr.len());

    // ============== loop =================== //
    // let arr1 = [1,2,4,5,6,7,8,9,10];
    // let mut loop_idx = 0;
    // Get odd elements from array using loop
    // loop {
    //     if loop_idx == arr1.len() - 1 {
    //         break;
    //     }
    //     if arr1[loop_idx] % 2 != 0 {
    //         println!("odd element: {}", arr1[loop_idx]);
    //     }
    //     loop_idx += 1;
    // }
    // // Get even elements from array using while loop
    // while loop_idx < arr1.len() {
    //     if arr1[loop_idx] % 2 == 0 {
    //         println!("even element: {}", arr1[loop_idx]);
    //     }
    //     loop_idx += 1;
    // }
    // // Get all elements from array using for loop
    // for val in arr1.iter() {
    //     println!("val: {}", val);
    // }

    // ============== Tuples =================== //
    // let my_tuple: (u32, String, bool) = (18, "Hello".to_string(), true);
    // println!("first: {}", my_tuple.0);
    // println!("sec: {}", my_tuple.1);
    // println!("three: {}", my_tuple.2);

    // let (v1, v2, v3) = my_tuple;
    // println!("v1: {}", v1);
    // println!("v2: {}", v2);
    // println!("v3: {}", v3);


    // ============== Strings =================== //
    // // String -> can be change, vector of chars
    // // &str -> can't be change, slice of string
    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str("BCD word");
    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }
    // let st2 = st1.replace("A", "a");
    // println!("st2: {}", st2);
    // let st3 = String::from("Hello world");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for c in v1 {
    //     println!("{}", c);
    // }
    // let st4: &str = "Random string";
    // let mut st5: String = st4.to_string();
    // println!("st5: {}", st5);
    // let bytes_arr1 = st5.as_bytes();
    // let st6 = &st5[1..5];
    // println!("String length: {}", st6.len());
    // st5.clear();
    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7;
    // for char in st8.bytes(){
    //     println!("{}", char);
    // }

    // ============== Casting =================== //
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    println!("test");
}
