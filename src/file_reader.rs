//! A couple functions for reading in custom word list files

use crate::unicode_normalization_check::uniform_unicode_normalization;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

/// Read text file into a `Vec<String>`. Also trims whitespace, avoids adding blank strings,
/// sorts, de-duplicates, and checks for uniform Unicode normalization.
pub fn read_in_custom_list(file_path: &Path) -> Result<Vec<String>, String> {
    let file_input: Vec<String> = match read_by_line(file_path.to_path_buf()) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error reading word list file: {}", e)),
    };
    let mut word_list: Vec<String> = vec![];
    for line in file_input {
        // Don't add blank lines or lines made up purely of whitespace
        if line.trim() != "" {
            // Remove any starting or trailing whitespace before adding word to list
            word_list.push(line.trim().to_string());
        }
    }
    // Remove any duplicate words, since duplicate words would undermine entropy estimates.
    word_list.sort();
    word_list.dedup();
    if !uniform_unicode_normalization(&word_list) {
        eprintln!("WARNING: Custom word list has multiple Unicode normalizations. Consider normalizing the Unicode of all words on the list before making a passphrase.");
    }
    Ok(word_list)
}

/// Generatic function that reads a file in, line by line.
/// Not sure if all of this is necessary, but it gets the job done.
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
