use criterion::{criterion_group, criterion_main, Criterion};
use phraze::cli::ListChoice;
use phraze::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Define a Criterion group, just so we can set a sample_size
    let mut group = c.benchmark_group("Generate a passphrase");
    group.sample_size(1200).significance_level(0.1);

    let number_of_words_to_put_in_passphrase = 7;
    let separator = "-";
    let title_case = false;

    group.bench_function("as is", |b| {
        b.iter(|| {
            // include the fetching of the (built-in) list
            // in the benchmark
            let wordlist = fetch_list(ListChoice::Medium);
            generate_a_passphrase(
                number_of_words_to_put_in_passphrase,
                separator,
                title_case,
                wordlist,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
