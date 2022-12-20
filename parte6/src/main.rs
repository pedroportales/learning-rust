use std::io;

fn main() {

    println!("{:-^40}", "Calculadora");
    let mut s: String = String::new();
    
    let banner = 
        "Digite uma sequência de números\n\
        separados  por vírgula\n\
        exemplo: 1,2,3,45";

    println!("{}", banner);

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    
    println!("Você digitou {s}");
    // println!("Quantidade de letras: {}", s.trim().chars());
    
    println!("{}", "-".repeat(40));

}
