//! This crate provides a generate that constructs random name strings suitable
//! for use in container instances, project names, application instances, etc.
//!
//! The name `Generator` implements the `Iterator` trait so it can be used with
//! adapters, consumers, and in loops.
//!
//! ## Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/names) and can be
//! used by adding `names` to your dependencies in your project's `Cargo.toml`
//! file:
//!
//! ```toml
//! [dependencies]
//! names = { version = "0.14", default-features = false }
//! ```
//! ## Examples
//!
//! ### Example: painless defaults
//!
//! The easiest way to get started is to use the default `Generator` to return
//! a name:
//!
//! ```
//! use names::Generator;
//!
//! let mut generator = Generator::default();
//! println!("Your project is: {}", generator.next().unwrap());
//! // #=> "Your project is: rusty-nail"
//! ```
//!
//! If more randomness is required, you can generate a name with a trailing
//! 4-digit number via the builder pattern:
//!
//! ```
//! use names::{GeneratorBuilder, Name};
//!
//! let mut generator = GeneratorBuilder::default()
//!     .naming(Name::Numbered(4))
//!     .build()
//!     .unwrap(); // this can safely be unwrapped as the builder will always return a valid generator
//! 
//! println!("Your project is: {}", generator.next().unwrap());
//! // #=> "Your project is: pushy-pencil-5602"
//! ```
//!
//! ### Example: with custom dictionaries
//!
//! If you would rather supply your own custom adjective and noun word lists,
//! you can provide your own by supplying 2 string slices. For example,
//! this returns only one result:
//!
//! ```
//! use names::{GeneratorBuilder, Name};
//!
//! let adjectives = &["imaginary"];
//! let nouns = &["roll"];
//! let mut generator = GeneratorBuilder::default()
//!     .adjectives(adjectives)
//!     .nouns(nouns)
//!     .build()
//!     .unwrap(); // this can safely be unwrapped as the builder will always return a valid generator
//!
//! assert_eq!("imaginary-roll", generator.next().unwrap());
//! ```

#![doc(html_root_url = "https://docs.rs/names/0.14.1-dev")]
#![deny(missing_docs)]

use derive_builder::Builder;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

/// List of English adjective words
pub const ADJECTIVES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));

/// List of English noun words
pub const NOUNS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/nouns.rs"));

/// A naming strategy for the `Generator`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Name {
    /// This represents a plain naming strategy of the form `"ADJECTIVE-NOUN"`
    Plain,
    /// This represents a naming strategy with a random number appended to the
    /// end, of the form `"ADJECTIVE-NOUN-NUMBER"`
    Numbered(usize),
    /// This represents a naming strategy with a zero-padded number appended to
    /// the end, of the form `"ADJECTIVE-NOUN-NUMBER"`
    ZeroPaddedNumbered(usize),
}

impl Default for Name {
    fn default() -> Self {
        Name::Plain
    }
}

/// A seperator for the `Generator`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Seperator {
    /// This represents a seperator of the form `"ADJECTIVE-NOUN"`
    Dash,
    /// This represents a seperator of the form `"ADJECTIVE_NOUN"`
    Underscore,
    /// A custom seperator
    Custom(&'static str),
    /// This represents no seperator of the form `"ADJECTIVENOUN"`
    None,
}

impl Default for Seperator {
    fn default() -> Self {
        Seperator::Dash
    }
}
impl std::fmt::Display for Seperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seperator::Dash => write!(f, "-"),
            Seperator::Underscore => write!(f, "_"),
            Seperator::Custom(s) => write!(f, "{}", s),
            Seperator::None => write!(f, ""),
        }
    }
}

/// A length for the `Generator`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Length {
    /// This forces the generator to truncate the generated name to the given length.
    Truncate(usize),
    /// This forces the generator to reroll the generated name until it is the given length.
    Reroll(usize),
    /// No length limit
    None,
}
impl Default for Length {
    fn default() -> Self {
        Length::None
    }
}

/// A random name generator which combines an adjective, a noun, and an
/// optional number
///
/// A [`Generator`] takes a slice of adjective and noun words strings and has
/// a naming strategy (with or without a number appended).
/// 
/// To generate a [`Generator`], use [`GeneratorBuilder`], view the [examples](crate#examples) for more information.
/// 
/// **NOTE**: You may safely unwrap the result of [`GeneratorBuilder::build`](crate::GeneratorBuilder::build) as the builder will always return a valid [`Generator`].
#[derive(Builder, Clone, Debug)]
pub struct Generator<'a> {
    /// A slice of adjective words
    #[builder(default = "ADJECTIVES")]
    adjectives: &'a [&'a str],
    /// A slice of noun words
    #[builder(default = "NOUNS")]
    nouns: &'a [&'a str],
    /// A naming strategy
    #[builder(default)]
    naming: Name,
    #[builder(default)]
    /// A seperator
    seperator: Seperator,
    /// The maximum length of the generated name
    #[builder(default)]
    length: Length,
    #[builder(default = "ThreadRng::default()")]
    /// The random number generator
    rng: ThreadRng,
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        GeneratorBuilder::default().build().unwrap()
    }
}

impl<'a> Iterator for Generator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let adj = self.adjectives.choose(&mut self.rng).unwrap();
        let noun = self.nouns.choose(&mut self.rng).unwrap();
        let seperator = &self.seperator;

        let mut generated = match self.naming {
            Name::Plain => format!("{}{seperator}{}", adj, noun),
            Name::Numbered(x) => format!("{}{seperator}{}{seperator}{}", adj, noun, generate_number_with_x_digits(x, &mut self.rng)),
            Name::ZeroPaddedNumbered(x) => format!("{}{seperator}{}{seperator}{}", adj, noun, generate_padded_number_with_x_digits(x, &mut self.rng)),
        };
        
        Some(match self.length {
            Length::Truncate(x) => { generated.truncate(x); generated },
            Length::Reroll(x) => {
                while generated.len() != x {
                    generated = self.next().unwrap();
                }
                generated
            },
            Length::None => generated,
        })
    }
}

fn generate_number_with_x_digits(x: usize, rng: &mut ThreadRng) -> usize {
    let lower_bound = 10usize.pow((x - 1) as u32);
    let upper_bound = 10usize.pow(x as u32) - 1;
    rng.gen_range(lower_bound..=upper_bound)
}

fn generate_padded_number_with_x_digits(x: usize, rng: &mut rand::rngs::ThreadRng) -> String {
    let number = generate_number_with_x_digits(x, rng);
    format!("{:0>width$}", number, width = x)
}