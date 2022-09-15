use std::{path::Path, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dm_utils::{stemming::stem, words::Words, StopWords};
use lingua::Language;

fn mid_sized(c: &mut Criterion) {
    let mut group = c.benchmark_group("Stemming with mid-sized word set");

    group.warm_up_time(Duration::from_secs(2));
    group.sample_size(100);
    // group.measurement_time(Duration::from_secs(15));

    let en_prose = Words::from_path(
        &Path::new("src/stemming/en.txt"), //
        None,
    )
    .expect("unable to load stemming/en.txt");

    let en_prose_no_stop = StopWords::parse(
        &en_prose.corpus(), //
        Language::English,
    )
    .expect("problems removing stop words from english prose");

    let de_prose = Words::from_path(
        &Path::new("src/stemming/de.txt"), //
        None,
    )
    .expect("unable to load stemming/de.txt");

    let de_prose_no_stop = StopWords::parse(
        &de_prose.corpus(), //
        Language::German,
    )
    .expect("unable to load stemming/de.txt");

    group.bench_function(
        format!("English - {} characters", &en_prose.len()), //
        |b| {
            b.iter(|| {
                black_box({
                    stem(&en_prose.as_tokens(), &Language::English).unwrap();
                })
            })
        },
    );

    group.bench_function(
        format!(
            "English w/o Stop Words - {} characters",
            &en_prose_no_stop.len()
        ), //
        |b| {
            b.iter(|| {
                black_box({
                    stem(&en_prose_no_stop.as_tokens(), &Language::English).unwrap();
                })
            })
        },
    );

    group.bench_function(
        format!("German - {} characters", &de_prose.len()), //
        |b| {
            b.iter(|| {
                black_box({
                    stem(&de_prose.as_tokens(), &Language::German).unwrap();
                })
            })
        },
    );

    group.bench_function(
        format!(
            "German w/o Stop Words - {} characters",
            &de_prose_no_stop.len()
        ), //
        |b| {
            b.iter(|| {
                black_box({
                    stem(&de_prose_no_stop.as_tokens(), &Language::German).unwrap();
                })
            })
        },
    );
}

criterion_group!(benches, mid_sized);
criterion_main!(benches);
