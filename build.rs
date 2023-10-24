use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation

fn words(mut f_dest: &File, const_name: &str, fname_src: &str, list_size: usize) {
    // Declare a new Rust constant
    // make it an array of slices of the exact size of this word list
    // to maximize effiency.
    write!(f_dest, "const {const_name}: &[&str; {list_size}] = &[").unwrap();

    // Read words in and add them to this array
    let f_src = BufReader::new(File::open(fname_src).unwrap());
    for line in f_src.lines() {
        f_dest.write_all(b"\"").unwrap();

        f_dest.write_all(line.unwrap().as_bytes()).unwrap();

        f_dest.write_all(b"\",").unwrap();
    }

    // Close array syntax
    f_dest.write_all(b"];").unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wordlists.rs");
    let f = File::create(dest_path).unwrap();

    words(&f, "WL_LONG", "word-lists/orchard-street-long.txt", 17576);
    words(
        &f,
        "WL_MEDIUM",
        "word-lists/orchard-street-medium.txt",
        7776,
    );
    words(
        &f,
        "WL_QWERTY",
        "word-lists/orchard-street-qwerty.txt",
        1296,
    );
    words(&f, "WL_ALPHA", "word-lists/orchard-street-alpha.txt", 1296);
    words(&f, "WL_EFF", "word-lists/eff-long.txt", 7776);
    words(&f, "WL_EFFSHORT", "word-lists/eff-short-1.txt", 1296);
    words(&f, "WL_MNEMONICODE", "word-lists/mnemonicode.txt", 1633);
}
