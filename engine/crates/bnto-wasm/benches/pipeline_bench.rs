// Pipeline Benchmarks -- track node and recipe performance over time.
//
// Uses Criterion to measure engine processing speed. The first run
// establishes a baseline; subsequent runs detect regressions.
// Results saved in engine/target/criterion/.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use bnto_core::{
    NodeRegistry, PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline,
};

static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static LARGE_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/large.png");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

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

// =========================================================================
// Node-Level Benchmarks
// =========================================================================

fn bench_individual_nodes(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

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

// =========================================================================
// Recipe-Level Benchmarks
// =========================================================================

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

    // Batch scaling: measure how performance changes with file count.
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

// =========================================================================
// Registry Benchmarks
// =========================================================================

fn bench_registry(c: &mut Criterion) {
    let registry = real_registry();

    // resolve() hot path -- called once per node per file.
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

// =========================================================================
// PNG Compression Benchmarks
// =========================================================================
//
// PNG uses quantizr (median cut + Floyd-Steinberg dithering) for palette
// reduction before DEFLATE. Tracks both speed and output size.

fn bench_png_compression(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

    let compress_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress" } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image:compress/png/large", |b| {
        b.iter(|| {
            let files = vec![file("photo.png", LARGE_PNG, "image/png")];
            execute_pipeline(&compress_def, files, &registry, &reporter, fake_now).unwrap()
        })
    });

    // One-shot: print output size for regression tracking.
    let files = vec![file("photo.png", LARGE_PNG, "image/png")];
    let result = execute_pipeline(&compress_def, files, &registry, &reporter, fake_now).unwrap();
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

criterion_group!(
    benches,
    bench_individual_nodes,
    bench_recipes,
    bench_registry,
    bench_png_compression
);
criterion_main!(benches);
