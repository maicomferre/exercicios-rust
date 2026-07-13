use std::io;

fn resolve(values:Vec<&str>) -> (){
    if values.len() < 2{
        return;
    }

    let v:i32 = values[0].parse().unwrap();
    let t:i32 = values[1].parse().unwrap();

    println!("{}", (v * t) * 2);
}

fn main(){
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).unwrap() > 0{
        resolve(buffer.trim().split_whitespace().collect());
        buffer.clear();
    }
}
