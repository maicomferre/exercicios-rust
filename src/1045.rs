use std::io;

fn main(){
	let mut buffer:String = String::new();

	io::stdin().read_line(&mut buffer).unwrap();

	let mut val: Vec<f64> = buffer
        	.trim()
        	.split_whitespace()
        	.map(|s| s.parse().unwrap())
        	.collect();
	val.sort_by(|a, b| b.partial_cmp(a).unwrap());


	if val[0] >= val[1]+val[2]{
		println!("NAO FORMA TRIANGULO");
		return;
	}

	let aoquadrado = val[0] *val[0];
	let bmaisc = val[1] * val[1] + val[2]*val[2];

	if aoquadrado == bmaisc{
		println!("TRIANGULO RETANGULO");
	}
	if aoquadrado > bmaisc{
		println!("TRIANGULO OBTUSANGULO");
	}
	if aoquadrado < bmaisc{
		println!("TRIANGULO ACUTANGULO");
	}

        if val[0] == val[1] && val[1] == val[2]{
                println!("TRIANGULO EQUILATERO");
        }else if val[0] == val[1] || val[1] == val[2] || val[0] == val[2]{
                println!("TRIANGULO ISOSCELES");
	}

}
