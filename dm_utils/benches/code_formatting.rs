use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dm_utils::code_highlighting::CodeBlock;
use std::fs::read_to_string;
use std::{path::Path, time::Duration};

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
    let mut group = c.benchmark_group("Code Highlighting");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(125);
    group.measurement_time(Duration::from_secs(5));

    // JS
    group.bench_function("JS/small -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_small, "javascript")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });
    group.bench_function("JS/large -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_large, "javascript")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });

    // TS
    group.bench_function("TS/small -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_small, "typescript")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });
    group.bench_function("TS/large -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_large, "typescript")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });

    // RUST
    group.bench_function("rust/small -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_small, "rust")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });
    group.bench_function("rust/large -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_large, "rust")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });

    // CSS
    group.bench_function("css/small -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_small, "css")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });
    group.bench_function("css/large -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_large, "css")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });

    // HTML
    group.bench_function("HTML/small -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_small, "html")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });
    group.bench_function("HTML/large -> HTML", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_large, "html")
                    .unwrap()
                    .as_html(&true),
            )
        })
    });

    // CONSOLE OUTPUT

    // JS
    group.bench_function("JS/small -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_small, "javascript")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });
    group.bench_function("JS/large -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.javascript_large, "javascript")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });

    // TS
    group.bench_function("TS/small -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_small, "typescript")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });
    group.bench_function("TS/large -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.typescript_large, "typescript")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });

    // RUST
    group.bench_function("rust/small -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_small, "rust")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });
    group.bench_function("rust/large -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.rust_large, "rust")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });

    // CSS
    group.bench_function("css/small -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_small, "css")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });
    group.bench_function("css/large -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.css_large, "css")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });

    // HTML
    group.bench_function("HTML/small -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_small, "html")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });
    group.bench_function("HTML/large -> Console", |b| {
        b.iter(|| {
            black_box(
                CodeBlock::new_with_lang(&data.html_large, "html")
                    .unwrap()
                    .as_escaped_console(&true),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, code_formatting_by_block_size_and_language);
criterion_main!(benches);
