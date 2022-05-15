use std::{fs::File, io::Read};

use case_conv::{to_lowercase, to_uppercase};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn conv(c: &mut Criterion) {
    let mut ascii = String::new();
    let mut unicode = String::new();

    let mut ascii_file = File::open("benches/macbeth.ascii.txt").unwrap();
    ascii_file.read_to_string(&mut ascii).unwrap();

    let mut unicode_file = File::open("benches/macbeth.unicode.txt").unwrap();
    unicode_file.read_to_string(&mut unicode).unwrap();

    {
        let mut g = c.benchmark_group("lowercase");

        g.bench_function("ascii", |b| b.iter(|| to_lowercase(black_box(&ascii))));
        g.bench_function("unicode", |b| b.iter(|| to_lowercase(black_box(&unicode))));

        g.bench_function("ascii_std", |b| b.iter(|| black_box(&ascii).to_lowercase()));
        g.bench_function("unicode_std", |b| {
            b.iter(|| black_box(&unicode).to_lowercase())
        });
    }

    {
        let mut g = c.benchmark_group("uppercase");

        g.bench_function("ascii", |b| b.iter(|| to_uppercase(black_box(&ascii))));
        g.bench_function("unicode", |b| b.iter(|| to_uppercase(black_box(&unicode))));

        g.bench_function("ascii_std", |b| b.iter(|| black_box(&ascii).to_uppercase()));
        g.bench_function("unicode_std", |b| {
            b.iter(|| black_box(&unicode).to_uppercase())
        });
    }
}

criterion_group!(benches, conv);
criterion_main!(benches);
