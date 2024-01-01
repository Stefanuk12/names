use names::{Generator, ThreadRng};

fn main() {
    let mut generator = Generator::<ThreadRng>::default();
    println!("Your project is: {}", generator.next().unwrap());
}