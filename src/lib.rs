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
//! use names::{GeneratorBuilder, Name, NumberSeperator, ThreadRng};
//!
//! let mut generator = GeneratorBuilder::default()
//!     .naming(Name::Numbered(4, NumberSeperator::Dash))
//!     .rng(ThreadRng::default())
//!     .build()
//!     .unwrap();
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
//! use names::{GeneratorBuilder, Name, ThreadRng};
//!
//! let adjectives = &["imaginary"];
//! let nouns = &["roll"];
//! let mut generator = GeneratorBuilder::default()
//!     .adjectives(adjectives)
//!     .nouns(nouns)
//!     .rng(ThreadRng::default())
//!     .build()
//!     .unwrap();
//!
//! assert_eq!("imaginary-roll", generator.next().unwrap());
//! ```

#![doc(html_root_url = "https://docs.rs/names/0.14.1-dev")]
#![deny(missing_docs)]

use core::{fmt, str::FromStr, convert::{Infallible, TryFrom}};

use derive_builder::{Builder, UninitializedFieldError};
use rand::{seq::SliceRandom, Rng};
pub use rand::rngs::*;
use serde::{Serialize, Deserialize, Deserializer};

/// List of English adjective words
pub const ADJECTIVES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));

/// List of English noun words
pub const NOUNS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/nouns.rs"));

/// A naming strategy for the [`Generator`]
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NumberSeperator {
    /// This represents a seperator of the form `"ADJECTIVE-NOUN"`
    Dash,
    /// This represents a seperator of the form `"ADJECTIVE_NOUN"`
    Underscore,
    /// A custom seperator
    Custom(String),
    /// This represents no seperator of the form `"ADJECTIVENOUN"`
    None,
}

impl Default for NumberSeperator {
    fn default() -> Self {
        NumberSeperator::Dash
    }
}
impl FromStr for NumberSeperator {
    type Err = Infallible;
    fn from_str(
        s: &str,
    ) -> Result<NumberSeperator, <Self as FromStr>::Err> {
        Result::Ok(match s {
            "-" => NumberSeperator::Dash,
            "_" => NumberSeperator::Underscore,
            "" => NumberSeperator::None,
            _ => return Result::Ok(NumberSeperator::Custom(s.into())),
        })
    }
}
#[allow(clippy::use_self)]
impl TryFrom<&str> for NumberSeperator {
    type Error = Infallible;
    fn try_from(
        s: &str,
    ) -> Result<NumberSeperator, <Self as TryFrom<&str>>::Error>
    {
        FromStr::from_str(s)
    }
}
impl fmt::Display for NumberSeperator {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        match *self {
            NumberSeperator::Dash => f.pad("-"),
            NumberSeperator::Underscore => f.pad("_"),
            NumberSeperator::Custom(ref s) => f.pad(s),
            NumberSeperator::None => f.pad(""),
        }
    }
}
impl Serialize for NumberSeperator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        match self {
            NumberSeperator::Custom(seperator) => serializer.serialize_str(seperator),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}
impl<'de> Deserialize<'de> for NumberSeperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// A length for the [`Generator`]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

fn adjectives<'a>() -> Vec<String> {
    ADJECTIVES.iter().map(|s| s.to_string()).collect()
}
fn nouns<'a>() -> Vec<String> {
    NOUNS.iter().map(|s| s.to_string()).collect()
}

/// All of the errors for this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Uninitialized field
    #[error("uninitialized field: {0}")]
    UninitializedField(&'static str),
    /// Custom validation error
    #[error("validation error: {0}")]
    ValidationError(String),
    /// Adjectives is empty
    #[error("adjectives must not be empty")]
    AdjectivesEmpty,
    /// Nouns is empty
    #[error("nouns must not be empty")]
    NounsEmpty,
    /// The iterator was empty
    #[error("the iterator was empty")]
    EmptyIterator,
}
impl From<UninitializedFieldError> for Error {
    fn from(e: UninitializedFieldError) -> Self { Self::UninitializedField(e.field_name()) }
}
impl From<String> for Error {
    fn from(s: String) -> Self { Self::ValidationError(s) }
}

#[derive(Deserialize)]
struct GeneratorJson {
    #[serde(default = "adjectives")]
    adjectives: Vec<String>,
    #[serde(default = "nouns")]
    nouns: Vec<String>,
    #[serde(default)]
    naming: Name,
    #[serde(default)]
    casing: Casing,
    #[serde(default)]
    length: Length,
}
impl GeneratorJson {
    fn thread_rng(self) -> Generator<ThreadRng> {
        Generator {
            adjectives: self.adjectives,
            nouns: self.nouns,
            naming: self.naming,
            casing: self.casing,
            length: self.length,
            rng: rand::thread_rng(),
        }
    }

    fn os_rng(self) -> Generator<OsRng> {
        Generator {
            adjectives: self.adjectives,
            nouns: self.nouns,
            naming: self.naming,
            casing: self.casing,
            length: self.length,
            rng: OsRng,
        }
    }

