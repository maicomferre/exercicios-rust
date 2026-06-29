use std::io;

fn main(){
    let mut buffer:String = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let cha_certo:char = buffer.trim().chars().next().unwrap();
    buffer.clear();

    io::stdin().read_line(&mut buffer).unwrap();

    let mut total:i16 = 0;

    for x in buffer.split_whitespace(){
        if x.chars().next().unwrap() == cha_certo{
            total += 1;
        }
    }

    println!("{}", total);

}