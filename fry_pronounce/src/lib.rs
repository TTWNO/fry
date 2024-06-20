#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Sound {
    AA,
    AE,
    AH,
    AO,
    AW,
    AY,
    B,
    CH,
    D,
    DH,
    EH,
    ER,
    EY,
    F,
    G,
    HH,
    IH,
    IY,
    JH,
    K,
    L,
    M,
    N,
    NG,
    OW,
    OY,
    P,
    R,
    S,
    SH,
    T,
    TH,
    UH,
    UM,
    UW,
    V,
    W,
    Y,
    Z,
    ZH,
}
#[derive(Debug)]
pub enum Error {
    InvalidSound(String),
}
impl TryFrom<&str> for Sound {
    type Error = Error;
    fn try_from(s: &str) -> Result<Sound, Self::Error> {
        match s {
            "AA" => Ok(Self::AA),
            "AE" => Ok(Self::AE),
            "AH" => Ok(Self::AH),
            "AO" => Ok(Self::AO),
            "AW" => Ok(Self::AW),
            "AY" => Ok(Self::AY),
            "B" => Ok(Self::B),
            "CH" => Ok(Self::CH),
            "D" => Ok(Self::D),
            "DH" => Ok(Self::DH),
            "EH" => Ok(Self::EH),
            "ER" => Ok(Self::ER),
            "EY" => Ok(Self::EY),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "HH" => Ok(Self::HH),
            "IH" => Ok(Self::IH),
            "IY" => Ok(Self::IY),
            "JH" => Ok(Self::JH),
            "K" => Ok(Self::K),
            "L" => Ok(Self::L),
            "M" => Ok(Self::M),
            "N" => Ok(Self::N),
            "NG" => Ok(Self::NG),
            "OW" => Ok(Self::OW),
            "OY" => Ok(Self::OY),
            "P" => Ok(Self::P),
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            "SH" => Ok(Self::SH),
            "T" => Ok(Self::T),
            "TH" => Ok(Self::TH),
            "UH" => Ok(Self::UH),
            "UM" => Ok(Self::UM),
            "UW" => Ok(Self::UW),
            "V" => Ok(Self::V),
            "W" => Ok(Self::W),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            "ZH" => Ok(Self::ZH),
            _ => Err(Error::InvalidSound(s.to_string())),
        }
    }
}

pub static PRONOUNCIATION_DICT: Lazy<BTreeMap<String, Vec<Sound>>> = Lazy::new(|| {
    let text = include_str!("../data/cmudict-en-us.dict");
    let mut map = BTreeMap::new();
    for line in text.lines() {
        let (word, pronounciation) = line
            .split_once(" ")
            .expect("Each line must contain a space");
        let sounds = pronounciation
            .split_whitespace()
            .map(|sound| Sound::try_from(sound).expect("Invalid sound"))
            .collect();
        map.insert(word.to_string(), sounds);
    }
    map
});
