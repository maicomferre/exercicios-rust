use std::io;

fn main(){
    let mut buffer:String = String::new();

    let array_preco:[f32; 6] = [0.0,  4.00, 4.50, 5.00, 2.00, 1.50];

    io::stdin().read_line(&mut buffer).unwrap();

    let vec:Vec<&str> = buffer.split_whitespace().collect();

    let preco = array_preco[vec[0].parse::<u8>().unwrap() as usize];
    let quantidade:f32 = vec[1].parse().unwrap();

    println!("Total: R$ {:.2}", quantidade * preco);
}