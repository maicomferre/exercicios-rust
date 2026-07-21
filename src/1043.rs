use std::io;


fn main(){
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer).unwrap();

	let values:Vec<&str> = buffer.trim().split_whitespace().collect();

	let a:f32 = values[0].parse().unwrap();
	let b:f32 = values[1].parse().unwrap();
	let c:f32 = values[2].parse().unwrap();

	if a < b + c && b < a + c && c < a + b {
		println!("Perimetro = {:.1}", a+b+c);
	} else {
		println!("Area = {:.1}", ((a+b)*c)/2.0);
	}
}
