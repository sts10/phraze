use clap::Parser;
use phraze::*;

/// Generate random passphrases
#[derive(Parser, Debug)]
#[clap(version, name = "phraze")]
struct Args {
    /// Strengthen your passphrase the easy way: Each flag increases minimum entropy by 20 bits (above the default of
    /// 80 bits).
    #[clap(short = 'S', long = "strong", conflicts_with = "number_of_words", action = clap::ArgAction::Count)]
    strong_setting: u8,

    /// Set minimum amount of entropy in bits for generated passphrase. If neither minimum_entropy or
    /// number_of_words is specified, Phraze will default to an 80-bit minimum.
    #[clap(
        short = 'e',
        long = "minimum-entropy",
        conflicts_with = "number_of_words"
    )]
    minimum_entropy: Option<usize>,

    /// Set exactly how many words to use in generated passphrase. If neither number_of_words or
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

    // If user is using 'strong' setting, we need to parse some settings to determine
    // what minimum_entropy and number_of_words_desired to pass to the
    // generate_passphrase function.
    let (minimum_entropy, number_of_words_desired) = if opt.strong_setting > 0 {
        (
            determine_minimum_entropy_conservatively(opt.strong_setting, opt.minimum_entropy),
            None,
        )
    } else {
        (opt.minimum_entropy, opt.number_of_words)
    };

    // Generate and print passphrase
    println!(
        "{}",
        generate_passphrase(
            number_of_words_desired,
            minimum_entropy,
            &opt.separator,
            opt.title_case,
            opt.list_choice,
        )
    );
}

/// If user is using Ss to request a stronger passphrase, we need to do a little math to convert
/// that into a number of bits.
/// If user inputs a number of Ss _and_ a specific minimum entropy in bits, using `-e`, we'll take
/// which ever is HIGHER.
fn determine_minimum_entropy_conservatively(
    number_of_s_s: u8,
    requested_minimum_entropy: Option<usize>,
) -> Option<usize> {
    // Convert number of Ss into an actual minimum entropy in bits
    // We start with 80 bits and add 20 bits for every S the user inputs.
    // Here's how that looks in a formula:
    let minimum_entropy_from_strength_s_s: usize = (80 + number_of_s_s * 20).into();

    match requested_minimum_entropy {
        Some(requested_minimum_entropy) => {
            // Take whichever setting leads to a HIGHER minimum entropy
            if requested_minimum_entropy > minimum_entropy_from_strength_s_s {
                Some(requested_minimum_entropy)
            } else {
                Some(minimum_entropy_from_strength_s_s)
            }
        }
        // No minimum entropy requested in bits. Just go with what the Ss tell us.
        None => Some(minimum_entropy_from_strength_s_s),
    }
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
