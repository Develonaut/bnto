// =============================================================================
// Pipeline Benchmarks — Track Node & Recipe Performance Over Time
// =============================================================================
//
// WHAT ARE THESE BENCHMARKS?
// These use the Criterion benchmarking framework to measure how fast our
// engine processes files. Criterion runs each benchmark many times,
// computes statistics, and reports whether performance has improved or
// regressed compared to the last run.
//
// WHY BENCHMARK AT THE ENGINE LEVEL?
// The engine is the hot path — every user-facing action (compress, resize,
// clean CSV) flows through these functions. If a code change accidentally
// makes compression 2x slower, these benchmarks will catch it.
//
// HOW TO RUN:
//   cd engine
//   cargo bench
//
// The first run establishes a baseline. Subsequent runs compare against it.
// Results are saved in `engine/target/criterion/`.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use bnto_core::{
    NodeRegistry, PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline,
};

// --- Test fixtures ---
// We embed real files for realistic benchmarks.
static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

// --- Helpers ---

/// Build the production registry with all 6 real processors.
fn real_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    registry.register(
        "image:compress",
        Box::new(bnto_image::CompressImages::new()),
    );
    registry.register("image:resize", Box::new(bnto_image::ResizeImages::new()));
    registry.register(
        "image:convert",
        Box::new(bnto_image::ConvertImageFormat::new()),
    );
    registry.register("spreadsheet:clean", Box::new(bnto_csv::CleanCsv::new()));
    registry.register(
        "spreadsheet:rename",
        Box::new(bnto_csv::RenameCsvColumns::new()),
    );
    registry.register(
        "file-system:rename",
        Box::new(bnto_file::RenameFiles::new()),
    );
    registry
}

/// Parse a JSON string into a PipelineDefinition.
fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

/// Create a PipelineFile.
fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: mime.to_string(),
    }
}

/// Deterministic clock for benchmarks (avoids system clock overhead).
fn fake_now() -> u64 {
    1000
}

// =============================================================================
// Node-Level Benchmarks — Individual Processor Performance
// =============================================================================

fn bench_individual_nodes(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

    // --- image:compress ---
    // Simple pipeline: input → compress → output
    let compress_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress", "quality": 80 } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image:compress/jpeg", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(&compress_def, files, &registry, &reporter, fake_now).unwrap();
        })
    });

    // --- image:resize ---
    let resize_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "resize", "type": "image", "parameters": { "operation": "resize", "width": 100 } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image:resize/jpeg", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(&resize_def, files, &registry, &reporter, fake_now).unwrap();
        })
    });

    // --- image:convert ---
    let convert_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "convert", "type": "image", "parameters": { "operation": "convert", "format": "png", "quality": 80 } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image:convert/jpeg_to_png", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(&convert_def, files, &registry, &reporter, fake_now).unwrap();
        })
    });

    // --- spreadsheet:clean ---
    let clean_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "clean", "type": "spreadsheet", "parameters": { "operation": "clean", "trimWhitespace": true, "removeEmptyRows": true, "removeDuplicates": true } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/spreadsheet:clean/csv", |b| {
        b.iter(|| {
            let files = vec![file("data.csv", MESSY_CSV, "text/csv")];
            execute_pipeline(&clean_def, files, &registry, &reporter, fake_now).unwrap();
        })
    });

    // --- file-system:rename ---
    let rename_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "rename", "type": "file-system", "parameters": { "operation": "rename", "prefix": "renamed-" } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/file-system:rename/txt", |b| {
        b.iter(|| {
            let files = vec![file("document.txt", b"hello world", "text/plain")];
            execute_pipeline(&rename_def, files, &registry, &reporter, fake_now).unwrap();
        })
    });
}

// =============================================================================
// Recipe-Level Benchmarks — Full Composite Recipe Performance
// =============================================================================

fn bench_recipes(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

    // --- Compress Images (full composite recipe) ---
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
                        "id": "compress-image", "type": "image",
                        "parameters": { "operation": "compress", "quality": 80 }
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
            execute_pipeline(&compress_recipe, files, &registry, &reporter, fake_now).unwrap();
        })
    });

    // --- Batch scaling: measure how performance changes with file count ---
    let mut batch_group = c.benchmark_group("recipe/compress-images/batch");
    for count in [1, 5, 10] {
        batch_group.throughput(Throughput::Elements(count as u64));
        batch_group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let files: Vec<PipelineFile> = (0..count)
                    .map(|i| file(&format!("photo_{}.jpg", i), SMALL_JPEG, "image/jpeg"))
                    .collect();
                execute_pipeline(&compress_recipe, files, &registry, &reporter, fake_now).unwrap();
            })
        });
    }
    batch_group.finish();

    // --- Clean CSV recipe ---
    let clean_recipe = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "csv-cleaner", "type": "group",
                "nodes": [{
                    "id": "clean", "type": "spreadsheet",
                    "parameters": { "operation": "clean", "trimWhitespace": true, "removeEmptyRows": true, "removeDuplicates": true }
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("recipe/clean-csv/1-file", |b| {
        b.iter(|| {
            let files = vec![file("data.csv", MESSY_CSV, "text/csv")];
            execute_pipeline(&clean_recipe, files, &registry, &reporter, fake_now).unwrap();
        })
    });

    // --- Rename Files recipe ---
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
                        "id": "rename-file", "type": "file-system",
                        "parameters": { "operation": "rename", "prefix": "renamed-" }
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
            execute_pipeline(&rename_recipe, files, &registry, &reporter, fake_now).unwrap();
        })
    });
}

// =============================================================================
// Registry Benchmarks — Lookup Performance
// =============================================================================

fn bench_registry(c: &mut Criterion) {
    let registry = real_registry();

    // Benchmark the resolve() hot path — this is called once per node per file.
    let mut params = serde_json::Map::new();
    params.insert(
        "operation".to_string(),
        serde_json::Value::String("compress".to_string()),
    );

    c.bench_function("registry/resolve", |b| {
        b.iter(|| {
            registry.resolve("image", &params).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_individual_nodes,
    bench_recipes,
    bench_registry
);
criterion_main!(benches);
