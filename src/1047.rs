use std::io;


fn main(){
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer).unwrap();

	let tempos:Vec<u16> = buffer.trim().split_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
	

	let inicio:u16 = tempos[0] * 60 + tempos[1];
	let mut fim:u16 = tempos[2] * 60 + tempos[3];

	if fim <= inicio{
		fim += 24 * 60;
	}

	let fim = fim - inicio;

	
	println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", fim / 60, fim % 60);
}
