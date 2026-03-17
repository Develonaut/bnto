// Benchmarks -- node-level processor performance and registry lookups.

use criterion::{Criterion, criterion_group, criterion_main};

use bnto_core::{
    NodeRegistry, PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline,
};

// --- Test fixtures ---
static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

// --- Helpers ---

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

// --- Node-Level Benchmarks ---

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

// --- Registry Benchmarks ---

fn bench_registry(c: &mut Criterion) {
    let registry = real_registry();

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

criterion_group!(benches, bench_individual_nodes, bench_registry);
criterion_main!(benches);
