use std::io;
use std::io::Read;

fn main(){
    let mut buffer:String = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let st:Vec<&str> = buffer.split_whitespace().collect();

    let a:i32 = st[0].parse().unwrap();
    let b:i32 = st[1].parse().unwrap();

    println!("{}", (a - b));
}