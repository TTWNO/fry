#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use derive_more::Display;
use num2words::{Currency, Num2Err, Num2Words};
use num_bigfloat::BigFloat;
use once_cell::sync::Lazy;
use regex::Regex;

/// Asserts that two `&str`s, the first passed through the [`normalize`]
/// function, and the second compared against the original are equal.
#[cfg(test)]
macro_rules! normalize_check {
    ($string:expr, $result:expr) => {
        assert_eq!(
            $string
                .split_whitespace()
                .map(TaggedWord::from_str)
                .map(|s| s.normalize())
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            $result.to_string(),
        );
    };
}

#[cfg(test)]
macro_rules! tagging_check {
    ($string:expr, $result:expr) => {
        assert_eq!(
            $string
                .split_whitespace()
                .map(TaggedWord::from_str)
                .collect::<Vec<TaggedWord>>(),
            $result,
        );
    };
}

#[cfg(test)]
macro_rules! regex_check {
    ($regex:expr, $string:expr, $result:expr) => {
        assert_eq!(regex_get_all!($regex, $string), $result,);
    };
}

#[cfg(test)]
macro_rules! regex_get_all {
    ($regex:expr, $string:expr) => {{
        $string
            .split_whitespace()
            .filter(|word| {
                if let Some(mch) = $regex.find(word) {
                    mch.start() == 0 && mch.end() == word.len()
                } else {
                    false
                }
            })
            .collect::<Vec<&str>>()
    }};
}

macro_rules! auto_into_enum {
    ($type:ty, $variant:path, $inner:ty) => {
        impl From<$inner> for $type {
            fn from(inner: $inner) -> $type {
                $variant(inner)
            }
        }
    };
}

#[derive(Debug)]
pub enum Error {
    Regex(regex::Error),
    Num(Num2Err),
    NumberParsing,
}
auto_into_enum!(Error, Error::Regex, regex::Error);
auto_into_enum!(Error, Error::Num, Num2Err);

fn normalize_number(input: &str) -> Result<TaggedWord, Error> {
    let ordinal = input.ends_with("th") || input.ends_with("st") || input.ends_with("nd");
    let currency = input.starts_with("$");
    // remove all non-number values
    let number_only = input
        .chars()
        .filter(|c| ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(c))
        .collect::<String>();
    let mut ntw = Num2Words::new(BigFloat::parse(&number_only).ok_or(Error::NumberParsing)?);
    if currency {
        ntw = ntw.currency(Currency::DOLLAR);
    }
    if ordinal {
        ntw = ntw.ordinal();
    }
    Ok(TaggedWord::Number(ntw.to_words()?))
}
fn normalize_word(word: &str) -> Result<TaggedWord, Error> {
    Ok(TaggedWord::Word(word.to_string()))
}
fn normalize_abbr(word: &str) -> Result<TaggedWord, Error> {
    if let Some(abbr) = ABBR_DICT.get(word) {
        Ok(TaggedWord::Abbr(abbr.to_string()))
    } else {
        Ok(TaggedWord::Word(word.to_string()))
    }
}
fn normalize_symbol(sym: &str) -> Result<TaggedWord, Error> {
    Ok(TaggedWord::Word(
        sym.chars()
            .map(|c| c.to_uppercase().to_string())
            .collect::<Vec<String>>()
            .join("."),
    ))
}
macro_rules! regex_m {
    ($reg:expr, $variant:path, $word:ident) => {
        if let Some(mch) = $reg.find($word) {
            if mch.start() == 0 && mch.end() == $word.len() {
                return $variant($word.to_string());
            }
        }
    };
}

