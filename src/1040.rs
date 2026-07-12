use std::io;

fn main(){
    let mut buffer:String = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let valores:Vec<&str> = buffer.split_whitespace().collect();

    let notas:Vec<f32> = valores.iter().map(|&x| x.parse().unwrap()).collect();

    let media:f32 = ((notas[0] * 2.0) + (notas[1] * 3.0) + (notas[2] * 4.0) + (notas[3] * 1.0)) / 10.0;

    println!("Media: {:.1}", media);
    if media >= 7.0{
        println!("Aluno aprovado.");
        return;
    }

    if media < 5.0{
        println!("Aluno reprovado.");
        return;
    }

    println!("Aluno em exame.");

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();

    let prova_final:f32 = buffer.trim().parse().unwrap();

    println!("Nota do exame: {:.1}",prova_final);

    let media_final = (prova_final + media) / 2.0;

    if media_final < 5.0{
        println!("Aluno reprovado.");
    }else{
        println!("Aluno aprovado.");
    }

    println!("Media final: {:.1}",media_final);
}