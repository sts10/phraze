use clap::Parser;
use phraze::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

/// Generate random passphrases
#[derive(Parser, Debug)]
#[clap(version, name = "phraze")]
struct Args {
    /// Strengthen your passphrase the easy way: Each flag increases minimum entropy by 20 bits (above the default of
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

    /// Provide a text file with a list of words to randomly generate passphrase from
    #[clap(short = 'c', long = "custom-list", conflicts_with = "list_choice")]
    custom_list_file_path: Option<PathBuf>,

    /// Use Title Case for words in generated usernames
    #[clap(short = 't', long = "title-case")]
    title_case: bool,

    /// Print estimated entropy of generated passphrase, in bits, along with the passphrase itself
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() {
    let opt = Args::parse();

    if opt.custom_list_file_path.is_some() && opt.separator == "" && !opt.title_case {
        panic!("Must use a separator or title case when using a custom word list");
    }

    // Fetch requested word list
    let custom_list = match opt.custom_list_file_path {
        Some(custom_list_file_path) => Some(read_in_custom_list(&custom_list_file_path)),
        None => None,
    };
    let built_in_list = fetch_list(opt.list_choice);

    // Unfortunately, since I can't have just one list variable yet,
    // I need to do this to get the legnth of the list
    let list_length = match custom_list {
        Some(ref custom_list) => custom_list.len(),
        None => built_in_list.len(),
    };

    // Since user can define a minimum entropy, we might have to do a little math to
    // figure out how many words we need to include in this passphrase.
    let number_of_words_to_put_in_passphrase = calculate_number_words_needed(
        opt.number_of_words,
        opt.minimum_entropy,
        opt.strength_count,
        list_length,
    );

    // If user enabled verbose option
    if opt.verbose {
        // print entropy information, but use eprint to only print it
        // to the terminal
        print_entropy(
            number_of_words_to_put_in_passphrase,
            list_length,
            opt.n_passphrases,
        );
    }

    for _ in 0..opt.n_passphrases {
        // Generate and print passphrase
        let passphrase = match custom_list {
            Some(ref custom_list) => generate_passphrase(
                number_of_words_to_put_in_passphrase,
                &opt.separator,
                opt.title_case,
                custom_list,
            ),
            None => generate_passphrase(
                number_of_words_to_put_in_passphrase,
                &opt.separator,
                opt.title_case,
                built_in_list,
            ),
        };
        println!("{}", passphrase);
    }
}

fn print_entropy(number_of_words: usize, list_length: usize, n_passphrases: usize) {
    let passphrase_entropy = (list_length as f64).log2() * number_of_words as f64;
    if n_passphrases == 1 {
        eprintln!(
            "Passphrase has an estimated {:.2} bits of entropy.",
            passphrase_entropy
        );
    } else {
        eprintln!(
            "Each passphrase has an estimated {:.2} bits of entropy.",
            passphrase_entropy
        );
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

fn read_in_custom_list(file_path: &Path) -> Vec<String> {
    let file_input: Vec<String> = match read_by_line(file_path.to_path_buf()) {
        Ok(r) => r,
        Err(e) => panic!("Error reading word list file: {}", e),
    };
    let mut word_list: Vec<String> = vec![];
    for line in file_input {
        if line.to_string().trim() != "" {
            word_list.push(line.to_string().trim().to_string());
        }
    }
    word_list
}

fn read_by_line<T: FromStr>(file_path: PathBuf) -> io::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut vec = Vec::new();
    let f = match File::open(file_path) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(e) => panic!("Error parsing line from file: {:?}", e),
        }
    }
    Ok(vec)
}
