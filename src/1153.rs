use std::io;

fn main() {
    let mut buffer = String::new();
    
    io::stdin().read_line(&mut buffer).unwrap();
    
    let mut n:i32 = buffer.trim().parse().unwrap();
    let mut result = 1;
    
    while n > 0{
        result *= n;
        n -=1;
    }
    println!("{}",result);    
}
