fn main() {
    let a = 1..4;

    for number in a.rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
