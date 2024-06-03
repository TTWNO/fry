#![no_std]

extern crate alloc;

use num_bigfloat::BigFloat;
use num2words::Num2Words;

use alloc::collections::BTreeMap;
use alloc::string::{ToString, String};

pub mod types;

const ABBREVIATIONS: &'static str = include_str!("../data/abbr.txt");
const NUMBER_SYMBOLS: &'static str = "0123456789$.,";

fn generate_abbr_table() -> Option<BTreeMap<&'static str, &'static str>> {
    let mut map = BTreeMap::new();
    for pair in ABBREVIATIONS.lines() {
        let (k,v) = pair.split_once(" -> ")?;
        map.insert(k, v);
    }
    Some(map)
}

/// If a number sequence has been identified, you can pass it to this function to create a new
/// string representing the English speech identified by the number.
/// This function can handle dollar signs, and no other currencies.
/// This funtion only understands North American number format: NNN,NNN.FF;
/// it does not handle European-style formatting: NNN.NNN,FF
/// Any values which are not `[0-9]`, `$`, `,`, or `.` will be ignored.
fn number_conversion(slc: &str) -> String {
    let values: String = slc.to_string().chars()
        .filter(|c| NUMBER_SYMBOLS.find(*c).is_some())
        .collect();
    let is_currency = values.find('$') == Some(0);
    let is_point = values.find('.').is_some();
    let f = Num2Words::new(BigFloat::parse(&values).unwrap());
    values
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn abbr_table_loads_successfully() {
        let abbr_table = generate_abbr_table().expect("An abbreviation table must be generated");
        assert_eq!(abbr_table.len(), ABBREVIATIONS.lines().count(), "Abbreviation table must be the same length as there is lines");
    }
}
