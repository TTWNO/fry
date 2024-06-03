#![no_std]

extern crate alloc;

use num2words::{
	Num2Words,
	Num2Err,
	Currency
};
use alloc::{
	string::{String, ToString},
	boxed::Box,
	vec::Vec,
};
use regex::Regex;
use num_bigfloat::BigFloat;

/// Asserts that two `&str`s, the first passed through the [`normalize`]
/// function, and the second compared against the original are equal.
macro_rules! normalize_check {
	($string:expr, $result:expr) => {
		assert_eq!(
			normalize($string),
			$result.to_string(),
		);
	}
}

macro_rules! regex_check {
	($regex:expr, $string:expr, $result:expr) => {
		assert_eq!(
			regex_get_all!($regex, $string),
			$result,
		);
	}
}

macro_rules! regex_get_all {
	($regex:expr, $string:expr) => {
		{
			let regex = Regex::new($regex).unwrap();
			$string.split_whitespace()
				.filter(|word| if let Some(mch) = regex.find(word) {
					mch.start() == 0 && mch.end() == word.len()
				} else { false })
				.collect::<Vec<&str>>()
		}
	}
}

macro_rules! regex_match {
	($regex:expr, $func:ident, $list:ident) => {
		{
			let boxed: Box<dyn NormalizationHandler> = Box::new($func);
			let set = (
				Regex::new($regex).unwrap(),
				boxed
			);
			$list.push(set);
		}
	}
}

macro_rules! auto_into_enum {
	($type:ty, $variant:path, $inner:ty) => {
		impl From<$inner> for $type {
			fn from(inner: $inner) -> $type {
				$variant(inner)
			}
		}
	}
}

#[derive(Debug)]
pub enum Error {
	Regex(regex::Error),
	Num(Num2Err),
	NumberParsing,
}
auto_into_enum!(Error, Error::Regex, regex::Error);
auto_into_enum!(Error, Error::Num, Num2Err);

pub trait NormalizationHandler {
	fn normalize(&self, input: &str) -> String;
}
impl<F> NormalizationHandler for F
where
		F: Fn(&str) -> Result<String, Error> {
	fn normalize(&self, input: &str) -> String {
		self(input).unwrap_or(input.to_string())
	}
}

type NormalizationItems = Vec<(Regex, Box<dyn NormalizationHandler>)>;

fn normalize_number(input: &str) -> Result<String, Error> {
	let ordinal = input.ends_with("th") || input.ends_with("st") || input.ends_with("nd");
	let currency = input.starts_with("$");
	// remove all non-number values
	let number_only = input.chars().filter(|c| ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(c)).collect::<String>();
	let mut ntw = Num2Words::new(BigFloat::parse(&number_only).ok_or(Error::NumberParsing)?);
	if currency {
		ntw = ntw.currency(Currency::DOLLAR);
	}
	if ordinal {
		ntw = ntw.ordinal();
	}
	Ok(ntw.to_words()?)
}
fn normalize_word(word: &str) -> Result<String, Error> {
	Ok(word.to_string())
}
fn normalize_symbol(_sym: &str) -> Result<String, Error> {
	todo!()
}

const NUMBER_REGEX: &str = "\\$?[0-9,]+((st)|(nd)|(th))?";
const REGEX_WORD: &str = "[a-zA-Z]?[a-z']+";
// All uppercasae words are symbols and are spoken letter by letter
const SYMBOL_REGEX: &str = "[A-Z.]+";

fn regex_map() -> Result<NormalizationItems, Error> {
	let mut resp = Vec::new();
	regex_match!(NUMBER_REGEX, normalize_number, resp);
	regex_match!(REGEX_WORD, normalize_word, resp);
	regex_match!(SYMBOL_REGEX, normalize_symbol, resp);
	Ok(resp)
}

fn normalize(s: &str) -> String {
	let rm = regex_map().unwrap();
	s.split_whitespace()
		.map(|word| {
			for (regex, func) in rm.iter() {
				if let Some(mtc) = regex.find(word) {
					if mtc.start() == 0 && mtc.end() == word.len() {
						return func.normalize(word);
					}
				}
			}
			word.to_string()
		})
		.collect::<Vec<String>>()
		.join(" ")
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;

	#[test]
	fn regex_loads_successfully() {
		let map = regex_map().unwrap();
		assert!(!map.is_empty(), "The map must contain at least one item");
	}
	#[test]
	fn test_number_normalization() {
		normalize_check!("I am 4096 years old", "I am four thousand and ninety-six years old");
		normalize_check!("I have $10 in gift cards", "I have ten dollars in gift cards");
		// in English, we would actually say "one hundred thousand dollar prize", not dollars
		normalize_check!("He won a $100,000 prize!", "He won a one hundred thousand dollars prize!");
		normalize_check!("He won 1st place!", "He won first place!");
		normalize_check!("He won 12th place!", "He won twelfth place!");
		normalize_check!("He won 582nd place!", "He won five hundred eighty-second place!");
		normalize_check!("He won $1,000,000", "He won one million dollars");
		normalize_check!("He won 2938345 ducks", "He won two million nine hundred thirty-eight thousand three hundred and forty-five ducks");
	}
	#[test]
	fn test_number_regex() {
		regex_check!(
			NUMBER_REGEX,
			"I am 4096 years old",
			vec!["4096"]
		);
		regex_check!(
			NUMBER_REGEX,
			"I have $10",
			vec!["$10"]
		);
		regex_check!(
			NUMBER_REGEX,
			"The 10th player",
			vec!["10th"]
		);
		regex_check!(
			NUMBER_REGEX,
			"The 10stth player",
			Vec::<&str>::new()
		);
	}
}

