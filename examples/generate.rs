use names::{GeneratorBuilder, Name, Length, Casing, NumberSeperator, ThreadRng};

fn main() -> Result<(), names::Error>  {
    let mut generated = GeneratorBuilder::default()
        .casing(Casing::CamelCase)
        .naming(Name::ZeroPaddedNumbered(2, NumberSeperator::Underscore))
        .length(Length::Truncate(20))
        .rng(ThreadRng::default())
        .build()?;

    println!("My new name is: {}", generated.next().unwrap());
    Ok(())
}