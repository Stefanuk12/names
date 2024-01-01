use names::{GeneratorBuilder, Name, NumberSeperator, ThreadRng};

fn main() -> Result<(), names::Error> {
    let mut generator = GeneratorBuilder::default()
        .naming(Name::ZeroPaddedNumbered(4, NumberSeperator::Dash))
        .rng(ThreadRng::default())
        .build()?;
    println!("Your project is: {}", generator.next().unwrap());
    Ok(())
}