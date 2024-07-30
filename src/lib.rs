pub mod cli;
pub mod file_reader;
pub mod separators;
pub mod unicode_normalization_check;

use crate::cli::ListChoice;
use crate::separators::make_separator;
use include_lines::include_lines;
use rand::{seq::SliceRandom, thread_rng, Rng};

/// Given user's inputs, figure out how many words the generated passphrase will need. If user
/// specified an exact `number_of_words`, just return that `number_of_words`. If user is using a
/// strength_count, do the necessary math. If user specified a `minimum_entropy`, we need to do
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

/// Calculate the number of words needed to meet a desired
/// minimum entropy, given the length of the word list we're using.
pub fn convert_minimum_entropy_to_number_of_words(
    minimum_entropy: usize,
    list_length: usize,
) -> usize {
    let entropy_per_word_from_this_list = (list_length as f64).log2();
    (minimum_entropy as f64 / entropy_per_word_from_this_list).ceil() as usize
}

/// Take enum of `list_choice` and use the `include_lines!` macro (from crate)
/// to read-in the correct word list.
pub fn fetch_list(list_choice: ListChoice) -> &'static [&'static str] {
    match list_choice {
        ListChoice::Long => &include_lines!("word-lists/orchard-street-long.txt"),
        ListChoice::Medium => &include_lines!("word-lists/orchard-street-medium.txt"),
        ListChoice::Qwerty => &include_lines!("word-lists/orchard-street-qwerty.txt"),
        ListChoice::Alpha => &include_lines!("word-lists/orchard-street-alpha.txt"),
        ListChoice::Eff => &include_lines!("word-lists/eff-long.txt"),
        ListChoice::Effshort => &include_lines!("word-lists/eff-short-1.txt"),
        ListChoice::Mnemonicode => &include_lines!("word-lists/mnemonicode.txt"),
    }
}

/// Actually generate the passphrase, given a couple neccessary parameters.
/// This function uses some Rust magic to be able to accept a word list as
/// either a `&[&str]` (if the users uses a built-in word lists) or as a
/// `&[String]` (if user provides a file as word list).
pub fn generate_a_passphrase<T: AsRef<str> + std::fmt::Display>(
    number_of_words_to_put_in_passphrase: usize,
    separator: &str,
    title_case: bool,
    list: &[T], // We accept either type by using `T`!
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

/// Given an array of words, pick a random element. Then make  
/// the selected word a `String` for simplicity's sake.
fn get_random_element<T: AsRef<str>>(rng: &mut impl Rng, word_list: &[T]) -> String
where
    T: std::fmt::Display,
{
    match word_list.choose(rng) {
        Some(word) => word.to_string(),
        None => panic!("Couldn't pick a random word"),
    }
}

/// Make given string slice `s` all lowercase, then make first character uppercase
fn make_title_case(s: &str) -> String {
    // First, make entire word lowercase
    let s = s.to_lowercase();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[test]
fn can_make_word_title_case() {
    let test_word = "alpha";
    assert_eq!(make_title_case(test_word), "Alpha".to_string());
    let test_word = "ALPHA";
    assert_eq!(make_title_case(test_word), "Alpha".to_string());
    let test_word = "aLPHA";
    assert_eq!(make_title_case(test_word), "Alpha".to_string());
    let test_word = "aLPhA";
    assert_eq!(make_title_case(test_word), "Alpha".to_string());
}

/// Print the calculated (estimated) entropy of a passphrase, based on three variables
pub fn print_entropy(number_of_words: usize, list_length: usize, n_passphrases: usize) {
    let passphrase_entropy = (list_length as f64).log2() * number_of_words as f64;
    // Depending on how many different passphrases the user wants printed, change the printed text
    // accordingly
    if n_passphrases == 1 {
        eprintln!(
            "Passphrase has an estimated {:.2} bits of entropy ({} words from a list of {} words)",
            passphrase_entropy, number_of_words, list_length,
        );
    } else {
        eprintln!(
            "Each passphrase has an estimated {:.2} bits of entropy ({} words from a list of {} words)",
            passphrase_entropy, number_of_words, list_length
        );
    }
}
