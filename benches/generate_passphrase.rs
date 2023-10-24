use phraze::*;
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(sample_size = 20, sample_count = 1000)]
fn as_is() -> String {
    let number_of_words = None;
    let minimum_entropy = Some(80);
    let separator = "-";
    let title_case = false;
    let list = List::Medium;
    generate_passphrase(
        number_of_words,
        minimum_entropy,
        separator,
        title_case,
        list,
    )
}
