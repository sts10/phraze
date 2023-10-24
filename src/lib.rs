use rand::{seq::SliceRandom, thread_rng, Rng};

/// The possible word lists that Phraze can use.
pub enum List {
    Long,
    Medium,
    Eff,
    Mnemonicode,
    Effshort,
    Qwerty,
    Alpha,
}

/// Given user's inputs, figure out how many words the generated passphrase will need. If user
/// specified an exact number_of_words, just return that number_of_words. If user specified a
/// minimum_entropy, we need to do some math to figure out how many words will clear that minimum.
pub fn calculate_number_words_needed(
    number_of_words: Option<usize>,
    minimum_entropy: Option<usize>,
    list_length: usize,
) -> usize {
    // If they specified an exact number of words, give them that.
    if let Some(number_of_words) = number_of_words {
        return number_of_words.into();
    }
    // Check if they gave a minimum_entropy instead
    let minimum_entropy = match minimum_entropy {
        Some(minimum_entropy) => minimum_entropy,
        None => {
            // Neither specified
            // Use a default minimum_entropy
            80
        }
    };
    // Do the actual math
    let entropy_per_word_from_this_list = (list_length as f64).log2();
    (minimum_entropy as f64 / entropy_per_word_from_this_list).ceil() as usize
}

/// Actually generate the passphrase, give a couple neccessary parameters.
pub fn generate_passphrase(
    number_of_words: Option<usize>,
    minimum_entropy: Option<usize>,
    separator: &str,
    title_case: bool,
    list_to_use: List,
) -> String {
    // Make our word list based on user choice
    let list = make_list(list_to_use);

    let number_of_words =
        calculate_number_words_needed(number_of_words, minimum_entropy, list.len());

    let mut rng = thread_rng();
    // Create a blank String for our passphrase
    let mut passphrase = String::new();
    for i in 0..number_of_words {
        // Check if we're doing title_case
        let random_word = if title_case {
            make_title_case(&get_random_element(&mut rng, &list))
        } else {
            get_random_element(&mut rng, &list)
        };
        // Add random_word to our passphrase
        passphrase += &random_word;
        // Add a separator
        if i != number_of_words - 1 {
            passphrase += &make_separator(&mut rng, separator);
        }
    }
    passphrase.to_string()
}

fn make_separator(rng: &mut impl Rng, sep: &str) -> String {
    match sep {
        "_n" => get_random_number(rng),
        "_s" => get_random_symbol(rng),
        "_b" => get_random_number_or_symbol(rng),
        _ => sep.to_string(),
    }
}

/// Read in the appropriate word list, give the desired list enum
fn make_list(list_to_use: List) -> Vec<&'static str> {
    match list_to_use {
        List::Medium => include_str!("../word-lists/orchard-street-medium.txt")
            .lines()
            .collect(),
        List::Long => include_str!("../word-lists/orchard-street-long.txt")
            .lines()
            .collect(),
        List::Qwerty => include_str!("../word-lists/orchard-street-qwerty.txt")
            .lines()
            .collect(),
        List::Alpha => include_str!("../word-lists/orchard-street-alpha.txt")
            .lines()
            .collect(),
        List::Eff => include_str!("../word-lists/eff-long.txt").lines().collect(),
        List::Effshort => include_str!("../word-lists/eff-short-1.txt")
            .lines()
            .collect(),
        List::Mnemonicode => include_str!("../word-lists/mnemonicode.txt")
            .lines()
            .collect(),
    }
}

fn get_random_number_or_symbol(rng: &mut impl Rng) -> String {
    let x: f64 = rng.gen();
    if x > 0.5 {
        get_random_symbol(rng)
    } else {
        get_random_number(rng)
    }
}

fn get_random_symbol(rng: &mut impl Rng) -> String {
    const CHARSET: &[u8] = b"!@#$%^&*(){}[]\\:;'<>?,./_-+=";
    let idx = rng.gen_range(0..CHARSET.len());
    (CHARSET[idx] as char).to_string()
}

fn get_random_number(rng: &mut impl Rng) -> String {
    rng.gen_range(0..=9).to_string()
}

/// Give an array of words, pick a random element and make it a String for
/// simplicity's sake.
fn get_random_element(rng: &mut impl Rng, word_list: &[&str]) -> String {
    match word_list.choose(rng) {
        Some(word) => word.to_string(),
        None => panic!("Couldn't pick a random word"),
    }
}

/// Make first character of a &str uppercase
fn make_title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
