mod fibonacci_number;
use fibonacci_number::fibonacci_number;
use std::io;
fn main() {
    println!("Enter the required fibonacci: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("expecting a input");
    let n: u32 = n.trim().parse().expect("not a number");
    println!("nth fibonacci number is {}", fibonacci_number(n));
}
