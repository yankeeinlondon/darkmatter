use std::{fs::read_to_string, path::Path, time::Duration};

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
    .expect("unable to load stemming/en.text");

    let en_prose_no_stop = StopWords::parse(
        &en_prose.corpus(), //
        Language::English,
    );

    let de_prose = Words::from_path(
        &Path::new("src/stemming/de.txt"), //
        None,
    )
    .expect("unable to load stemming/en.text");

    let en_prose_no_stop = StopWords::parse(
        &en_prose.corpus(), //
        Language::German,
    )
    .expect("unable to load stemming/de.text")
    .as_prose();

    let de_prose = de_prose.corpus();

    group.bench_function(
        format!("English - {} characters", &en_prose.len()), //
        |b| {
            b.iter(|| {
                black_box({
                    stem(&en_prose, &Language::English).unwrap();
                })
            })
        },
    );

    group.bench_function(
        "German", //
        |b| {
            b.iter(|| {
                black_box({
                    stem(&en_prose, &Language::English).unwrap();
                })
            })
        },
    );
}

criterion_group!(benches, mid_sized);
criterion_main!(benches);
