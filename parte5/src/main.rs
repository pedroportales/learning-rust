use std::io;

fn main() {

    // let nome: &str = "Bruno"; // str slice, string reference

    // Heap
    // Heap String
    // String Dinâmica
    // String
    // let mut s = String::new();

    // let s: String = "Bruno".into();

    // println!("{s}");

    let mut s: String = String::new();
    
    println!("Digite um texto: ");
    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    
    println!("Você digitou {s}");
    println!("Quantidade de letras: {}", s.trim().len());
    
    soma(5, 2);
}

fn soma(a: u32, b: u32) {
    println!("{}", a + b);
}
