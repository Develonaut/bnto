// =============================================================================
// Rename Files Node — Transform Filenames in the Browser
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the `rename-files` node. When a user drops files on the
// /rename-files page and clicks "Run", this code does the actual work.
//
// HOW DOES FILE RENAMING WORK IN THE BROWSER?
//
//   In the Go engine (CLI/server), rename-files uses the real filesystem:
//   it lists files, then calls `os.Rename()` to move them. But in the
//   browser, there IS no filesystem. Files are just blobs of data with
//   a name attached.
//
//   So the browser version is a FILENAME TRANSFORMER:
//     1. Take a file's current name (e.g., "IMG_1234.jpg")
//     2. Apply transformation rules (pattern, prefix, suffix, find/replace, case)
//     3. Return the same file data with the new name (e.g., "vacation-1234.jpg")
//
//   The Web Worker handles the "file" part (reading bytes, creating downloads).
//   We just handle the "rename" part (transforming the filename string).
//
// TRANSFORMATION ORDER:
//   When multiple transformations are specified, they apply in this order:
//     1. find/replace — modify specific parts of the filename
//     2. case — transform the entire stem to lower/upper/title case
//     3. prefix — prepend a string to the stem
//     4. suffix — append a string to the stem (before the extension)
//     5. pattern — if provided, replaces the ENTIRE filename using a template
//
//   This order means pattern has "final say" — it can reference the original
//   name/ext, but prefix/suffix/case are applied before pattern evaluation.
//   If you use a pattern, it overrides everything else.

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use regex::Regex;

// =============================================================================
// The RenameFiles Struct
// =============================================================================
//
// RUST CONCEPT: Unit Struct
// `struct RenameFiles;` is a "unit struct" — it has no fields. It's like
// an empty class in JavaScript: `class RenameFiles {}`. We use it because
// the NodeProcessor trait needs a type to implement on, but rename-files
// doesn't need any persistent state. Each call to `process()` is stateless.

/// The rename-files node processor. Transforms filenames using patterns,
/// prefixes, suffixes, find/replace, and case transformations.
///
/// The file data passes through UNCHANGED — only the filename is modified.
pub struct RenameFiles;

impl RenameFiles {
    /// Create a new RenameFiles processor.
    ///
    /// RUST CONCEPT: `Self`
    /// `Self` is shorthand for the type we're implementing — `RenameFiles`.
    /// `Self` and `RenameFiles` are interchangeable here.
    pub fn new() -> Self {
        Self
    }
}

// RUST CONCEPT: `Default` trait
// The `Default` trait provides a `default()` method that creates a
// "default" instance of a type. For our unit struct, the default IS
// the only possible instance. Implementing Default is a Rust convention
// that lets callers write `RenameFiles::default()` instead of `RenameFiles::new()`.
impl Default for RenameFiles {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// NodeProcessor Implementation
// =============================================================================

impl NodeProcessor for RenameFiles {
    /// Returns the node type name — used for logging and progress reporting.
    fn name(&self) -> &str {
        "rename-files"
    }

    /// Process a single file: transform its filename according to the params.
    ///
    /// The file data (bytes) passes through UNCHANGED. Only the filename
    /// is transformed based on the configuration parameters.
    ///
    /// PARAMETERS (from the `params` map):
    ///   - `pattern` (string): Template for the output filename.
    ///     Supports: {{name}}, {{ext}}, {{index}}, {{date}}
    ///   - `prefix` (string): Prepend this to the filename stem
    ///   - `suffix` (string): Append this to the stem (before extension)
    ///   - `find` (string): Regex or literal to search for in the stem
    ///   - `replace` (string): Replacement text (used with `find`)
    ///   - `case` (string): "lower", "upper", or "title"
    ///   - `index` (string or number): File index for {{index}} in patterns
    ///
    /// RETURNS:
    ///   - `Ok(NodeOutput)` with one file (same data, new filename)
    ///   - `Err(BntoError)` if the filename is empty or processing fails
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        // --- Step 1: Validate the input ---
        //
        // An empty filename is invalid — we can't rename something with no name.
        // We check this first to give a clear error message.
        if input.filename.is_empty() {
            return Err(BntoError::InvalidInput(
                "Filename cannot be empty".to_string(),
            ));
        }

