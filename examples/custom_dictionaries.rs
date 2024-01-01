// NOTE: Every `.unwrap()` seen here is SAFE.

use names::{GeneratorBuilder, Generator};
use rand::rngs::ThreadRng;

fn main() {
    let adjectives: Vec<String> = vec!["imaginary".into()];
    let nouns: Vec<String> = vec!["roll".into()];
    let mut generator: Generator<ThreadRng> = GeneratorBuilder::default()
        .adjectives(adjectives)
        .nouns(nouns)
        .build()
        .unwrap();

    assert_eq!("imaginary-roll", generator.next().unwrap());
}