fn main() {
    let mut generated = names::GeneratorBuilder::default()
        .seperator(names::Seperator::Underscore)
        .naming(names::Name::ZeroPaddedNumbered(2))
        .length(names::Length::Truncate(10))
        .build()
        .unwrap(); // this can safely be unwrapped as the builder will always return a valid generator

    println!("My new name is: {}", generated.next().unwrap());  
}