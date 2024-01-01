// NOTE: Every `.unwrap()` seen here is SAFE.

use names::{GeneratorBuilder, Name, Length, Casing, NumberSeperator};

fn main() {
    let mut generated = GeneratorBuilder::default()
        .casing(Casing::CamelCase)
        .naming(Name::ZeroPaddedNumbered(2, NumberSeperator::Underscore))
        .length(Length::Truncate(20))
        .build()
        .unwrap();

    println!("My new name is: {}", generated.next().unwrap());  
}