        // Report that we're starting.
        progress.report(10, "Parsing filename...");

        // --- Step 2: Split the filename into stem and extension ---
        //
        // "photo.jpg" -> stem = "photo", ext = "jpg"
        // "archive.tar.gz" -> stem = "archive.tar", ext = "gz"
        // "README" -> stem = "README", ext = "" (no extension)
        //
        // We split on the LAST dot, because files can have multiple dots
        // (like "archive.tar.gz" — the extension is "gz", not "tar.gz").
        let (original_stem, original_ext) = split_filename(&input.filename);

        progress.report(30, "Applying transformations...");

        // --- Step 3: Start with the original stem and apply transformations ---
        //
        // We clone the stem into a mutable String so we can modify it
        // as we apply each transformation step.
        //
        // RUST CONCEPT: `.to_string()`
        // `original_stem` is a `&str` (a borrowed string slice — read-only).
        // `.to_string()` creates an owned `String` (like copying a string
        // in JavaScript). We need an owned String because we're going to
        // modify it with each transformation step.
        let mut stem = original_stem.to_string();

        // --- Step 3a: Find/Replace ---
        //
        // If `find` is provided, search for it in the stem and replace with
        // the `replace` value (or empty string if `replace` is not provided).
        //
        // We try to compile `find` as a regex first. If it's not valid regex,
        // we treat it as a literal string (exact match replacement).
        if let Some(find_str) = get_string_param(&input.params, "find") {
            let replace_str = get_string_param(&input.params, "replace").unwrap_or_default();
            stem = apply_find_replace(&stem, &find_str, &replace_str);
        }

        // --- Step 3b: Case Transformation ---
        //
        // If `case` is provided, transform the entire stem to the specified case.
        // Supported values: "lower", "upper", "title"
        if let Some(case_str) = get_string_param(&input.params, "case") {
            stem = apply_case_transform(&stem, &case_str);
        }

        // --- Step 3c: Prefix ---
        //
        // If `prefix` is provided, prepend it to the stem.
        // Example: prefix = "new-" → "photo" becomes "new-photo"
        if let Some(prefix) = get_string_param(&input.params, "prefix") {
            stem = format!("{prefix}{stem}");
        }

        // --- Step 3d: Suffix ---
        //
        // If `suffix` is provided, append it to the stem (before extension).
        // Example: suffix = "-final" → "photo" becomes "photo-final"
        if let Some(suffix) = get_string_param(&input.params, "suffix") {
            stem = format!("{stem}{suffix}");
        }

        progress.report(60, "Building new filename...");

        // --- Step 3e: Pattern (overrides everything) ---
        //
        // If `pattern` is provided, it replaces the ENTIRE filename using
        // template variables. The stem we've been building so far is
        // discarded — the pattern uses the ORIGINAL stem/ext plus
        // special variables like {{index}} and {{date}}.
        //
        // Template variables:
        //   {{name}}  — original filename without extension
        //   {{ext}}   — original extension without dot
        //   {{index}} — 1-based file index (from params or default "1")
        //   {{date}}  — current date as YYYY-MM-DD
        let new_filename = if let Some(pattern) = get_string_param(&input.params, "pattern") {
            let index = get_string_param(&input.params, "index").unwrap_or_else(|| "1".to_string());
            let date = get_current_date();
            apply_pattern(&pattern, original_stem, original_ext, &index, &date)
        } else {
            // No pattern — reconstruct filename from the transformed stem + original extension.
            // If the file had no extension, just use the stem.
            if original_ext.is_empty() {
                stem
            } else {
                format!("{stem}.{original_ext}")
            }
        };

        progress.report(90, "Preparing output...");

        // --- Step 4: Build the output ---
        //
        // The file data passes through UNCHANGED. We just wrap it with
        // the new filename and a generic MIME type.
        //
        // We also include metadata about what was done — the original
        // filename, the new filename, and which transformations were applied.
        // The UI uses this to show a summary to the user.
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalFilename".to_string(),
            serde_json::Value::String(input.filename.clone()),
        );
        metadata.insert(
            "newFilename".to_string(),
            serde_json::Value::String(new_filename.clone()),
        );

