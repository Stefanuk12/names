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
//! names = { version = "0.14.0", default-features = false }
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
//! 4-digit number:
//!
//! ```
//! use names::{Generator, Name};
//!
//! let mut generator = Generator::with_naming(Name::Numbered);
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
//! use names::{Generator, Name};
//!
//! let adjectives = &["imaginary"];
//! let nouns = &["roll"];
//! let mut generator = Generator::new(adjectives, nouns, Name::default());
//!
//! assert_eq!("imaginary-roll", generator.next().unwrap());
//! ```

#![doc(html_root_url = "https://docs.rs/names/0.14.1-dev")]
#![deny(missing_docs)]

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

/// List of English adjective words
pub const ADJECTIVES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));

/// List of English noun words
pub const NOUNS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/nouns.rs"));

/// A naming strategy for the `Generator`
pub enum Name {
    /// This represents a plain naming strategy of the form `"ADJECTIVE-NOUN"`
    Plain,
    /// This represents a naming strategy with a random number appended to the
    /// end, of the form `"ADJECTIVE-NOUN-NUMBER"`
    Numbered,
}

impl Default for Name {
    fn default() -> Self {
        Name::Plain
    }
}

/// A seperator for the `Generator`
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

/// A random name generator which combines an adjective, a noun, and an
/// optional number
///
/// A `Generator` takes a slice of adjective and noun words strings and has
/// a naming strategy (with or without a number appended).
pub struct Generator<'a> {
    adjectives: &'a [&'a str],
    nouns: &'a [&'a str],
    naming: Name,
    seperator: Seperator,
    rng: ThreadRng,
}

impl<'a> Generator<'a> {
    /// Constructs a new `Generator<'a>`
    ///
    /// # Examples
    ///
    /// ```
    /// use names::{Generator, Name};
    ///
    /// let adjectives = &["sassy"];
    /// let nouns = &["clocks"];
    /// let naming = Name::Plain;
    /// let seperator = Seperator::Dash;
    ///
    /// let mut generator = Generator::new(adjectives, nouns, naming, seperator);
    ///
    /// assert_eq!("sassy-clocks", generator.next().unwrap());
    /// ```
    pub fn new(adjectives: &'a [&'a str], nouns: &'a [&'a str], naming: Name, seperator: Seperator) -> Self {
        Generator {
            adjectives,
            nouns,
            naming,
            seperator,
            rng: ThreadRng::default(),
        }
    }

    /// Construct and returns a default `Generator<'a>` containing a large
    /// collection of adjectives and nouns
    ///
    /// ```
    /// use names::{Generator, Name, Seperator};
    ///
    /// let mut generator = Generator::custom(Name::Plain, Seperator::Dash);
    ///
    /// println!("My new name is: {}", generator.next().unwrap());
    /// ```
    pub fn custom(naming: Name, seperator: Seperator) -> Self {
        Generator::new(ADJECTIVES, NOUNS, naming, seperator)
    }
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Generator::new(ADJECTIVES, NOUNS, Name::default(), Seperator::Dash)
    }
}

impl<'a> Iterator for Generator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let adj = self.adjectives.choose(&mut self.rng).unwrap();
        let noun = self.nouns.choose(&mut self.rng).unwrap();
        let seperator = &self.seperator;

        Some(match self.naming {
            Name::Plain => format!("{}{seperator}{}", adj, noun),
            Name::Numbered => format!("{}{seperator}{}{seperator}{:04}", adj, noun, rand_num(&mut self.rng)),
        })
    }
}

fn rand_num(rng: &mut ThreadRng) -> u16 {
    rng.gen_range(1..10000)
}
