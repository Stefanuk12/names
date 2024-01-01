use names::{GeneratorBuilder, ThreadRng};

fn main() -> Result<(), names::Error>  {
    let adjectives: Vec<String> = vec!["imaginary".into()];
    let nouns: Vec<String> = vec!["roll".into()];
    let mut generator = GeneratorBuilder::default()
        .adjectives(adjectives)
        .nouns(nouns)
        .rng(ThreadRng::default())
        .build()?;

    assert_eq!("imaginary-roll", generator.next().unwrap());
    Ok(())
}