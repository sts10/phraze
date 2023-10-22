use clap::Parser;
use phraze::*;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/wordlists.rs"));

/// Generate random passphrases
#[derive(Parser, Debug)]
#[clap(version, name = "phraze")]
struct Args {
    /// Set how many words in generated passphrase
    #[clap(short = 'w', long = "words", default_value = "7")]
    number_of_words: u8,

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

    /// Choose a word list to use. Options:
    ///
    /// m -> Orchard Street Medium List (7,776 words)
    ///
    /// l -> Orchard Street Long List (17,576 words)
    ///
    /// q -> Orchard Street QWERTY list (1,296 words). Optimized to minimize travel
    /// distance on QWERTY keyboard layouts.
    ///
    /// a -> Orchard Street Alpha list (1,296 words). Optimized to minimize travel
    /// distance on alphabetical keyboard layouts.
    #[clap(short = 'l', long = "list")]
    list_choice: Option<char>,

    /// Use Title Case for words in generated usernames
    #[clap(short = 't', long = "title-case")]
    title_case: bool,
}

fn main() {
    let opt = Args::parse();
    let list_to_use = parse_list(opt.list_choice);
    println!(
        "{}",
        generate_passphrase(
            opt.number_of_words,
            &opt.separator,
            opt.title_case,
            list_to_use
        )
    );
}

/// Parse which word list to use. Thanks to build script file (build.rs),
/// we have access to the word lists as environmental variables.
/// See: https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn parse_list(list_choice: Option<char>) -> &'static [&'static str] {
    match list_choice {
        Some(c) => match c.to_ascii_lowercase() {
            'l' => WL_LONG,
            'm' => WL_MEDIUM,
            'q' => WL_QWERTY,
            'a' => WL_ALPHA,
            _ => panic!("Unknown list"),
        },
        // If none given, default to the Medium list
        None => WL_MEDIUM,
    }
}
