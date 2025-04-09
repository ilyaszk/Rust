fn main() {
    let x: i32 = 5;

    match x {
        2..=10 => println!("x is between 5 and 10"),
        _ => println!("x is something else"),
    }
}
