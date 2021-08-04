use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line)
	.expect("Faied to read this");

    let iter = line.split_whitespace();
    let vec: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();

    let ans = vec[0] - vec[1];
    
    println!("answer : {}", ans);
}
