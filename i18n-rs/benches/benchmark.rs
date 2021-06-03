use criterion::{black_box, criterion_group, criterion_main, Criterion};

use simple_i18n::{i18n, lang};

fn i18n_loose() {
    black_box(i18n!("name"; loose));
}
fn i18n_strict() {
    black_box(i18n!("name"));
}
fn change_lang() {
    lang!("en-us");
}

fn criterion_benchmark(c: &mut Criterion) {
    lang!("en-us");

    let mut strict_group = c.benchmark_group("strict contrast");
    strict_group.bench_function("no strict", |b| b.iter(|| i18n_loose()));
    strict_group.bench_function("strict", |b| b.iter(|| i18n_strict()));
    strict_group.finish();

    c.bench_function("change_lang", |b| b.iter(|| change_lang()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
