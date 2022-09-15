use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dm_utils::detect_lang::{detect_language, LanguageOptions};
use lingua::Language;
use std::{fs::read_to_string, time::Duration};

const LANG_SUBSET: [Language; 5] = [
    Language::English,
    Language::French,
    Language::German,
    Language::Spanish,
    Language::Italian,
];

const LANG_SUBSET_LG: [Language; 10] = [
    Language::English,
    Language::French,
    Language::German,
    Language::Spanish,
    Language::Italian,
    Language::Afrikaans,
    Language::Hindi,
    Language::Chinese,
    Language::Dutch,
    Language::Portuguese,
];

const LANG_SUBSET_XL: [Language; 25] = [
    Language::English,
    Language::French,
    Language::German,
    Language::Spanish,
    Language::Italian,
    Language::Afrikaans,
    Language::Hindi,
    Language::Chinese,
    Language::Dutch,
    Language::Portuguese,
    // next 10
    Language::Arabic,
    Language::Armenian,
    Language::Basque,
    Language::Bengali,
    Language::Belarusian,
    Language::Russian,
    Language::Bulgarian,
    Language::Croatian,
    Language::Czech,
    Language::Estonian,
    // last 5
    Language::Finnish,
    Language::Georgian,
    Language::Greek,
    Language::Gujarati,
    Language::Hungarian,
];

const LANG_SUBSET_SM: [Language; 2] = [Language::English, Language::Spanish];

fn sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("Language Detection on Sentence");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(15));
    const DE_TEXT: &str = "Der schnelle braune Fuchs sprang über den faulen Hund";

    group.bench_function(
        "2 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    DE_TEXT,
                    &LanguageOptions::whitelist(&LANG_SUBSET_SM),
                ))
            })
        },
    );

    group.bench_function(
        "5 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    DE_TEXT,
                    &LanguageOptions::whitelist(&LANG_SUBSET),
                ))
            })
        },
    );

    group.bench_function(
        "10 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    DE_TEXT,
                    &LanguageOptions::whitelist(&LANG_SUBSET_LG),
                ))
            })
        },
    );

    group.bench_function(
        "25 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    DE_TEXT,
                    &LanguageOptions::whitelist(&LANG_SUBSET_XL),
                ))
            })
        },
    );

    group.bench_function(
        "25 languages, preloaded lang", //
        |b| {
            let mut options = LanguageOptions::whitelist(&LANG_SUBSET_XL);
            options.eager_loading = true;
            b.iter(|| black_box(detect_language(DE_TEXT, &options)))
        },
    );

    group.bench_function(
        "all languages", //
        |b| b.iter(|| black_box(detect_language(DE_TEXT, &LanguageOptions::default()))),
    );
}

fn paragraph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Language Detection on Paragraph");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(20));

    let paragraph =
        read_to_string("benches/paragraph.txt").expect("Couldn't load paragraph.txt fixture");

    group.bench_function(
        "2 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    &paragraph,
                    &LanguageOptions::whitelist(&LANG_SUBSET_SM),
                ))
            })
        },
    );

    group.bench_function(
        "5 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    &paragraph,
                    &LanguageOptions::whitelist(&LANG_SUBSET),
                ))
            })
        },
    );

    group.bench_function(
        "10 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    &paragraph,
                    &LanguageOptions::whitelist(&LANG_SUBSET_LG),
                ))
            })
        },
    );

    group.bench_function(
        "25 languages", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    &paragraph,
                    &LanguageOptions::whitelist(&LANG_SUBSET_XL),
                ))
            })
        },
    );

    group.bench_function(
        "all languages", //
        |b| b.iter(|| black_box(detect_language(&paragraph, &LanguageOptions::default()))),
    );

    group.bench_function(
        "all languages, no threshold", //
        |b| {
            b.iter(|| {
                black_box(detect_language(
                    &paragraph,
                    &LanguageOptions::all_with_confidence(None),
                ))
            })
        },
    );
}

criterion_group!(benches, paragraph, sentence);
criterion_main!(benches);
