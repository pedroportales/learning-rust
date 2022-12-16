fn main() {
    let total = 30;
    { // inicio
        let total = "quarenta";
        println!(" Trabalhou {} horas", total);
    } // fim
    println!(" Trabalhou {} horas", total);
}