    fn std_rng(self) -> Generator<StdRng> {
        use rand::SeedableRng;

        Generator {
            adjectives: self.adjectives,
            nouns: self.nouns,
            naming: self.naming,
            casing: self.casing,
            length: self.length,
            rng: StdRng::from_entropy(),
        }
    }

    fn small_rng(self) -> Generator<SmallRng> {
        use rand::SeedableRng;

        Generator {
            adjectives: self.adjectives,
            nouns: self.nouns,
            naming: self.naming,
            casing: self.casing,
            length: self.length,
            rng: SmallRng::from_entropy(),
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
#[derive(Serialize, Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate", error = "Error"))]
pub struct Generator<R: Rng> {
    /// A slice of adjective words
    #[builder(setter(into), default = "adjectives()")]
    pub adjectives: Vec<String>,
    /// A slice of noun words
    #[builder(setter(into), default = "nouns()")]
    nouns: Vec<String>,
    /// A naming strategy
    #[builder(setter(into), default)]
    #[serde(default)]
    naming: Name,
    #[builder(setter(into), default)]
    #[serde(default)]
    /// The casing to use.
    casing: Casing,
    /// The maximum length of the generated name
    #[builder(setter(into), default)]
    #[serde(default)]
    length: Length,
    #[serde(default)]
    #[serde(skip)]
    /// The random number generator
    rng: R
}

impl<'de> Deserialize<'de> for Generator<ThreadRng> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Ok(GeneratorJson::deserialize(deserializer)?.thread_rng())
    }
}
impl<'de> Deserialize<'de> for Generator<OsRng> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Ok(GeneratorJson::deserialize(deserializer)?.os_rng())
    }
}
impl<'de> Deserialize<'de> for Generator<StdRng> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Ok(GeneratorJson::deserialize(deserializer)?.std_rng())
    }
}
impl<'de> Deserialize<'de> for Generator<SmallRng> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Ok(GeneratorJson::deserialize(deserializer)?.small_rng())
    }
}

impl<R: Rng> GeneratorBuilder<R> {
    fn validate(&self) -> Result<(), Error> {
        if let Some(adjectives) = &self.adjectives {
            if adjectives.is_empty() {
                return Err(Error::AdjectivesEmpty);
            }
        }
        if let Some(nouns) = &self.nouns {
            if nouns.is_empty() {
                return Err(Error::NounsEmpty);
            }
        }
        Ok(())
    }
}

impl Default for Generator<ThreadRng> {
    fn default() -> Self {
        Self {
            adjectives: adjectives(),
            nouns: nouns(),
            naming: Name::Plain,
            casing: Casing::Lowercase(NumberSeperator::Dash),
            length: Length::None,
            rng: rand::thread_rng(),
        }
    }
}
impl Default for Generator<OsRng> {
    fn default() -> Self {
        Self {
            adjectives: adjectives(),
            nouns: nouns(),
            naming: Name::Plain,
            casing: Casing::Lowercase(NumberSeperator::Dash),
            length: Length::None,
            rng: OsRng,
        }
    }
}
impl Default for Generator<StdRng> {
    fn default() -> Self {
        use rand::SeedableRng;

        Self {
            adjectives: adjectives(),
            nouns: nouns(),
            naming: Name::Plain,
            casing: Casing::Lowercase(NumberSeperator::Dash),
            length: Length::None,
            rng: StdRng::from_entropy(),
        }
    }
}
impl Default for Generator<SmallRng> {
    fn default() -> Self {
        use rand::SeedableRng;

        Self {
            adjectives: adjectives(),
            nouns: nouns(),
            naming: Name::Plain,
            casing: Casing::Lowercase(NumberSeperator::Dash),
            length: Length::None,
            rng: SmallRng::from_entropy(),
        }
    }
}

impl<R: Rng> Iterator for Generator<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let adj = self.adjectives.choose(&mut self.rng)?;
        let noun = self.nouns.choose(&mut self.rng)?;
        let combined = self.casing.apply(vec![adj, noun]);

        let mut generated = match &self.naming {
            Name::Plain => combined,
            Name::Numbered(x, num_sep) => format!("{combined}{num_sep}{}", generate_number_with_x_digits(*x, &mut self.rng)),
            Name::ZeroPaddedNumbered(x, num_sep) => format!("{combined}{num_sep}{}", generate_padded_number_with_x_digits(*x, &mut self.rng)),
        };
        
        Some(match self.length {
            Length::Truncate(x) => { generated.truncate(x); generated },
            Length::Reroll(x) => {
                while generated.len() != x {
                    generated = self.next()?;
                }
                generated
            },
            Length::None => generated,
        })
    }
}

fn generate_number_with_x_digits<R: Rng + ?Sized>(x: usize, rng: &mut R) -> usize {
    let lower_bound = 10usize.pow((x - 1) as u32);
    let upper_bound = 10usize.pow(x as u32) - 1;
    rng.gen_range(lower_bound..=upper_bound)
}

fn generate_padded_number_with_x_digits<R: Rng + ?Sized>(x: usize, rng: &mut R) -> String {
    let number = generate_number_with_x_digits(x, rng);
    format!("{:0>width$}", number, width = x)
}