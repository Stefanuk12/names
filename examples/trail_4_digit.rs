// NOTE: Every `.unwrap()` seen here is SAFE.

use names::{GeneratorBuilder, Name, NumberSeperator};

fn main() {
    let mut generator = GeneratorBuilder::default()
        .naming(Name::ZeroPaddedNumbered(4, NumberSeperator::Dash))
        .build()
        .unwrap();
    println!("Your project is: {}", generator.next().unwrap());
}