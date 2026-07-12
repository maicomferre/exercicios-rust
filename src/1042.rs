use std::io;

fn main(){
    let mut buffer:String = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let mut numbers:Vec<i32> = buffer.trim().split_whitespace().map(|x:&str| x.parse::<i32>().unwrap()).collect();
    let backup = numbers.clone();
    numbers.sort();

    println!("{}\n{}\n{}\n",numbers[0],numbers[1],numbers[2]);

    println!("{}\n{}\n{}", backup[0], backup[1], backup[2]);
}

