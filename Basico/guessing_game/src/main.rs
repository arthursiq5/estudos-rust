use std::io;

fn main() {
    println!("Adivinhe o número!");
    println!("Por favor, coloque seu palpite.");

    let mut palpite = String::new();

    io::stdin()
        .read_line(&mut palpite)
        .expect("Failed to read line");
    
    println!("Seu palpite é {palpite}");
}
