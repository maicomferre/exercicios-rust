use std::io;
fn main(){
    let mut buffer:String = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let v:f32 = buffer.trim().parse().unwrap();

    if v >= 0.0 && v <= 25.0{
        println!("Intervalo [0,25]");
    } else if v > 25.0 && v <= 50.0{
        println!("Intervalo (25,50]");
    } else if v > 50.0 && v <= 75.0  {
        println!("Intervalo (50,75]");
    } else if v > 75.0 && v <= 100.0 {
        println!("Intervalo (75,100]");
    } else{
        println!("Fora de intervalo");
    }
}