//! Kitchen-sink field definitions for the demo binary.
//!
//! Ten fields covering every builder method, field kind, and validator
//! that bnto-form supports. Used by both the demo binary and snapshot tests.

use crate::field::Field;
use crate::validators;
use crate::{confirm, number, select, text};

/// Build the full set of 10 kitchen-sink demo fields.
pub fn build_fields() -> Vec<Field> {
    vec![
        // 1. Text — placeholder, required validator, description
        text("recipe_name")
            .label("Recipe Name")
            .placeholder("My Recipe")
            .required()
            .description(Some("A short name for your recipe"))
            .build(),
        // 2. Text — char_limit, description
        text("description")
            .label("Description")
            .placeholder("What does this recipe do?")
            .char_limit(120)
            .description(Some("Max 120 characters"))
            .build(),
        // 3. Select — 4 options, compact cycle, description
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
        // 4. Select — 8 options, filterable, description
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
        // 5. Confirm — standard Yes/No, description
        confirm("overwrite")
            .label("Overwrite Existing?")
            .value("false")
            .description(Some("Replace files if they already exist"))
            .build(),
        // 6. Number — range, step, suffix, default, slider
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
        // 7. Number — range, step, suffix
        number("max_width")
            .label("Max Width")
            .range(100.0, 4096.0)
            .step(100.0)
            .suffix("px")
            .value("1920")
            .description(Some("Maximum output width in pixels"))
            .build(),
        // 8. Text — pattern("@") validator, description
        text("email")
            .label("Notification Email")
            .placeholder("you@example.com")
            .validator(validators::pattern("@"))
            .description(Some("Must contain @"))
            .build(),
        // 9. Number — range validator on commit (narrower than field range)
        number("min_size")
            .label("Min File Size")
            .range(0.0, 10000.0)
            .step(50.0)
            .suffix("KB")
            .value("100")
            .validator(validators::range(10.0, 5000.0))
            .description(Some("Validated range: 10-5000 KB"))
            .build(),
        // 10. Text — visible(false), shows field hiding
        text("hidden_field")
            .label("Hidden Secret")
            .value("you can't see me")
            .visible(false)
            .build(),
    ]
}
