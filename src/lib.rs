use rand::seq::SliceRandom;

/// Actually generate the passphrase, give a couple neccessary parameters.
pub fn generate_passphrase(
    number_of_words: u8,
    separator: &str,
    title_case: bool,
    list: &[&str],
) -> String {
    let mut passphrase = String::new();
    for i in 0..number_of_words {
        // Check if we're doing title_case
        let random_word = if title_case {
            make_title_case(&get_random_element(&list))
        } else {
            get_random_element(&list)
        };
        // add this word to our passphrase
        passphrase += &random_word;
        // Add a separator
        if i != number_of_words - 1 {
            passphrase += separator;
        }
    }
    passphrase.to_string()
}

/// Give an array of words, pick a random element and make it a String for
/// simplicity's sake.
fn get_random_element(word_list: &[&str]) -> String {
    match word_list.choose(&mut rand::thread_rng()) {
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
