#![cfg(feature = "std")]

use std::collections::BTreeSet;
use fry_pronounce::{
    PRONOUNCIATION_DICT,
    Sound,
};

#[test]
fn test_hello_goodbye() {
    let hello = PRONOUNCIATION_DICT.get("hello").expect("hello should be in the dictionary");
    let goodbye = PRONOUNCIATION_DICT.get("goodbye").expect("hello should be in the dictionary");
    assert_eq!(hello, &vec![Sound::HH, Sound::AH, Sound::L, Sound::OW]);
    assert_eq!(goodbye, &vec![Sound::G, Sound::UH, Sound::D, Sound::B, Sound::AY]);
}