        // Build a list of which transformations were applied.
        let transforms = build_transforms_list(&input.params);
        metadata.insert(
            "transformsApplied".to_string(),
            serde_json::Value::Array(
                transforms
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );

        progress.report(100, "Done!");

        // Return the output — same data, new filename.
        //
        // RUST CONCEPT: `Ok(...)`
        // `Ok` wraps the successful result in a `Result`. The caller
        // will get `Ok(NodeOutput { ... })` and can unwrap it to get
        // the NodeOutput inside.
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: new_filename,
                // We use application/octet-stream as a generic binary MIME type
                // because rename-files doesn't know or care about the file's actual
                // content type — it only transforms the filename.
                mime_type: "application/octet-stream".to_string(),
            }],
            metadata,
        })
    }
}

// =============================================================================
// Helper Functions — Pure Rust, no WASM boundary concerns
// =============================================================================
//
// These are all pure functions: they take input, return output, no side effects.
// They're easy to test independently and don't depend on the WASM runtime.

/// Split a filename into its stem (name without extension) and extension.
///
/// Examples:
///   "photo.jpg"       → ("photo", "jpg")
///   "archive.tar.gz"  → ("archive.tar", "gz")
///   "README"           → ("README", "")
///   ".gitignore"       → (".gitignore", "")
///
/// We split on the LAST dot. If there's no dot, or the filename starts
/// with a dot and has no other dots (like ".gitignore"), the extension
/// is empty and the entire filename is the stem.
///
/// RUST CONCEPT: `(&str, &str)` return type
/// This returns a tuple (pair) of two string slices. The slices borrow
/// from the input string — no allocation needed. The `'_` lifetime is
/// elided (the compiler figures it out automatically).
fn split_filename(filename: &str) -> (&str, &str) {
    // RUST CONCEPT: `rfind()`
    // `.rfind('.')` searches for the LAST occurrence of '.' in the string.
    // It returns `Option<usize>` — either `Some(position)` or `None`.
    //
    // We look for the last dot because "archive.tar.gz" should split as
    // stem="archive.tar" ext="gz", not stem="archive" ext="tar.gz".
    match filename.rfind('.') {
        // Found a dot at position `pos`.
        // If pos is 0, the filename starts with a dot (like ".gitignore")
        // and we treat the whole thing as the stem with no extension.
        Some(pos) if pos > 0 => {
            // RUST CONCEPT: String slicing with `&str[..pos]`
            // `&filename[..pos]` takes a slice from the start up to (but
            // not including) position `pos`. `&filename[pos + 1..]` takes
            // a slice from one past the dot to the end. This is like
            // `filename.substring(0, pos)` and `filename.substring(pos + 1)`
            // in JavaScript.
            let stem = &filename[..pos];
            let ext = &filename[pos + 1..];
            (stem, ext)
        }
        // No dot found, or dot is at position 0 — no extension.
        _ => (filename, ""),
    }
}

/// Extract a string parameter from the params map.
///
/// Params can be either JSON strings or numbers. We handle both:
///   - `"prefix": "new-"` → Some("new-")
///   - `"index": 3` → Some("3")
///   - key not present → None
///
/// RUST CONCEPT: `Option<String>`
/// Returns `Some(string)` if the key exists with a value we can convert,
/// or `None` if the key doesn't exist or the value is null.
fn get_string_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    // RUST CONCEPT: `match` expression
    // `match` is Rust's pattern matching — like a super-powered `switch`
    // statement in JavaScript. It checks the value against each pattern
    // and runs the code for the first match. Unlike `switch`, `match`
    // is exhaustive — the compiler makes sure you handle every case.
    params.get(key).and_then(|v| match v {
        // If it's a JSON string, return the string value.
        serde_json::Value::String(s) => Some(s.clone()),
        // If it's a JSON number, convert it to a string.
        // This handles the case where `index` is passed as `3` instead of `"3"`.
        serde_json::Value::Number(n) => Some(n.to_string()),
        // Anything else (null, bool, array, object) → None.
        _ => None,
    })
}

