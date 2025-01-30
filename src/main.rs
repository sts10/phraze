use crate::cli::Args;
use crate::file_reader::read_in_custom_list;
use clap::Parser;
use phraze::*;

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
