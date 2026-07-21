use std::io;


fn main(){
	let mut buffer:String = String::new();

	io::stdin().read_line(&mut buffer).unwrap();

	let hora:Vec<u8> = buffer.trim().split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
	
	let tempo:u8
	= if hora[0] == hora[1]{
		24
	}else if hora[0] >= hora[1]{
		 24 - hora[0] + hora[1]
	}else{
		 hora[1] - hora[0]
	};

	println!("O JOGO DUROU {} HORA(S)", tempo);
}
