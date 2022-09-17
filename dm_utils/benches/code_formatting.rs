use std::{path::Path, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dm_utils::code_highlighting::CodeBlock;
use std::fs::read_to_string;

pub struct TestData {
    pub javascript_small: String,
    pub javascript_large: String,
    pub typescript_small: String,
    pub typescript_large: String,
    pub rust_small: String,
    pub rust_large: String,
    pub css_small: String,
    pub css_large: String,
    pub html_small: String,
    pub html_large: String,
}
impl TestData {
    pub fn new() -> Self {
        TestData {
            javascript_small: read_to_string(Path::new(
                "benches/fixtures/small.js",
            ))
            .unwrap(),
            javascript_large: read_to_string(Path::new(
                "benches/fixtures/small.js",
            ))
            .unwrap(),
            typescript_small: read_to_string(Path::new(
                "benches/fixtures/small.ts",
            ))
            .unwrap(),
            typescript_large: read_to_string(Path::new(
                "benches/fixtures/small.ts",
            ))
            .unwrap(),
            rust_small: read_to_string(Path::new("benches/fixtures/small.rs"))
                .unwrap(),
            rust_large: read_to_string(Path::new("benches/fixtures/small.rs"))
                .unwrap(),
            css_small: read_to_string(Path::new("benches/fixtures/small.css"))
                .unwrap(),
            css_large: read_to_string(Path::new("benches/fixtures/small.css"))
                .unwrap(),
            html_small: read_to_string(Path::new(
                "benches/fixtures/small.html",
            ))
            .unwrap(),
            html_large: read_to_string(Path::new(
                "benches/fixtures/small.html",
            ))
            .unwrap(),
        }
    }
}

fn code_formatting_by_block_size_and_language(c: &mut Criterion) {
    // HTML Highlighting
    let data = TestData::new();
    let mut group = c.benchmark_group(
        "Code highlighting by size, language, and output format",
    );
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(125);
    group.measurement_time(Duration::from_secs(5));

    // JS
    group.bench_function("js/small/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_small, "javascript")
                    .as_html(),
            )
        })
    });
    group.bench_function("js/large/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_large, "javascript")
                    .as_html(),
            )
        })
    });

    // TS
    group.bench_function("ts/small/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_small, "typescript")
                    .as_html(),
            )
        })
    });
    group.bench_function("ts/large/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_large, "typescript")
                    .as_html(),
            )
        })
    });

    // RUST
    group.bench_function("rust/small/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_small, "rust").as_html(),
            )
        })
    });
    group.bench_function("rust/large/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_large, "rust").as_html(),
            )
        })
    });

    // CSS
    group.bench_function("css/small/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_small, "css").as_html(),
            )
        })
    });
    group.bench_function("css/large/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_large, "css").as_html(),
            )
        })
    });

    // HTML
    group.bench_function("html/small/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_small, "html").as_html(),
            )
        })
    });
    group.bench_function("html/large/html", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_large, "html").as_html(),
            )
        })
    });

    // CONSOLE OUTPUT

    // JS
    group.bench_function("js/small/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_small, "javascript")
                    .for_console(),
            )
        })
    });
    group.bench_function("js/large/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_large, "javascript")
                    .for_console(),
            )
        })
    });

    // TS
    group.bench_function("ts/small/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_small, "typescript")
                    .for_console(),
            )
        })
    });
    group.bench_function("ts/large/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_large, "typescript")
                    .for_console(),
            )
        })
    });

    // RUST
    group.bench_function("rust/small/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_small, "rust")
                    .for_console(),
            )
        })
    });
    group.bench_function("rust/large/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_large, "rust")
                    .for_console(),
            )
        })
    });

    // CSS
    group.bench_function("css/small/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_small, "css").for_console(),
            )
        })
    });
    group.bench_function("css/large/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_large, "css").for_console(),
            )
        })
    });

    // HTML
    group.bench_function("html/small/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_small, "html")
                    .for_console(),
            )
        })
    });
    group.bench_function("html/large/console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_large, "html")
                    .for_console(),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, code_formatting_by_block_size_and_language);
criterion_main!(benches);
