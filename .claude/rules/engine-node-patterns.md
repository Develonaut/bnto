# Engine Node Processor Patterns

Correct patterns for creating and extending Rust node processors. Companion to [node-responsibilities.md](node-responsibilities.md).

---

## Parameter Contract

**Every parameter defined in `metadata()` MUST be read and used in ALL code paths of `process()`.** If a param only applies to some formats, document that in the param description and validate it in `validate()`.

```rust
// BAD — quality param defined but only used for JPEG
fn process(&self, ...) -> Result<...> {
    let quality = get_param("quality");  // read
    match format {
        Jpeg => encode_jpeg(img, quality),  // used
        Png => encode_png(img),             // quality ignored!
        WebP => encode_webp(img),           // quality ignored!
    }
}

// GOOD — quality param flows to all format paths via shared encode
fn process(&self, ...) -> Result<...> {
    let quality = get_param("quality");
    encode::encode_image(&img, format, quality)  // all formats respect quality
}
```

**Test requirement:** Every parameterized node MUST have a test that verifies different parameter values produce measurably different outputs.

---

## Shared Encoding

All image processors MUST use `encode::encode_image()` for final encoding. Never write format-specific encode functions inside individual processors.

```rust
// encode.rs — the single encoding entrypoint
pub fn encode_image(img: &DynamicImage, format: ImageFormat, quality: u8) -> Result<Vec<u8>>
```

Format-specific behavior:

- **JPEG**: `quality` maps to encoder quality (1-100)
- **PNG**: `quality` maps to `CompressionType` — <33 Fast, <66 Default, >=66 Best (lossless, affects speed/size)
- **WebP**: Lossless only (`image` crate limitation). Quality param accepted but documented as no-op

---

## Shared Parameter Definitions

Reusable params live in `common.rs`, not duplicated per-processor:

| Param           | Location                            | Used by              |
| --------------- | ----------------------------------- | -------------------- |
| `quality`       | `common::quality_param_def()`       | resize, convert      |
| `compression`   | `compress::compression_param_def()` | compress             |
| `image_accepts` | `common::image_accepts()`           | all image processors |

When adding a new shared param, put it in `common.rs` and reference it from each processor's `metadata()`.

---

## Parameter Extraction Pattern

Standard pattern for reading params from the JSON config:

```rust
let quality = params
    .get("quality")
    .and_then(|v| v.as_u64())
    .unwrap_or(DEFAULT_JPEG_QUALITY as u64) as u8;
let quality = quality.clamp(1, 100);
```

Steps: get -> and_then (type coerce) -> unwrap_or (default) -> clamp (bounds).

---

## Checklist for Adding a New Node

1. **metadata()** — Define all parameters with types, defaults, constraints, descriptions
2. **process()** — Read and use EVERY parameter in ALL code paths
3. **validate()** — Validate param combinations that metadata constraints can't express
4. **shared encode** — Image processors use `encode::encode_image()`, never custom encode functions
5. **parameterized tests** — Test that different param values produce different outputs
6. **codegen** — Run `task wasm:codegen` to regenerate TypeScript from updated catalog

---

## Common Violations

| Violation                                                               | Fix                                                             |
| ----------------------------------------------------------------------- | --------------------------------------------------------------- |
| Param defined in `metadata()` but not read in some `process()` branches | Wire the param through all branches, or use shared encode       |
| Duplicated encode functions across processors                           | Delete them, use `encode::encode_image()`                       |
| Default value in code differs from `metadata()` default                 | Use the constant from `bnto-core` (e.g., `DEFAULT_COMPRESSION`) |
| Test only checks output validity, not param sensitivity                 | Add a test comparing outputs at two different param values      |
