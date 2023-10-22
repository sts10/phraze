use rand::distributions::{Distribution, Uniform};
use rand::{seq::SliceRandom, thread_rng, Rng};

/// The four possible word lists that Phraze can use.
pub enum List {
    Long,
    Medium,
    Qwerty,
    Alpha,
    Eff,
}

/// Actually generate the passphrase, give a couple neccessary parameters.
pub fn generate_passphrase(
    number_of_words: u8,
    separator: &str,
    title_case: bool,
    list_to_use: List,
) -> String {
    let mut rng = thread_rng();
    let list = make_list(list_to_use);

    let mut passphrase = String::new();
    for i in 0..number_of_words {
        loop {
            // Check if we're doing title_case
            let random_word = if title_case {
                make_title_case(&get_random_element(&mut rng, &list))
            } else {
                get_random_element(&mut rng, &list)
            };
            // Weirdly, sometimes we get a blank word. I'm investigating
            // but for now this little loop and check will
            // have to do.
            if random_word.trim() != "" {
                // add this word to our passphrase
                passphrase += &random_word;
                // break out of `loop`
                break;
            }
        }
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
            .split('\n')
            .collect(),
        List::Long => include_str!("../word-lists/orchard-street-long.txt")
            .split('\n')
            .collect(),
        List::Qwerty => include_str!("../word-lists/orchard-street-qwerty.txt")
            .split('\n')
            .collect(),
        List::Alpha => include_str!("../word-lists/orchard-street-alpha.txt")
            .split('\n')
            .collect(),
        List::Eff => include_str!("../word-lists/eff-long.txt")
            .split('\n')
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
    Uniform::from(0..10).sample(rng).to_string()
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
