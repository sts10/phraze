use rand::Rng;

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

/// Get either a random number or symbol. 50/50 chance!
fn get_random_number_or_symbol(rng: &mut impl Rng) -> String {
    let x: f64 = rng.gen();
    if x > 0.5 {
        get_random_symbol(rng)
    } else {
        get_random_number(rng)
    }
}

/// Pick a random symbol for a separator between words.
fn get_random_symbol(rng: &mut impl Rng) -> String {
    const CHARSET: &[u8] = b"!@#$%&*(){}[]\\:;'<>?,./_-+=";
    let idx = rng.gen_range(0..CHARSET.len());
    (CHARSET[idx] as char).to_string()
}

/// Pick a random digit (0 to 9) for a separator between words.
fn get_random_number(rng: &mut impl Rng) -> String {
    rng.gen_range(0..=9).to_string()
}
