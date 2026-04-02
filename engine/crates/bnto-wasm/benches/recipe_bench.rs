// Benchmarks -- recipe-level composite pipelines and PNG compression.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use bnto_core::{
    NoopContext, PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline,
};

// --- Test fixtures ---
static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static LARGE_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/large.png");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

// --- Helpers ---

fn real_registry() -> bnto_core::NodeRegistry {
    bnto_engine::create_default_registry()
}

fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: mime.to_string(),
        metadata: serde_json::Map::new(),
    }
}

fn fake_now() -> u64 {
    1000
}

// --- Recipe-Level Benchmarks ---

fn bench_recipes(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

    let compress_recipe = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-compress", "type": "group",
                "nodes": [{
                    "id": "compress-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "compress-image", "type": "image-compress",
                        "parameters": { "quality": 80 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("recipe/compress-images/1-file", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(
                &compress_recipe,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });

    // Batch scaling: measure how performance changes with file count.
    let mut batch_group = c.benchmark_group("recipe/compress-images/batch");
    for count in [1, 5, 10] {
        batch_group.throughput(Throughput::Elements(count as u64));
        batch_group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let files: Vec<PipelineFile> = (0..count)
                    .map(|i| file(&format!("photo_{}.jpg", i), SMALL_JPEG, "image/jpeg"))
                    .collect();
                execute_pipeline(
                    &compress_recipe,
                    files,
                    &registry,
                    &reporter,
                    &NoopContext,
                    fake_now,
                )
                .unwrap();
            })
        });
    }
    batch_group.finish();

    let clean_recipe = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "csv-cleaner", "type": "group",
                "nodes": [{
                    "id": "clean", "type": "spreadsheet-clean",
                    "parameters": { "trimWhitespace": true, "removeEmptyRows": true, "removeDuplicates": true }
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("recipe/clean-csv/1-file", |b| {
        b.iter(|| {
            let files = vec![file("data.csv", MESSY_CSV, "text/csv")];
            execute_pipeline(
                &clean_recipe,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });

    let rename_recipe = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-rename", "type": "group",
                "nodes": [{
                    "id": "rename-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "rename-file", "type": "file-rename",
                        "parameters": { "prefix": "renamed-" }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("recipe/rename-files/1-file", |b| {
        b.iter(|| {
            let files = vec![file("document.txt", b"hello world", "text/plain")];
            execute_pipeline(
                &rename_recipe,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });
}

// --- PNG Compression Benchmarks ---
//
// PNG compression uses quantizr (median cut + Floyd-Steinberg dithering) to
// reduce 24-bit truecolor to 8-bit indexed palette before DEFLATE. Tracks
// both speed and output size for regression detection.

fn bench_png_compression(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

    let compress_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image-compress" },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image-compress/png/large", |b| {
        b.iter(|| {
            let files = vec![file("photo.png", LARGE_PNG, "image/png")];
            execute_pipeline(
                &compress_def,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap()
        })
    });

    // One-shot: Print output size for regression tracking.
    let files = vec![file("photo.png", LARGE_PNG, "image/png")];
    let result = execute_pipeline(
        &compress_def,
        files,
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();
    if let Some(output_file) = result.files.first() {
        let input_kb = LARGE_PNG.len() / 1024;
        let output_kb = output_file.data.len() / 1024;
        let reduction_pct = (1.0 - output_file.data.len() as f64 / LARGE_PNG.len() as f64) * 100.0;
        eprintln!(
            "\n  PNG compression: {} KB -> {} KB ({:.1}% reduction)\n",
            input_kb, output_kb, reduction_pct
        );
    }
}

criterion_group!(benches, bench_recipes, bench_png_compression);
criterion_main!(benches);
