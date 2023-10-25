use clap::Parser;
use phraze::*;

/// Generate random passphrases
#[derive(Parser, Debug)]
#[clap(version, name = "phraze")]
struct Args {
    /// Set minimum amount of entropy for generated passphrase. If neither minimum_entropy or
    /// number_of_words is specified, Phraze will default to an 80-bit minimum.
    #[clap(
        short = 'e',
        long = "minimum_entropy",
        conflicts_with = "number_of_words"
    )]
    minimum_entropy: Option<usize>,

    /// Set how many words in generated passphrase. If neither number_of_words or
    /// minimum_entropy is specified, Phraze will default to an 80-bit minimum.
    #[clap(short = 'w', long = "words", conflicts_with = "minimum_entropy")]
    number_of_words: Option<usize>,

    /// Word separator. Can accept single quotes around the separator.
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
    /// distance on QWERTY keyboard layouts.
    ///
    /// a: Orchard Street Alpha list (1,296 words). Optimized to minimize travel distance on an
    /// alphabetical keyboard layout
    #[clap(short = 'l', long = "list", value_parser=parse_list_choice, default_value="m")]
    list_choice: List,

    /// Use Title Case for words in generated usernames
    #[clap(short = 't', long = "title-case")]
    title_case: bool,
}

fn main() {
    let opt = Args::parse();

    println!(
        "{}",
        generate_passphrase(
            opt.number_of_words,
            opt.minimum_entropy,
            &opt.separator,
            opt.title_case,
            opt.list_choice,
        )
    );
}

/// Convert list_choice string slice into a List enum. Clap calls this function.
fn parse_list_choice(list_choice: &str) -> Result<List, String> {
    match list_choice.to_lowercase().as_ref() {
        "l" => Ok(List::Long),
        "m" => Ok(List::Medium),
        "e" => Ok(List::Eff),
        "n" => Ok(List::Mnemonicode),
        "s" => Ok(List::Effshort),
        "q" => Ok(List::Qwerty),
        "a" => Ok(List::Alpha),
        _ => Err(format!(
            "Inputted list choice '{}' doesn't correspond to an available word list",
            list_choice
        )),
    }
}
