fn main() {
    let numbers = [1.1, 2.0, 3.3];
    println!("{:?}", numbers[1]);
    // slice
    println!("{:?}", &numbers[1..3])
}
