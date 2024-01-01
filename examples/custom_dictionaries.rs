// NOTE: Every `.unwrap()` seen here is SAFE.

use names::GeneratorBuilder;

fn main() {
    let adjectives = vec!["imaginary"];
    let nouns = vec!["roll"];
    let mut generator = GeneratorBuilder::default()
        .adjectives(adjectives)
        .nouns(nouns)
        .build()
        .unwrap();

    assert_eq!("imaginary-roll", generator.next().unwrap());
}