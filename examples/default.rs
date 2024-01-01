// NOTE: Every `.unwrap()` seen here is SAFE.

use names::Generator;

fn main() {
    let mut generator = Generator::default();
    println!("Your project is: {}", generator.next().unwrap());
}