use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let str1 :&str = &line;
    let str2 :&str = str1.split(' ').collect::<&str>();
    let str3 :Vec<u8> = str2.as_bytes().to_vec();

    println!("Hello, world! {} , {}", str3[0], str3[1]);
}
