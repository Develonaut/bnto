//! Kitchen-sink field definitions for the demo binary.
//!
//! Thirteen fields covering every builder method, field kind, and validator
//! that tonkotsu supports. Used by both the demo binary and snapshot tests.

use crate::field::Field;
use crate::group::FieldGroup;
use crate::validators;
use crate::{confirm, file_path, multiselect, note, number, select, text};

/// Build the full set of 13 kitchen-sink demo fields.
pub fn build_fields() -> Vec<Field> {
    vec![
        // 1. Note — read-only informational text (not focusable)
        note("intro", "Configure your recipe settings below.").build(),
        // 1b. MultiSelect — multiple choice with checkboxes
        multiselect(
            "tags",
            &[
                ("image", "Image"),
                ("video", "Video"),
                ("vector", "Vector"),
                ("data", "Data"),
            ],
        )
        .label("Tags")
        .value("image,vector")
        .description(Some("Space to toggle, Enter to confirm"))
        .build(),
        // 2. Text — placeholder, required validator, description
        text("recipe_name")
            .label("Recipe Name")
            .placeholder("My Recipe")
            .required()
            .description(Some("A short name for your recipe"))
            .build(),
        // 3. Text — char_limit, description
        text("description")
            .label("Description")
            .placeholder("What does this recipe do?")
            .char_limit(120)
            .description(Some("Max 120 characters"))
            .build(),
        // 4. Select — 4 options, compact cycle, description
        select(
            "output_format",
            &[
                ("json", "JSON"),
                ("yaml", "YAML"),
                ("toml", "TOML"),
                ("csv", "CSV"),
            ],
        )
        .label("Output Format")
        .value("json")
        .description(Some("Choose the output serialization format"))
        .build(),
        // 5. Select — 8 options, filterable, description
        select(
            "category",
            &[
                ("image", "Image"),
                ("video", "Video"),
                ("audio", "Audio"),
                ("document", "Document"),
                ("data", "Data"),
                ("web", "Web"),
                ("devops", "DevOps"),
                ("ai", "AI"),
            ],
        )
        .label("Category")
        .filterable()
        .description(Some("Filter with keyboard, Enter to expand"))
        .build(),
        // 6. Confirm — standard Yes/No, description
        confirm("overwrite")
            .label("Overwrite Existing?")
            .value("false")
            .description(Some("Replace files if they already exist"))
            .build(),
        // 7. Number — range, step, suffix, default, slider
        number("quality")
            .label("Quality")
            .range(1.0, 100.0)
            .step(5.0)
            .suffix("%")
            .slider(true)
            .value("80")
            .default(Some("80"))
            .description(Some("Output quality from 1-100"))
            .build(),
        // 8. Number — range, step, suffix
        number("max_width")
            .label("Max Width")
            .range(100.0, 4096.0)
            .step(100.0)
            .suffix("px")
            .value("1920")
            .description(Some("Maximum output width in pixels"))
            .build(),
        // 9. Text — pattern("@") validator, description
        text("email")
            .label("Notification Email")
            .placeholder("you@example.com")
            .validator(validators::pattern("@"))
            .description(Some("Must contain @"))
            .build(),
        // 10. Number — range validator on commit (narrower than field range)
        number("min_size")
            .label("Min File Size")
            .range(0.0, 10000.0)
            .step(50.0)
            .suffix("KB")
            .value("100")
            .validator(validators::range(10.0, 5000.0))
            .description(Some("Validated range: 10-5000 KB"))
            .build(),
        // 11. FilePath — file browser with extension filter
        file_path("input_file")
            .label("Input File")
            .extensions(&["jpg", "png", "webp"])
            .description(Some("Browse and select an image file"))
            .build(),
        // 12. Text — visible(false), shows field hiding
        text("hidden_field")
            .label("Hidden Secret")
            .value("you can't see me")
            .visible(false)
            .build(),
    ]
}

/// Build the same kitchen-sink fields organized into named groups.
///
/// Three groups: General (note + tags + name + description + category),
/// Output (format + overwrite + quality + max_width),
/// Advanced (email + min_size + input_file + hidden).
pub fn build_grouped_fields() -> (Vec<Field>, Vec<FieldGroup>) {
    let fields = build_fields();
    let groups = vec![
        FieldGroup::new("General", vec![0, 1, 2, 3, 4]),
        FieldGroup::new("Output", vec![5, 6, 7, 8]),
        FieldGroup::new("Advanced", vec![9, 10, 11, 12]),
    ];
    (fields, groups)
}