#[derive(Debug, PartialEq, Display)]
enum TaggedWord {
    Word(String),
    Number(String),
    Symbol(String),
    Abbr(String),
}
impl From<TaggedWord> for String {
    fn from(tw: TaggedWord) -> String {
        match tw {
            TaggedWord::Word(word) => word,
            TaggedWord::Number(word) => word,
            TaggedWord::Symbol(word) => word,
            TaggedWord::Abbr(word) => word,
        }
    }
}
impl TaggedWord {
    fn from_str<S: AsRef<str>>(s: S) -> Self {
        let s: &str = s.as_ref();
        regex_m!(SYMBOL_REGEX, TaggedWord::Symbol, s);
        regex_m!(NUMBER_REGEX, TaggedWord::Number, s);
        if ABBR_DICT.get(s).is_some() {
            return TaggedWord::Abbr(s.to_string());
        }
        TaggedWord::Word(s.to_string())
    }
    fn normalize(self) -> Self {
        match self {
            Self::Word(ref word) => normalize_word(word),
            Self::Number(ref word) => normalize_number(word),
            Self::Symbol(ref word) => normalize_symbol(word),
            Self::Abbr(ref word) => normalize_abbr(word),
        }
        .unwrap_or(Self::Word(self.into()))
    }
}

const NUMBER_REGEX_STR: &str = "\\$?[0-9,]+((st)|(nd)|(th))?";
static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(NUMBER_REGEX_STR).unwrap());
// All uppercasae words are symbols and are spoken letter by letter
const SYMBOL_REGEX_STR: &str = "[A-Z.]{2,}";
static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(SYMBOL_REGEX_STR).unwrap());
static ABBR_DICT: Lazy<BTreeMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut abbr_dict = BTreeMap::new();
    let text = include_str!("../data/abbr.txt");
    for line in text.lines() {
        let (from, to) = line.split_once(" = ").unwrap();
        abbr_dict.insert(from, to);
    }
    abbr_dict
});

pub fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .map(TaggedWord::from_str)
        .map(|s| s.normalize())
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn regex_loads_successfully() {
        //let map = regex_map().unwrap();
        //assert!(!map.is_empty(), "The map must contain at least one item");
    }
    #[test]
    fn test_tagging() {
        tagging_check!(
            "Dr. Kaur is my Prof.",
            vec![
                TaggedWord::Abbr("Dr.".to_string()),
                TaggedWord::Word("Kaur".to_string()),
                TaggedWord::Word("is".to_string()),
                TaggedWord::Word("my".to_string()),
                TaggedWord::Abbr("Prof.".to_string()),
            ]
        );
        tagging_check!(
            "I am 4096 years old",
            vec![
                TaggedWord::Word("I".to_string()),
                TaggedWord::Word("am".to_string()),
                TaggedWord::Number("4096".to_string()),
                TaggedWord::Word("years".to_string()),
                TaggedWord::Word("old".to_string()),
            ]
        );
        tagging_check!(
            "He went to MIT",
            vec![
                TaggedWord::Word("He".to_string()),
                TaggedWord::Word("went".to_string()),
                TaggedWord::Word("to".to_string()),
                TaggedWord::Symbol("MIT".to_string()),
            ]
        );
    }
    #[test]
    fn test_number_normalization() {
        normalize_check!(
            "I am 4096 years old",
            "I am four thousand and ninety-six years old"
        );
        normalize_check!(
            "I have $10 in gift cards",
            "I have ten dollars in gift cards"
        );
        // in English, we would actually say "one hundred thousand dollar prize", not dollars
        normalize_check!(
            "He won a $100,000 prize!",
            "He won a one hundred thousand dollars prize!"
        );
        normalize_check!("He won 1st place!", "He won first place!");
        normalize_check!("He won 12th place!", "He won twelfth place!");
        normalize_check!(
            "He won 582nd place!",
            "He won five hundred eighty-second place!"
        );
        normalize_check!("He won $1,000,000", "He won one million dollars");
        normalize_check!("He won 2938345 ducks", "He won two million nine hundred thirty-eight thousand three hundred and forty-five ducks");
    }
    #[test]
    fn test_number_regex() {
        regex_check!(NUMBER_REGEX, "I am 4096 years old", vec!["4096"]);
        regex_check!(NUMBER_REGEX, "I have $10", vec!["$10"]);
        regex_check!(NUMBER_REGEX, "The 10th player", vec!["10th"]);
        regex_check!(NUMBER_REGEX, "The 10stth player", Vec::<&str>::new());
    }
}
