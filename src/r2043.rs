use std::io;

fn main(){
    let mut buffer:String = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let t:u16 = buffer.trim().parse().unwrap();

    println!("{}", t * 4);
}