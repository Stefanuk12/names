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

/// A naming strategy for the [`Generator`]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Name {
    /// This represents a plain naming strategy of the form `"ADJECTIVE-NOUN"`
    Plain,
    /// This represents a naming strategy with a random number appended to the
    /// end, of the form `"ADJECTIVE-NOUN{seperator}NUMBER"`
    Numbered(usize, NumberSeperator),
    /// This represents a naming strategy with a zero-padded number appended to
    /// the end, of the form `"ADJECTIVE-NOUN{seperator}NUMBER"`
    ZeroPaddedNumbered(usize, NumberSeperator),
}

impl Default for Name {
    fn default() -> Self {
        Name::Plain
    }
}

/// A seperator for the [`Generator`]. This is only applied if there are any digits on the end or within certain [`Casing`]s.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NumberSeperator {
    /// This represents a seperator of the form `"ADJECTIVE-NOUN"`
    Dash,
    /// This represents a seperator of the form `"ADJECTIVE_NOUN"`
    Underscore,
    /// A custom seperator
    Custom(&'static str),
    /// This represents no seperator of the form `"ADJECTIVENOUN"`
    None,
}

impl Default for NumberSeperator {
    fn default() -> Self {
        NumberSeperator::Dash
    }
}
impl std::fmt::Display for NumberSeperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberSeperator::Dash => write!(f, "-"),
            NumberSeperator::Underscore => write!(f, "_"),
            NumberSeperator::Custom(s) => write!(f, "{}", s),
            NumberSeperator::None => write!(f, ""),
        }
    }
}

/// A length for the [`Generator`]
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

/// A casing style for the [`Generator`]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Casing {
    /// This represents a casing style of the form `"adjective-noun"`
    Lowercase(NumberSeperator),
    /// This represents a casing style of the form `"ADJECTIVE-NOUN"`
    Uppercase(NumberSeperator),
    /// This represents a casing style of the form `"Adjective-Noun"`
    Capitalize(NumberSeperator),
    /// This represents a casing style of the form `"Adjective-noun"`
    CapitalizeFirst(NumberSeperator),
    /// This represents a casing style of the form `"adjective-Noun"`
    CapitalizeLast(NumberSeperator),
    /// This represents a casing style of the form `"adjective_noun"`
    SnakeCase,
    /// This represents a casing style of the form `"ADJECTIVE_NOUN"`
    ScreamingSnakeCase,
    /// This represents a casing style of the form `"adjectiveNoun"`
    CamelCase,
    /// This represents a casing style of the form `"AdjectiveNoun"`
    PascalCase,
    /// This represents a casing style of the form `"adjective-noun"`
    KebabCase,
    /// This represents a casing style of the form `"ADJECTIVE-NOUN"`
    ScreamingKebabCase,
}

impl Default for Casing {
    fn default() -> Self {
        Casing::Lowercase(NumberSeperator::Dash)
    }
}
impl Casing {
    /// Returns the seperator for the casing style
    pub fn seperator(&self) -> String {
        match self {
            Casing::Lowercase(seperator) => seperator.to_string(),
            Casing::Uppercase(seperator) => seperator.to_string(),
            Casing::Capitalize(seperator) => seperator.to_string(),
            Casing::CapitalizeFirst(seperator) => seperator.to_string(),
            Casing::CapitalizeLast(seperator) => seperator.to_string(),
            Casing::SnakeCase => "_".to_string(),
            Casing::ScreamingSnakeCase => "_".to_string(),
            Casing::CamelCase => "".to_string(),
            Casing::PascalCase => "".to_string(),
            Casing::KebabCase => "-".to_string(),
            Casing::ScreamingKebabCase => "-".to_string(),
        }
    }

    /// Applies the casing style to the given words
    pub fn apply(&self, words: Vec<&str>) -> String {
        match self {
            Casing::Lowercase(seperator) => words.join(seperator.to_string().as_str()).to_lowercase(),
            Casing::Uppercase(seperator) => words.join(seperator.to_string().as_str()).to_uppercase(),
            Casing::Capitalize(seperator) => words
                .into_iter()
                .map(|word| {
                    let mut c = word.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(seperator.to_string().as_str()),
            Casing::CapitalizeFirst(seperator) => words
                .into_iter()
                .enumerate()
                .map(|(i, word)| {
                    if i == 0 {
                        let mut c = word.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                        }
                    } else {
                        word.to_lowercase()
                    }
                })
                .collect::<Vec<_>>()
                .join(seperator.to_string().as_str()),
            Casing::CapitalizeLast(seperator) => words
                .iter()
                .enumerate()
                .map(|(i, word)| {
                    if i == words.len() - 1 {
                        let mut c = word.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                        }
                    } else {
                        word.to_lowercase()
                    }
                })
                .collect::<Vec<_>>()
                .join(seperator.to_string().as_str()),
            Casing::SnakeCase => words.join("_").to_lowercase(),
            Casing::ScreamingSnakeCase => words.join("_").to_uppercase(),
            Casing::CamelCase => words
                .into_iter()
                .enumerate()
                .map(|(i, word)| {
                    if i == 0 {
                        word.to_lowercase()
                    } else {
                        let mut c = word.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
            Casing::PascalCase => Casing::Capitalize(NumberSeperator::None).apply(words),
            Casing::KebabCase => words.join("-").to_lowercase(),
            Casing::ScreamingKebabCase => words.join("-").to_uppercase(),
        }
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
    /// The casing to use.
    casing: Casing,
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
        let combined = self.casing.apply(vec![adj, noun]);

        let mut generated = match self.naming {
            Name::Plain => combined,
            Name::Numbered(x, num_sep) => format!("{combined}{num_sep}{}", generate_number_with_x_digits(x, &mut self.rng)),
            Name::ZeroPaddedNumbered(x, num_sep) => format!("{combined}{num_sep}{}", generate_padded_number_with_x_digits(x, &mut self.rng)),
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