use criterion::{criterion_group, criterion_main, Criterion};
use phraze::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Define a Criterion group, just so we can set a sample_size
    let mut group = c.benchmark_group("Generate a passphrase");
    group.sample_size(1200).significance_level(0.1);

    let number_of_words = None;
    let minimum_entropy = Some(80);
    let separator = "-";
    let title_case = false;
    let list_choice = List::Medium;

    group.bench_function("as is", |b| {
        b.iter(|| {
            generate_passphrase(
                number_of_words,
                minimum_entropy,
                separator,
                title_case,
                list_choice,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
