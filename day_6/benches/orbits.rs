use criterion::{criterion_group, criterion_main, Criterion};

use day_6::orbits;
use day_6::orbits_simple;

fn criterion_benchmark(c: &mut Criterion) {
    let data = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    c.bench_function("orbits::get_checksum", |b| {
        b.iter(|| orbits::get_checksum(data))
    });
    c.bench_function("orbits_simple::get_checksum", |b| {
        b.iter(|| orbits_simple::get_checksum(data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