/// Apply find/replace to a string. Tries regex first, falls back to literal.
///
/// If `find` is a valid regex pattern, it uses regex replacement.
/// If `find` is NOT valid regex, it treats it as a literal string and
/// replaces all occurrences.
///
/// RUST CONCEPT: `Regex::new()` returns `Result`
/// Compiling a regex can fail (if the pattern is invalid). `Regex::new()`
/// returns `Result<Regex, Error>`. We use `.ok()` to convert the error to
/// `None`, and then handle both cases.
fn apply_find_replace(stem: &str, find: &str, replace: &str) -> String {
    // Try to compile the `find` string as a regex pattern.
    match Regex::new(find) {
        // It's valid regex! Use `replace_all` for regex-powered replacement.
        //
        // RUST CONCEPT: `Regex::replace_all()`
        // Returns a `Cow<str>` — either a borrowed reference to the original
        // string (if no replacements were made) or a new owned String.
        // `.to_string()` converts either case to an owned String.
        Ok(re) => re.replace_all(stem, replace).to_string(),

        // Not valid regex — fall back to literal string replacement.
        // `.replace()` does an exact string match and replaces ALL occurrences.
        Err(_) => stem.replace(find, replace),
    }
}

/// Apply a case transformation to a string.
///
/// Supported case values:
///   - "lower" → all lowercase ("PHOTO" → "photo")
///   - "upper" → all uppercase ("photo" → "PHOTO")
///   - "title" → first letter uppercase, rest lowercase ("pHoTo" → "Photo")
///   - anything else → no change (returns the input unchanged)
fn apply_case_transform(stem: &str, case: &str) -> String {
    match case {
        "lower" => stem.to_lowercase(),
        "upper" => stem.to_uppercase(),
        "title" => to_title_case(stem),
        // Unknown case value — return unchanged.
        // We don't error here because it's a user config mistake, not a
        // processing failure. The file still gets renamed with other rules.
        _ => stem.to_string(),
    }
}

/// Convert a string to title case (first letter uppercase, rest lowercase).
///
/// "hello world" → "Hello world"
/// "PHOTO" → "Photo"
/// "" → ""
///
/// RUST CONCEPT: `.chars()` and iterators
/// Strings in Rust are UTF-8 encoded bytes. `.chars()` gives us an iterator
/// over the individual characters. We take the first character, uppercase it,
/// then lowercase the rest. This handles Unicode correctly — `'ü'.to_uppercase()`
/// gives `'Ü'`, not some ASCII-only hack.
fn to_title_case(s: &str) -> String {
    // RUST CONCEPT: `let mut` — mutable variable
    // By default, variables in Rust are immutable (can't be changed).
    // `let mut chars` means "I want to be able to advance this iterator."
    let mut chars = s.chars();

    // RUST CONCEPT: `match` on `Option`
    // `chars.next()` returns `Option<char>` — either `Some('h')` (the first
    // character) or `None` (empty string). We handle both cases.
    match chars.next() {
        None => String::new(),
        Some(first) => {
            // `.to_uppercase()` returns an iterator of chars (because some
            // characters uppercase to multiple characters, like 'ß' → "SS").
            // `.collect::<String>()` joins those chars into a String.
            // Then we append the rest of the string in lowercase.
            let upper_first: String = first.to_uppercase().collect();
            let lower_rest: String = chars.as_str().to_lowercase();
            format!("{upper_first}{lower_rest}")
        }
    }
}

/// Apply a template pattern to generate a new filename.
///
/// Template variables:
///   {{name}}  — original filename stem (without extension)
///   {{ext}}   — original file extension (without dot)
///   {{index}} — file index (for batch processing)
///   {{date}}  — current date as YYYY-MM-DD
///
/// Example:
///   pattern = "{{date}}-{{name}}-v2.{{ext}}"
///   name = "photo", ext = "jpg", index = "1", date = "2026-02-25"
///   result = "2026-02-25-photo-v2.jpg"
fn apply_pattern(pattern: &str, name: &str, ext: &str, index: &str, date: &str) -> String {
    // RUST CONCEPT: Method chaining with `.replace()`
    // Each `.replace()` call returns a new String with the substitution made.
    // We chain them together to apply all substitutions in sequence.
    // This is like doing `pattern.replaceAll("{{name}}", name)` in JavaScript.
    pattern
        .replace("{{name}}", name)
        .replace("{{ext}}", ext)
        .replace("{{index}}", index)
        .replace("{{date}}", date)
}

