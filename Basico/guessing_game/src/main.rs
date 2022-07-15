use rand::Rng;
use std::io;

fn main() {
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("Número secreto: {numero_secreto}");

    println!("Adivinhe o número!");
    println!("Por favor, coloque seu palpite.");

    let mut palpite = String::new();

    io::stdin()
        .read_line(&mut palpite)
        .expect("Failed to read line");
    
    println!("Seu palpite é {palpite}");
}
