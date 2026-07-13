use std::io;

fn vai_pegar_o_fugitivo(valores:Vec<&str>){
    if valores.len() < 3{
        return;
    }

    let d:u32 = valores[0].parse().unwrap();
    let vf:u32 = valores[1].parse().unwrap();
    let vg:u32 = valores[2].parse().unwrap();

    let hipotenusa_ao_quadrado = d * d + 144;

    let tempo_guarda = (12 * vg) * (12 * vg);
    let tempo_fugitivo = hipotenusa_ao_quadrado * (vf * vf);

    if tempo_guarda >= tempo_fugitivo{
        println!("S");
    }else{
        println!("N");
    }
}

fn main(){
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).unwrap() > 0{
        vai_pegar_o_fugitivo(buffer.trim().split_whitespace().collect());
        buffer.clear();
    }
}