/// Get the current date as a YYYY-MM-DD string.
///
/// In a WASM environment, we use `js_sys::Date` to get the current date
/// from the browser's JavaScript runtime. In native Rust tests, we also
/// use `js_sys::Date` if available, but our tests mock this or accept
/// any valid date format.
///
/// For unit tests (native Rust, no JS runtime), we return a placeholder
/// date. The WASM integration tests verify the real JS Date path.
fn get_current_date() -> String {
    // In a WASM environment, we'd use js_sys::Date. But since this function
    // is also called from native unit tests (no JS runtime), we use cfg to
    // switch between implementations.
    //
    // RUST CONCEPT: `#[cfg(target_arch = "wasm32")]`
    // This is conditional compilation. The compiler only includes this code
    // when building for the wasm32 target (browser). For native builds
    // (unit tests), it uses the other branch.
    #[cfg(target_arch = "wasm32")]
    {
        // Use JavaScript's Date object to get the current date.
        let date = js_sys::Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1; // JS months are 0-indexed (Jan=0)
        let day = date.get_date();
        format!("{year:04}-{month:02}-{day:02}")
    }

    // For native Rust tests, return a fixed date so tests are deterministic.
    #[cfg(not(target_arch = "wasm32"))]
    {
        "2026-02-25".to_string()
    }
}

/// Build a list of which transformations were applied based on the params.
///
/// This is used for metadata — the UI shows "Applied: prefix, case, suffix"
/// so the user knows what happened to their filename.
fn build_transforms_list(params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    // RUST CONCEPT: `Vec::new()` and `.push()`
    // We start with an empty vector (dynamic array) and add items to it
    // as we discover which params are present. This is like building an
    // array with `const arr = []; arr.push(...)` in JavaScript.
    let mut transforms = Vec::new();

    if params.contains_key("find") {
        transforms.push("find/replace".to_string());
    }
    if params.contains_key("case") {
        transforms.push("case".to_string());
    }
    if params.contains_key("prefix") {
        transforms.push("prefix".to_string());
    }
    if params.contains_key("suffix") {
        transforms.push("suffix".to_string());
    }
    if params.contains_key("pattern") {
        transforms.push("pattern".to_string());
    }

    transforms
}

