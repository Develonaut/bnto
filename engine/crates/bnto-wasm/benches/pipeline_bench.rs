// Benchmarks -- node-level processor performance and registry lookups.

use criterion::{Criterion, criterion_group, criterion_main};

use bnto_core::{
    NoopContext, PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline,
};

// --- Test fixtures ---
static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

// --- Helpers ---

fn real_registry() -> bnto_core::NodeRegistry {
    bnto_engine::create_browser_registry()
}

fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: bnto_core::processor::FileData::Bytes(data.to_vec()),
        mime_type: mime.to_string(),
        metadata: serde_json::Map::new(),
    }
}

fn fake_now() -> u64 {
    1000
}

// --- Node-Level Benchmarks ---

fn bench_individual_nodes(c: &mut Criterion) {
    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();

    let compress_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image-compress", "parameters": { "quality": 80 } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image-compress/jpeg", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(
                &compress_def,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });

    let resize_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "resize", "type": "image-resize", "parameters": { "width": 100 } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image:resize/jpeg", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(
                &resize_def,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });

    let convert_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "convert", "type": "image-convert", "parameters": { "format": "png", "quality": 80 } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/image:convert/jpeg_to_png", |b| {
        b.iter(|| {
            let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
            execute_pipeline(
                &convert_def,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });

    let clean_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "clean", "type": "spreadsheet-clean", "parameters": { "trimWhitespace": true, "removeEmptyRows": true, "removeDuplicates": true } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/spreadsheet-clean/csv", |b| {
        b.iter(|| {
            let files = vec![file("data.csv", MESSY_CSV, "text/csv")];
            execute_pipeline(
                &clean_def,
                files,
                &registry,
                &reporter,
                &NoopContext,
                fake_now,
            )
            .unwrap();
        })
    });

    let rename_def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "rename", "type": "file-rename", "parameters": { "prefix": "renamed-" } },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    c.bench_function("node/file-rename/txt", |b| {
        b.iter(|| {
            let files = vec![file("document.txt", b"hello world", "text/plain")];
            execute_pipeline(
                &rename_def,
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

// --- Registry Benchmarks ---

fn bench_registry(c: &mut Criterion) {
    let registry = real_registry();

    let params = serde_json::Map::new();

    c.bench_function("registry/resolve", |b| {
        b.iter(|| {
            registry.resolve("image-compress", &params).unwrap();
        })
    });
}

criterion_group!(benches, bench_individual_nodes, bench_registry);
criterion_main!(benches);
