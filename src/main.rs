use clap::Parser;
use phraze::*;

/// Generate random passphrases
#[derive(Parser, Debug)]
#[clap(version, name = "phraze")]
struct Args {
    /// Set how many words in generated passphrase
    #[clap(short = 'w', long = "words", conflicts_with = "minimum_entropy")]
    number_of_words: Option<u8>,

    /// Set minimum amount of entropy for generated passprase
    #[clap(short = 'e', long = "entropy", conflicts_with = "number_of_words")]
    minimum_entropy: Option<usize>,

    /// Word separator. Can accept single quotes around the separator.
    ///
    /// There are special values that will trigger generated separators:
    ///
    /// _n: separators will be random numbers
    ///
    /// _s: separators will be random symbols
    ///
    /// _b: separators will be both random numbers and symbols
    #[clap(short = 's', long = "sep", default_value = "-")]
    separator: String,

    /// Choose a word list to use.
    ///
    /// Options:
    ///
    /// m: Orchard Street Medium List (7,776 words) [DEFAULT]
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
    /// distance on QWERTY keyboard layouts.
    ///
    /// a: Orchard Street Alpha list (1,296 words). Optimized to minimize travel distance on an
    /// alphabetical keyboard layout
    #[clap(short = 'l', long = "list")]
    list_choice: Option<char>,

    /// Use Title Case for words in generated usernames
    #[clap(short = 't', long = "title-case")]
    title_case: bool,
}

fn main() {
    let opt = Args::parse();
    // Attempt to parse chosen word list
    match parse_list(opt.list_choice) {
        Ok(list_to_use) => println!(
            "{}",
            generate_passphrase(
                opt.number_of_words,
                opt.minimum_entropy,
                &opt.separator,
                opt.title_case,
                list_to_use
            )
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Convert list_choice character into List enum
fn parse_list(list_choice: Option<char>) -> Result<List, String> {
    match list_choice {
        Some(c) => match c.to_ascii_lowercase() {
            'l' => Ok(List::Long),
            'm' => Ok(List::Medium),
            'e' => Ok(List::Eff),
            'n' => Ok(List::Mnemonicode),
            's' => Ok(List::Effshort),
            'q' => Ok(List::Qwerty),
            'a' => Ok(List::Alpha),
            _ => Err(format!("Unable to parse word list choice '{}'", c)),
        },
        None => Ok(List::Medium),
    }
}
