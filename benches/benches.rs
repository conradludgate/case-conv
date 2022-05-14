use case_conv::{to_lowercase, to_uppercase};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

static ASCII: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
static UNICODE: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. DÜis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn lowercase(c: &mut Criterion) {
    let mut g = c.benchmark_group("lowercase");

    g.bench_function("ascii_std", |b| b.iter(|| black_box(ASCII.to_lowercase())));
    g.bench_function("ascii", |b| b.iter(|| black_box(to_lowercase(ASCII))));
    g.bench_function("unicode_std", |b| {
        b.iter(|| black_box(UNICODE.to_lowercase()))
    });
    g.bench_function("unicode", |b| b.iter(|| black_box(to_lowercase(UNICODE))));

    g.finish()
}

pub fn uppercase(c: &mut Criterion) {
    let mut g = c.benchmark_group("uppercase");

    g.bench_function("ascii_std", |b| b.iter(|| black_box(ASCII.to_uppercase())));
    g.bench_function("ascii", |b| b.iter(|| black_box(to_uppercase(ASCII))));
    g.bench_function("unicode_std", |b| {
        b.iter(|| black_box(UNICODE.to_uppercase()))
    });
    g.bench_function("unicode", |b| b.iter(|| black_box(to_uppercase(UNICODE))));

    g.finish()
}

criterion_group!(benches, lowercase, uppercase);
criterion_main!(benches);
