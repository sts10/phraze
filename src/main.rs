use crate::file_reader::read_in_custom_list;
use clap::Parser;
use phraze::*;
use std::path::PathBuf;

/// Generate random passphrases
#[derive(Parser, Debug)]
#[clap(version, name = "phraze")]
struct Args {
    /// Strengthen your passphrase the easy way: Each -S flag increases minimum entropy by 20 bits (above the default of
    /// 80 bits).
    #[clap(short = 'S', long = "strength", conflicts_with = "number_of_words", conflicts_with = "minimum_entropy", action = clap::ArgAction::Count)]
    strength_count: u8,

    /// Set minimum amount of entropy in bits for generated passphrase. If neither minimum_entropy or
    /// number_of_words is specified, Phraze will default to an 80-bit minimum.
    #[clap(
        short = 'e',
        long = "minimum-entropy",
        conflicts_with = "number_of_words",
        conflicts_with = "strength_count"
    )]
    minimum_entropy: Option<usize>,

    /// Set exactly how many words to use in generated passphrase. If neither number_of_words or
    /// minimum_entropy is specified, Phraze will default to an 80-bit minimum.
    #[clap(
        short = 'w',
        long = "words",
        conflicts_with = "minimum_entropy",
        conflicts_with = "strength_count"
    )]
    number_of_words: Option<usize>,

    /// Number of passphrases to generate
    #[clap(short = 'n', long = "passphrases", default_value = "1")]
    n_passphrases: usize,

    /// Word separator. Can accept single quotes around the separator. To not use a separator,
    /// use empty single quotes: ''.
    ///
    /// There are special values that will trigger generated separators:
    ///
    /// _n: separators will be random numbers
    ///
    /// _s: separators will be random symbols
    ///
    /// _b: separators will be a mix of random numbers and symbols
    #[clap(short = 's', long = "sep", default_value = "-")]
    separator: String,

    /// Choose a word list to use.
    ///
    /// Options:
    ///
    /// m: Orchard Street Medium List (8,192 words) [DEFAULT]
    ///
    /// l: Orchard Street Long List (17,576 words)
    ///
    /// e: EFF long list (7,776 words)
    ///
    /// n: Mnemonicode list (1,633 words). Good if you know you're going to be speaking
    /// passphrases out loud.
    ///
    /// s: EFF short list (1,296 words)
    ///
    /// q: Orchard Street QWERTY list (1,296 words). Optimized to minimize travel
    /// distance on QWERTY keyboard layout.
    ///
    /// a: Orchard Street Alpha list (1,296 words). Optimized to minimize travel
    /// distance on alphabetical keyboard layout.
    #[clap(short = 'l', long = "list", value_parser=parse_list_choice, default_value="m")]
    list_choice: ListChoice,

    /// Provide a text file with a list of words to randomly generate passphrase from.
    ///
    /// Should be a text file with one word per line.
    #[clap(short = 'c', long = "custom-list", conflicts_with = "list_choice")]
    custom_list_file_path: Option<PathBuf>,

    /// Use Title Case for words in generated passphrase
    #[clap(short = 't', long = "title-case")]
    title_case: bool,

    /// Print estimated entropy of generated passphrase, in bits, along with
    /// the passphrase itself
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() -> Result<(), String> {
    let opt = Args::parse();

    // Check for a rare but potentially dangerous combination of settings
    if opt.custom_list_file_path.is_some() && opt.separator.is_empty() && !opt.title_case {
        let error_msg = "Must use a separator or Title Case when using a custom word list";
        return Err(error_msg.to_string());
    }

    match &opt.custom_list_file_path {
        Some(custom_list_file_path) => {
            generate_passphrases(&opt, &read_in_custom_list(custom_list_file_path)?)
        }
        None => generate_passphrases(&opt, fetch_list(opt.list_choice)),
    };

    Ok(())
}

/// This does the real work of the program: generating the passphrases
fn generate_passphrases<T: AsRef<str> + std::fmt::Display>(opt: &Args, word_list: &[T]) {
    // Since user can define a minimum entropy, we might have to do a little math to
    // figure out how many words we need to include in this passphrase.
    let number_of_words_to_put_in_passphrase = calculate_number_words_needed(
        opt.number_of_words,
        opt.minimum_entropy,
        opt.strength_count,
        word_list.len(),
    );

    // If user enabled verbose option
    if opt.verbose {
        // print entropy information, but use eprint to only print it
        // to the terminal
        print_entropy(
            number_of_words_to_put_in_passphrase,
            word_list.len(),
            opt.n_passphrases,
        );
    }

    // Now we can (finally) generate and print some number of passphrases
    for _ in 0..opt.n_passphrases {
        let passphrase = generate_a_passphrase(
            number_of_words_to_put_in_passphrase,
            &opt.separator,
            opt.title_case,
            word_list,
        );
        println!("{}", passphrase);
    }
}

/// Convert list_choice string slice into a ListChoice enum. Clap calls this function.
fn parse_list_choice(list_choice: &str) -> Result<ListChoice, String> {
    match list_choice.to_lowercase().as_ref() {
        "l" => Ok(ListChoice::Long),
        "m" => Ok(ListChoice::Medium),
        "e" => Ok(ListChoice::Eff),
        "n" => Ok(ListChoice::Mnemonicode),
        "s" => Ok(ListChoice::Effshort),
        "q" => Ok(ListChoice::Qwerty),
        "a" => Ok(ListChoice::Alpha),
        _ => Err(format!(
            "Inputted list choice '{}' doesn't correspond to an available word list",
            list_choice
        )),
    }
}
