use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("Número secreto: {numero_secreto}");

    println!("Adivinhe o número!");
    println!("Por favor, coloque seu palpite.");

    let mut palpite = String::new();

    io::stdin()
        .read_line(&mut palpite)
        .expect("Failed to read line");

    let palpite: u32 = palpite.trim().parse().expect("Por favor insira um número!");
    
    println!("Seu palpite é {palpite}");

    match palpite.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito pequeno!"),
        Ordering::Greater => println!("Muito grande!"),
        Ordering::Equal => println!("Você venceu"),
    }
}
