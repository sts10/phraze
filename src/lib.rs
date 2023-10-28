pub mod file_reader;
pub mod unicode_normalization_check;
use rand::{seq::SliceRandom, thread_rng, Rng};

// Pull in the wordlists as constants for us to use later.
// This is thanks to the build.rs build script. Learn more:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/wordlists.rs"));

/// The possible word lists that Phraze can use.
#[derive(Clone, Debug, Copy)]
pub enum ListChoice {
    Long,
    Medium,
    Eff,
    Mnemonicode,
    Effshort,
    Qwerty,
    Alpha,
}

/// Given user's inputs, figure out how many words the generated passphrase will need. If user
/// specified an exact number_of_words, just return that number_of_words. If user is using a
/// strength_count, do the necessary math. If user specified a minimum_entropy, we need to do
/// some math to figure out how many words will clear that minimum.
pub fn calculate_number_words_needed(
    number_of_words: Option<usize>,
    minimum_entropy: Option<usize>,
    strength_count: u8,
    list_length: usize,
) -> usize {
    // If a number of words was requested exactly by the user, use that
    if let Some(number_of_words) = number_of_words {
        return number_of_words;
    }

    const DEFAULT_MINIMUM_ENTROPY: usize = 80;
    // If they used the strength count option, do some math to calculate what minimum_entropy
    // we should give them, then convert that into number of bits of entropy.
    if strength_count > 0 {
        // Use number of Ss to calculate minimum_entropy in bits
        let minimum_entropy = DEFAULT_MINIMUM_ENTROPY + (strength_count as usize) * 20;
        // convert this into number of words, using list length
        return convert_minimum_entropy_to_number_of_words(minimum_entropy, list_length);
    }
    // If we made it here, that means either the user requested a specific minimum_entropy in bits,
    // or they entered no relevant settings. Let's handle both cases with a match statement.
    match minimum_entropy {
        // If a minimum_entropy is set by user, use that.
        Some(minimum_entropy) => {
            convert_minimum_entropy_to_number_of_words(minimum_entropy, list_length)
        }
        // If none of these 3 settings were given, use the DEFAULT_MINIMUM_ENTROPY
        None => convert_minimum_entropy_to_number_of_words(DEFAULT_MINIMUM_ENTROPY, list_length),
    }
}

/// A little helper function to actually calculate the number of words needed to meet a desired
/// minimum entropy, given the length of the word list we're using.
pub fn convert_minimum_entropy_to_number_of_words(
    minimum_entropy: usize,
    list_length: usize,
) -> usize {
    let entropy_per_word_from_this_list = (list_length as f64).log2();
    (minimum_entropy as f64 / entropy_per_word_from_this_list).ceil() as usize
}

/// Take enum of list_choice and find the constant that is the corresponding word list (with the
/// actual words). These are defined in the build script (build.rs)
pub fn fetch_list(list_choice: ListChoice) -> &'static [&'static str] {
    match list_choice {
        ListChoice::Long => WL_LONG,
        ListChoice::Medium => WL_MEDIUM,
        ListChoice::Qwerty => WL_QWERTY,
        ListChoice::Alpha => WL_ALPHA,
        ListChoice::Eff => WL_EFF,
        ListChoice::Effshort => WL_EFFSHORT,
        ListChoice::Mnemonicode => WL_MNEMONICODE,
    }
}

/// Actually generate the passphrase, given a couple neccessary parameters.
/// This function uses some Rust magic to be able to accept a word list as
/// either a &[&str] (built-in word lists) or as a &[String] if user provides a file
/// as word list.
pub fn generate_passphrase<T: AsRef<str> + std::fmt::Display>(
    number_of_words_to_put_in_passphrase: usize,
    separator: &str,
    title_case: bool,
    list: &[T], // Either type!
) -> String {
    let mut rng = thread_rng();
    // Create a blank String to put words into to create our passphrase
    let mut passphrase = String::new();
    for i in 0..number_of_words_to_put_in_passphrase {
        // Check if we're doing title_case
        let random_word = if title_case {
            make_title_case(&get_random_element(&mut rng, list))
        } else {
            get_random_element(&mut rng, list)
        };
        // Add this word to our passphrase
        passphrase += &random_word;
        // Add a separator
        if i != number_of_words_to_put_in_passphrase - 1 {
            passphrase += &make_separator(&mut rng, separator);
        }
    }
    passphrase.to_string()
}

/// Parse user's separator choice. The only reason we need this as its own function is to check if
/// they chose a "special" separator
fn make_separator(rng: &mut impl Rng, sep: &str) -> String {
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

/// Give an array of words, pick a random element and make it a String for
/// simplicity's sake.
fn get_random_element<T: AsRef<str>>(rng: &mut impl Rng, word_list: &[T]) -> String
where
    T: std::fmt::Display,
{
    match word_list.choose(rng) {
        Some(word) => word.to_string(),
        None => panic!("Couldn't pick a random word"),
    }
}

/// Make first character of a given &str uppercase
fn make_title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
