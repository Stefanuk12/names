// NOTE: Every `.unwrap()` seen here is SAFE.

use names::{GeneratorBuilder, Name, Length, Casing, NumberSeperator, Generator};
use rand::rngs::ThreadRng;

fn main() {
    let mut generated: Generator<ThreadRng> = GeneratorBuilder::default()
        .casing(Casing::CamelCase)
        .naming(Name::ZeroPaddedNumbered(2, NumberSeperator::Underscore))
        .length(Length::Truncate(20))
        .build()
        .unwrap();

    println!("{:?}", generated.adjectives);

    println!("My new name is: {}", generated.next().unwrap());  
}