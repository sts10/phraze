//! This module contains some functions that help deal with the separating punction between words
//! in a passphrase. Most of it handles cases where the user requests a random symbol or number or
//! either.
use rand::prelude::*;
use rand::seq::IndexedRandom;

#[derive(PartialEq)]
enum SeparatorType {
    Number,
    Symbol,
}

/// Parse user's separator choice. The only reason we need this as its own function is to check if
/// they chose a "special" separator
pub fn make_separator(rng: &mut impl Rng, sep: &str) -> String {
    match sep {
        "_n" => get_random_number(rng),
        "_s" => get_random_symbol(rng),
        "_b" => get_random_number_or_symbol(rng),
        _ => sep.to_string(),
    }
}

/// Return either a random number or symbol. 50/50 chance!
fn get_random_number_or_symbol(rng: &mut impl Rng) -> String {
    // Randomly choose which separator type to use
    let separator_type_to_use: &SeparatorType = [SeparatorType::Number, SeparatorType::Symbol]
        .choose(rng)
        .unwrap();
    if separator_type_to_use == &SeparatorType::Symbol {
        get_random_symbol(rng)
    } else {
        get_random_number(rng)
    }
}

/// Pick a random symbol for a separator between words.
fn get_random_symbol(rng: &mut impl Rng) -> String {
    const CHARSET: &[u8] = b"!@#$%&*(){}[]\\:;'<>?,./_-+=";
    let idx = rng.random_range(0..CHARSET.len());
    (CHARSET[idx] as char).to_string()
}

/// Pick a random digit (0 to 9) for a separator between words.
fn get_random_number(rng: &mut impl Rng) -> String {
    rng.random_range(0..=9).to_string()
}
