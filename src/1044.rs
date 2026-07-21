use std::io;

fn main(){
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer).unwrap();

	let val:Vec<&str> = buffer.trim().split_whitespace().collect();

	let a:i32 = val[0].parse().unwrap();
	let b:i32 = val[1].parse().unwrap();

	if a % b == 0 || b % a == 0{ 
		println!("Sao Multiplos");
	} else{
		println!("Nao sao Multiplos");
	}
}
