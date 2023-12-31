use names::{GeneratorBuilder, Name, Length, Casing, NumberSeperator};

fn main() {
    let mut generated = GeneratorBuilder::default()
        .casing(Casing::CamelCase)
        .naming(Name::ZeroPaddedNumbered(2, NumberSeperator::Underscore))
        .length(Length::Truncate(20))
        .build()
        .unwrap(); // this can safely be unwrapped as the builder will always return a valid generator

    println!("My new name is: {}", generated.next().unwrap());  
}