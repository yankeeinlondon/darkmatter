use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dm_utils::hasher::hash;
use rand::distributions::{Alphanumeric, DistString};

const TINY_HASH: usize = 32;
const SMALL_HASH: usize = 250;
const MEDIUM_HASH: usize = 2048;
const LARGE_HASH: usize = 8096;

fn hash_by_content_size(c: &mut Criterion) {
    let tiny_content: String =
        Alphanumeric.sample_string(&mut rand::thread_rng(), TINY_HASH);
    let small_content: String =
        Alphanumeric.sample_string(&mut rand::thread_rng(), SMALL_HASH);
    let medium_content: String =
        Alphanumeric.sample_string(&mut rand::thread_rng(), MEDIUM_HASH);
    let large_content: String =
        Alphanumeric.sample_string(&mut rand::thread_rng(), LARGE_HASH);

    let mut group = c.benchmark_group("Hashing");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(250);
    group.measurement_time(Duration::from_secs(3));

    group.bench_function(
        "tiny", //
        |b| b.iter(|| black_box(hash(&tiny_content, None))),
    );

    group.bench_function(
        "small", //
        |b| b.iter(|| black_box(hash(&small_content, None))),
    );

    group.bench_function(
        "medium", //
        |b| b.iter(|| black_box(hash(&medium_content, None))),
    );

    group.bench_function(
        "large", //
        |b| b.iter(|| black_box(hash(&large_content, None))),
    );

    group.finish();
}

criterion_group!(benches, hash_by_content_size);
criterion_main!(benches);