// =============================================================================
// Unit Tests — Pure Rust, no WASM boundary needed
// =============================================================================
//
// RUST CONCEPT: `#[cfg(test)]`
// This attribute means "only compile this module when running tests".
// It's completely removed from the production WASM binary — zero overhead.
// Unit tests run natively with `cargo test` — no browser or Node.js needed.

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    /// Create a NodeInput with the given data, filename, and params.
    ///
    /// This helper keeps test code concise. Instead of constructing a
    /// full NodeInput struct every time, we call this with just the
    /// pieces we care about.
    fn make_input(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params,
        }
    }

    /// Create an empty params map — for tests where no transformations are applied.
    fn empty_params() -> serde_json::Map<String, serde_json::Value> {
        serde_json::Map::new()
    }

    /// Create a params map with a single string key-value pair.
    fn string_param(key: &str, value: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert(
            key.to_string(),
            serde_json::Value::String(value.to_string()),
        );
        params
    }

    /// Create a params map from a list of string key-value pairs.
    fn string_params(pairs: &[(&str, &str)]) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        for (key, value) in pairs {
            params.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
        params
    }

    /// The no-op progress reporter — doesn't need a JS runtime.
    fn noop_progress() -> ProgressReporter {
        ProgressReporter::new_noop()
    }

    // =========================================================================
    // split_filename Tests
    // =========================================================================

    #[test]
    fn test_split_simple_filename() {
        let (stem, ext) = split_filename("photo.jpg");
        assert_eq!(stem, "photo");
        assert_eq!(ext, "jpg");
    }

    #[test]
    fn test_split_filename_multiple_dots() {
        // "archive.tar.gz" — extension is the LAST part after the last dot.
        let (stem, ext) = split_filename("archive.tar.gz");
        assert_eq!(stem, "archive.tar");
        assert_eq!(ext, "gz");
    }

    #[test]
    fn test_split_filename_no_extension() {
        let (stem, ext) = split_filename("README");
        assert_eq!(stem, "README");
        assert_eq!(ext, "");
    }

    #[test]
    fn test_split_dotfile() {
        // ".gitignore" starts with a dot — the whole thing is the stem.
        let (stem, ext) = split_filename(".gitignore");
        assert_eq!(stem, ".gitignore");
        assert_eq!(ext, "");
    }

    // =========================================================================
    // Pattern Template Tests
    // =========================================================================

    #[test]
    fn test_pattern_name_and_ext() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "{{name}}-compressed.{{ext}}");
        let input = make_input(b"file data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo-compressed.jpg");
    }

    #[test]
    fn test_pattern_with_index() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("pattern", "file-{{index}}.{{ext}}"), ("index", "5")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "file-5.jpg");
    }

    #[test]
    fn test_pattern_with_default_index() {
        // When no index is provided, it defaults to "1".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "file-{{index}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "file-1.jpg");
    }

    #[test]
    fn test_pattern_with_date() {
        // In native tests, get_current_date() returns "2026-02-25".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "{{date}}-{{name}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "2026-02-25-photo.jpg");
    }

    // =========================================================================
    // Prefix / Suffix Tests
    // =========================================================================

    #[test]
    fn test_prefix_only() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "new-photo.jpg");
    }

    #[test]
    fn test_suffix_only() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("suffix", "-final");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo-final.jpg");
    }

    #[test]
    fn test_prefix_and_suffix_combined() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "v2-"), ("suffix", "-edited")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "v2-photo-edited.jpg");
    }

    // =========================================================================
    // Find / Replace Tests
    // =========================================================================

    #[test]
    fn test_find_replace_literal() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "old"), ("replace", "new")]);
        let input = make_input(b"data", "old-photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "new-photo.jpg");
    }

    #[test]
    fn test_find_replace_regex() {
        // Use a regex pattern to capture and rewrite part of the filename.
        // "IMG_(\d+)" captures the number after "IMG_".
        // "photo_$1" replaces "IMG_1234" with "photo_1234".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", r"IMG_(\d+)"), ("replace", "photo_$1")]);
        let input = make_input(b"data", "IMG_1234.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo_1234.jpg");
    }

    #[test]
    fn test_find_replace_invalid_regex_falls_back_to_literal() {
        // "[invalid" is not valid regex (unclosed bracket).
        // It should fall back to literal string replacement.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "[invalid"), ("replace", "fixed")]);
        let input = make_input(b"data", "file-[invalid-name.txt", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "file-fixed-name.txt");
    }

    // =========================================================================
    // Case Transformation Tests
    // =========================================================================

    #[test]
    fn test_case_lower() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "lower");
        let input = make_input(b"data", "PHOTO.JPG", params);

        let output = processor.process(input, &progress).unwrap();

        // Note: case transforms the STEM only, not the extension.
        // The extension comes from the original filename's extension.
        assert_eq!(output.files[0].filename, "photo.JPG");
    }

    #[test]
    fn test_case_upper() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "upper");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "PHOTO.jpg");
    }

    #[test]
    fn test_case_title() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "title");
        let input = make_input(b"data", "hello world.txt", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "Hello world.txt");
    }

    // =========================================================================
    // Pass-Through and Edge Case Tests
    // =========================================================================

    #[test]
    fn test_no_params_filename_unchanged() {
        // With no transformation params, the filename should pass through unchanged.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let input = make_input(b"data", "photo.jpg", empty_params());

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo.jpg");
    }

    #[test]
    fn test_data_passes_through_unchanged() {
        // The file DATA should be exactly the same — only the filename changes.
        let original_data = b"This is the original file content. It should not change!";
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "renamed-");
        let input = make_input(original_data, "file.txt", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].data, original_data);
    }

    #[test]
    fn test_file_with_no_extension() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "README", params);

        let output = processor.process(input, &progress).unwrap();

        // No extension, so no dot appended.
        assert_eq!(output.files[0].filename, "new-README");
    }

    #[test]
    fn test_file_with_multiple_dots() {
        // "archive.tar.gz" — extension is "gz", stem is "archive.tar"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "backup-");
        let input = make_input(b"data", "archive.tar.gz", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "backup-archive.tar.gz");
    }

    #[test]
    fn test_empty_filename_returns_error() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let input = make_input(b"data", "", empty_params());

        let result = processor.process(input, &progress);

        // RUST CONCEPT: `assert!(result.is_err())`
        // We check that the result is an error, not success.
        assert!(result.is_err());

        // RUST CONCEPT: `if let Err(e) = result`
        // Pattern matching to extract the error without needing Debug on NodeOutput.
        // `unwrap_err()` requires the Ok type to implement Debug, but NodeOutput
        // doesn't. Using `if let` avoids that requirement.
        if let Err(e) = result {
            assert!(
                e.to_string().contains("empty"),
                "Error should mention empty filename: {e}"
            );
        }
    }

    // =========================================================================
    // Combined Transformation Tests
    // =========================================================================

    #[test]
    fn test_combined_prefix_case_suffix() {
        // Order: find/replace → case → prefix → suffix
        // Starting stem: "Photo" (from "Photo.jpg")
        // After case="lower": "photo"
        // After prefix="v2-": "v2-photo"
        // After suffix="-edited": "v2-photo-edited"
        // Final: "v2-photo-edited.jpg"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "v2-"), ("suffix", "-edited"), ("case", "lower")]);
        let input = make_input(b"data", "Photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "v2-photo-edited.jpg");
    }

    #[test]
    fn test_find_replace_with_case() {
        // Order: find/replace → case
        // Starting stem: "IMG_1234" (from "IMG_1234.jpg")
        // After find/replace (IMG→photo): "photo_1234"
        // After case="upper": "PHOTO_1234"
        // Final: "PHOTO_1234.jpg"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "IMG"), ("replace", "photo"), ("case", "upper")]);
        let input = make_input(b"data", "IMG_1234.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "PHOTO_1234.jpg");
    }

    // =========================================================================
    // Metadata Tests
    // =========================================================================

    #[test]
    fn test_metadata_includes_original_filename() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        let original = output.metadata.get("originalFilename").unwrap();
        assert_eq!(original.as_str().unwrap(), "photo.jpg");
    }

    #[test]
    fn test_metadata_includes_new_filename() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        let new_name = output.metadata.get("newFilename").unwrap();
        assert_eq!(new_name.as_str().unwrap(), "new-photo.jpg");
    }

    #[test]
    fn test_metadata_includes_transforms_applied() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "new-"), ("case", "lower")]);
        let input = make_input(b"data", "Photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        let transforms = output.metadata.get("transformsApplied").unwrap();
        let transforms_arr = transforms.as_array().unwrap();
        // Should include "case" and "prefix" (order defined by build_transforms_list).
        assert!(transforms_arr.len() >= 2);
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello"), "Hello");
        assert_eq!(to_title_case("PHOTO"), "Photo");
        assert_eq!(to_title_case("hello world"), "Hello world");
        assert_eq!(to_title_case(""), "");
        assert_eq!(to_title_case("a"), "A");
    }

    #[test]
    fn test_apply_pattern() {
        let result = apply_pattern(
            "{{date}}-{{name}}-{{index}}.{{ext}}",
            "photo",
            "jpg",
            "3",
            "2026-02-25",
        );
        assert_eq!(result, "2026-02-25-photo-3.jpg");
    }

    #[test]
    fn test_apply_find_replace_literal_no_match() {
        // When the find string doesn't match anything, return unchanged.
        let result = apply_find_replace("hello", "xyz", "abc");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_apply_find_replace_all_occurrences() {
        // Regex replace_all should replace ALL occurrences, not just the first.
        let result = apply_find_replace("aaa-bbb-aaa", "aaa", "xxx");
        assert_eq!(result, "xxx-bbb-xxx");
    }

    #[test]
    fn test_get_current_date_format() {
        let date = get_current_date();
        // Should be YYYY-MM-DD format (10 characters, dashes at positions 4 and 7).
        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
        assert_eq!(&date[7..8], "-");
    }
}
