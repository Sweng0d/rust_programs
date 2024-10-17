use std::io;

fn main() { 
    println!("Type your name");
    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Falha ao ler linha");

    println!("Your name is {}", name);
}
