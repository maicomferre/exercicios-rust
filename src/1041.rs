use std::io;

fn main(){
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let vec:Vec<&str> = buffer.trim().split_whitespace().collect();

    let x:f32 = vec[0].parse().unwrap();
    let y:f32 = vec[1].parse().unwrap();

    if x > 0.0 && y > 0.0{
        println!("Q1");
    }else if x > 0.0 && y < 0.0{
        println!("Q4");
    }else if x < 0.0 && y > 0.0{
        println!("Q2");
    }else if x < 0.0 && y < 0.0{
        println!("Q3");
    }else if x == 0.0 && y == 0.0{
        println!("Origem");
    }else if x == 0.0{
        println!("Eixo Y");
    }else{
        println!("Eixo X");
    }
}