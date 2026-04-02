// Recipe integration tests -- image processors (compress, resize, convert, metadata).

mod common;

use bnto_core::{NoopContext, PipelineReporter, execute_pipeline};
use common::{SMALL_JPEG, SMALL_PNG, fake_now, file, parse, real_registry};

// --- Compress Images -- full recipe integration ---

#[test]
fn compress_images_recipe_produces_smaller_output() {
    let def = parse(
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
                        "parameters": { "quality": 50 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let input_size = SMALL_JPEG.len();
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("compress pipeline should succeed");

    assert_eq!(result.files.len(), 1, "should output 1 file");
    assert!(result.files[0].data.len() >= 2, "output should have data");
    assert_eq!(
        &result.files[0].data[0..2],
        &[0xFF, 0xD8],
        "output should be a valid JPEG (magic bytes)"
    );
    assert!(
        result.files[0].data.len() < input_size,
        "compressed JPEG at q=50 ({} bytes) should be smaller than input ({} bytes)",
        result.files[0].data.len(),
        input_size
    );
}

// --- Compress Images -- batch (multiple files) ---

#[test]
fn compress_images_recipe_handles_batch() {
    let def = parse(
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

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![
        file("a.jpg", SMALL_JPEG, "image/jpeg"),
        file("b.png", SMALL_PNG, "image/png"),
    ];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("batch compress should succeed");

    assert_eq!(result.files.len(), 2, "batch should output 2 files");
    for f in &result.files {
        assert!(
            !f.data.is_empty(),
            "output file '{}' should have data",
            f.name
        );
    }
}

// --- Compress Images -- metadata verification ---

#[test]
fn compress_images_metadata_includes_size_stats() {
    let def = parse(
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
                        "parameters": { "quality": 50 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let input_size = SMALL_JPEG.len() as u64;
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("compress pipeline should succeed");

    assert_eq!(result.files.len(), 1);
    let metadata = &result.files[0].metadata;

    assert_eq!(
        metadata["originalSize"].as_u64().unwrap(),
        input_size,
        "originalSize should match the input file size"
    );

    let compressed_size = metadata["compressedSize"].as_u64().unwrap();
    assert_eq!(
        compressed_size,
        result.files[0].data.len() as u64,
        "compressedSize should match the output data length"
    );

    let ratio = metadata["compressionRatio"].as_f64().unwrap();
    assert!(
        ratio > 0.0 && ratio < 100.0,
        "compressionRatio {} should be between 0 and 100 (percentage)",
        ratio
    );
}

// --- Resize Images -- full recipe integration ---

#[test]
fn resize_images_recipe_produces_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-resize", "type": "group",
                "nodes": [{
                    "id": "resize-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "resize-image", "type": "image-resize",
                        "parameters": { "width": 100 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("resize pipeline should succeed");

    assert_eq!(result.files.len(), 1);
    assert!(result.files[0].data.len() >= 2);
    assert_eq!(&result.files[0].data[0..2], &[0xFF, 0xD8]);
}

// --- Convert Image Format -- full recipe integration ---

#[test]
fn convert_image_format_recipe_produces_png() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-convert", "type": "group",
                "nodes": [{
                    "id": "convert-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "convert-image", "type": "image-convert",
                        "parameters": { "format": "png", "quality": 80 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .expect("convert pipeline should succeed");

    assert_eq!(result.files.len(), 1);
    assert!(result.files[0].data.len() >= 4);
    assert_eq!(
        &result.files[0].data[0..4],
        &[0x89, 0x50, 0x4E, 0x47],
        "output should be a valid PNG (magic bytes)"
    );
}
