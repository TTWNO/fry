#![cfg(feature = "std")]

use fry_pronounce::{Sound, PRONOUNCIATION_DICT};
use std::collections::BTreeSet;

#[test]
fn test_hello_goodbye() {
    let hello = PRONOUNCIATION_DICT
        .get("hello")
        .expect("hello should be in the dictionary");
    let goodbye = PRONOUNCIATION_DICT
        .get("goodbye")
        .expect("hello should be in the dictionary");
    assert_eq!(hello, &vec![Sound::HH, Sound::AH, Sound::L, Sound::OW]);
    assert_eq!(
        goodbye,
        &vec![Sound::G, Sound::UH, Sound::D, Sound::B, Sound::AY]
    );
}
