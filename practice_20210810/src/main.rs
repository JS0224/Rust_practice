#![allow(unused)]
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::iter;

fn read_file(file_name: String) -> String {
    println!("current path : {:?}", env::current_dir().unwrap());

    let mut f = File::open(file_name).expect("Failed to open");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    return buffer
}

fn make_pyramids(letter: String, num: u32) {
    for i in (1..num+1).rev() {
        let mut line = std::iter::repeat("x").take(i as usize).collect::<String>();
        println!("{}", line);
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = line.split_at(line.len()-1).0.to_string();
    println!("File name from User : <{}>", line);

    let letter = read_file(line);
    println!("String in the file :{}", letter);

    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let n: u32 = num.split_at(num.len() - 1).0.to_string().parse().unwrap();
    println!("Number you entered : <{}>", n);

    make_pyramids(letter, n);
}
