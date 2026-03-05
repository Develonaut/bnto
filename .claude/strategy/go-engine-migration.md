# Go Engine Node Migration Reference

**Last Updated:** March 2026
**Status:** Pre-deletion audit. Document everything before removing `archive/engine-go/` and `archive/api-go/`.
**Backlog tasks:** See PLAN.md "Chore: Go Engine Archival & Node Migration Reference"

---

## Purpose

This document preserves the implementation details, parameters, patterns, and architectural decisions from the archived Go engine's 10 node types. It serves as the migration reference for:

1. **Rust WASM engine** -- bringing unmigrated node operations to browser execution
2. **M4 cloud execution** -- server-only nodes (`shell-command`, `http-request`) that will run on Railway
3. **Desktop (M3)** -- Tauri native execution using the same Rust codebase
4. **Orchestration patterns** -- `group`, `loop`, `parallel` composition that the Rust engine doesn't yet support

---

## Migration Status Summary

| Node Type | Go Source | Rust Crate | Status | Priority |
|-----------|----------|------------|--------|----------|
| `image` | `library/image/` (679 LOC) | `bnto-image` (224 tests) | Migrated | -- |
| `file-system` | `library/filesystem/` (529 LOC) | `bnto-file` (32 tests) | **Partial** -- rename only | Medium |
| `spreadsheet` | `library/spreadsheet/` (343 LOC) | `bnto-csv` (42 tests) | **Partial** -- CSV only, no Excel | Medium |
| `http-request` | `library/http/` (356 LOC) | -- | **Not migrated** | M4 |
| `transform` | `library/transform/` (129 LOC) | -- | **Not migrated** | Medium (Tier 2) |
| `edit-fields` | `library/editfields/` (178 LOC) | -- | **Not migrated** | Low |
| `shell-command` | `library/shellcommand/` (493 LOC) | -- | **Not migrated** | M4 (Pro) |
| `loop` | `library/loop/` (219 LOC) | -- | **Not migrated** | High (orchestration) |
| `parallel` | `library/parallel/` (408 LOC) | -- | **Not migrated** | M4 |
| `group` | `library/group/` (174 LOC) | -- | **Not migrated** | High (orchestration) |

---

## Fully Migrated Nodes (safe to delete)

### `image` -- Image Processing

**Go source:** `archive/engine-go/pkg/node/library/image/image.go` (679 lines)
**Rust replacement:** `engine/crates/bnto-image/` (224 tests)

Operations migrated: compress (JPEG quality, PNG compression level), resize (with aspect ratio, Lanczos3/CatmullRom), convert (JPEG/PNG/WebP).

**Not yet migrated from Go:**
- `composite` operation (overlay images, position control) -- needed for `/watermark-images` (Tier 2, 30K+ monthly searches)
- `batch` operation (process multiple images with different settings in one call) -- handled per-file in Rust via Web Worker loop
- EXIF metadata strip -- Go used `imaging` library; Rust has EXIF read but not strip. Needed for `/strip-exif` (Tier 2, 15K+ monthly searches)

**Go dependencies:** `github.com/disintegration/imaging`, `github.com/kolesa-team/go-webp`
**Rust dependencies:** `image` v0.25 (pure Rust, no CGO equivalent)

### `file-system` -- File Rename Only

**Go source:** `archive/engine-go/pkg/node/library/filesystem/` (529 lines + glob.go, operations.go, transfer.go, bntoignore.go)
**Rust replacement:** `engine/crates/bnto-file/` (32 tests) -- rename operation only

**Migrated:** `move` (rename) with pattern templates, regex find/replace, case transforms, prefix/suffix.

### `spreadsheet` -- CSV Only

**Go source:** `archive/engine-go/pkg/node/library/spreadsheet/` (343 lines + csv.go, excel.go)
**Rust replacement:** `engine/crates/bnto-csv/` (42 tests) -- CSV clean + rename columns only

**Migrated:** CSV read/write, clean (trim, remove empty rows, deduplicate), rename columns.

---

## Partially Migrated Nodes (document gaps)

### `file-system` -- Unmigrated Operations

The Rust `bnto-file` crate only implements `rename`. The Go engine supported 8 operations. These are needed for Tier 2+ recipes and general-purpose file manipulation.

**Operations to migrate:**

#### `read` -- Read file contents
```
Parameters:
  path (string, required): File path to read
Returns:
  content (string): File contents as UTF-8 text
  size (int): File size in bytes
```

#### `write` -- Write content to file
```
Parameters:
  path (string, required): Output file path
  content (string, required): Content to write
Returns:
  path (string): Written file path
  size (int): Bytes written
```

#### `copy` -- Copy file
```
Parameters:
  source (string, required): Source file path
  dest (string, required): Destination file path
Returns:
  source (string): Original path
  dest (string): New path
  size (int): Bytes copied
```

#### `move` -- Move/rename file (already in Rust as rename)
```
Parameters:
  source (string, required): Source file path
  dest (string, required): Destination file path
Returns:
  source (string): Original path
  dest (string): New path
```

#### `delete` -- Delete file(s) with glob support
```
Parameters:
  path (string, required): File path or glob pattern (e.g., "output/*/temp-*.png")
Returns:
  deleted (int): Number of files deleted
  paths ([]string): Deleted file paths
```

#### `mkdir` -- Create directory
```
Parameters:
  path (string, required): Directory path (creates parent dirs)
Returns:
  path (string): Created directory path
  created (bool): Whether directory was newly created
```

#### `exists` -- Check if path exists
```
Parameters:
  path (string, required): File or directory path
Returns:
  exists (bool): Whether the path exists
  isDir (bool): Whether it's a directory
```

#### `list` -- List files matching pattern
```
Parameters:
  path (string, required): Directory path or glob pattern (e.g., "images/*.jpg")
Returns:
  files ([]string): Matching file paths
  count (int): Number of matches
```

**Go-specific features:**
- Custom glob implementation (`glob.go`) with `**` recursive matching
- `.bntoignore` file support (`bntoignore.go`) -- similar to `.gitignore`, excludes files from list/delete operations
- File transfer helpers (`transfer.go`) -- copy with buffer, progress reporting

**Browser context:** In browser WASM, there's no real filesystem. These operations would apply to:
- Virtual filesystem (in-memory file tree for multi-step recipes)
- Desktop (Tauri) -- real filesystem access
- Cloud (M4) -- server-side filesystem within the execution sandbox

### `spreadsheet` -- Excel (.xlsx) Not Migrated

**Go source:** `archive/engine-go/pkg/node/library/spreadsheet/excel.go`
**Go dependency:** `github.com/xuri/excelize/v2` -- Excel .xlsx read/write

**Excel operations in Go:**
```
readExcel(path string) -> []map[string]interface{}
  - Reads first sheet
  - First row as column headers
  - Each subsequent row as a map (header -> cell value)
  - Handles empty cells gracefully

writeExcel(path string, rows []map[string]interface{}) -> int
  - Creates new .xlsx file
  - First row from map keys (sorted alphabetically)
  - Each map as a data row
  - Returns number of rows written
```

**Rust migration path:** `calamine` crate (read-only, pure Rust) or `rust_xlsxwriter` (write). No pure Rust equivalent to `excelize` that does both read and write with full feature parity. Consider keeping Excel as a Tier 2 feature.

---

## Not Migrated -- Orchestration Nodes

These nodes handle multi-step recipe composition. Currently, browser WASM processes files one-at-a-time via the Web Worker message loop. For multi-step pipelines (e.g., "fetch API -> transform -> write CSV"), orchestration is needed.

### `group` -- Container/Composition Node

**Go source:** `archive/engine-go/pkg/node/library/group/group.go` (174 lines)
**Used in:** Every predefined recipe as the root node.

```
Parameters:
  mode (string): "sequential" (default) or "parallel"
  nodes ([]Definition): Child node definitions (can be nested)
  _context (map): Execution context from previous nodes

Returns:
  mode (string): Execution mode used
  executed (int): Number of child nodes
  nodes ([]interface{}): Child node metadata (for engine orchestration)
```

**Key patterns:**
- Every Go recipe wraps its nodes in a `group` with `mode: "sequential"`
- Groups can nest (group -> group -> node) for hierarchical workflows
- The group node itself validates structure; the engine handles actual child execution
- Context flows forward in sequential mode (each child receives accumulated context from prior siblings)

**Why it matters for migration:**
- The Rust WASM engine currently has no concept of multi-node orchestration
- Browser execution processes one file at a time through one node type
- For multi-step recipes (Tier 2+), the browser adapter or a JS orchestration layer needs to replicate this

### `loop` -- Iteration Node

**Go source:** `archive/engine-go/pkg/node/library/loop/loop.go` (219 lines)
**Go dependency:** `github.com/expr-lang/expr` (for while/break conditions)
**Used in:** compress-images, resize-images, convert-image-format, rename-files (forEach over file lists)

```
Modes:
  forEach: Iterate over an array, passing each item
    Parameters: items ([]interface{}), breakCondition (string, optional)
    Returns: iterations (int), results ([]interface{}), broken (bool, optional)

  times: Repeat N times with index counter
    Parameters: count (int)
    Returns: iterations (int), results ([]{index: i})

  while: Loop while condition evaluates to true
    Parameters: condition (string -- expr expression), _context (map)
    Returns: iterations (int), results ([]{index: i})
    Safety: max 1000 iterations to prevent infinite loops
```

**Key patterns:**
- `forEach` is the workhorse -- every image/file recipe uses it to iterate over file lists
- Break conditions use `expr-lang/expr` for expression evaluation (e.g., `index > 10`)
- Context cancellation checked before each iteration (respects timeouts)
- Results array collects per-iteration output

**Browser context:** The Web Worker currently handles iteration by calling `processFile()` per file from the main thread. True loop support would enable:
- Multi-pass processing (compress -> then resize -> then convert)
- Conditional iteration (process until quality target met)
- Batch operations with inter-file dependencies

### `parallel` -- Concurrent Worker Pool

**Go source:** `archive/engine-go/pkg/node/library/parallel/parallel.go` (408 lines, includes worker.go, worker_execution.go, worker_collector.go)

```
Parameters:
  tasks ([]interface{}): Array of task definitions
  maxWorkers (int): Maximum concurrent workers (default: number of tasks)
  errorStrategy (string): "failFast" (default) or "collectAll"

Returns:
  results ([]interface{}): Task results (ordered by task index)
  errors ([]interface{}): Error array (only if errorStrategy is "collectAll")
```

**Key patterns:**
- Worker pool with configurable concurrency limit
- Two error strategies: `failFast` (cancel all on first error) and `collectAll` (complete all, return errors separately)
- Mutex-protected result collection for thread safety
- Context cancellation propagated to all workers
- Test hooks: `_onStart`, `_onComplete`, `_shouldError` callbacks for deterministic testing

**Browser context:** Web Workers are single-threaded. True parallelism in browser requires:
- Multiple Web Workers (one per parallel branch) -- complex but possible
- Or sequential execution with the `parallel` node as a hint for server-side optimization
- Desktop (Tauri) can use native threads

---

## Not Migrated -- Data Transformation Nodes

### `transform` -- Expression Evaluation

**Go source:** `archive/engine-go/pkg/node/library/transform/transform.go` (129 lines)
**Go dependency:** `github.com/expr-lang/expr` -- powerful expression language

```
Parameters:
  expression (string): Single expression for transformation
    OR
  mappings (map[string]interface{}): Field name -> expression pairs
  _context (map): Execution context with data from previous nodes

Returns (single expression):
  result (interface{}): Evaluation result

Returns (field mapping):
  mapped (map[string]interface{}): Transformed fields
```

**expr-lang capabilities (from Go implementation):**
- Arithmetic: `price * 1.1`, `total / count`
- String ops: `firstName + " " + lastName`, `upper(name)`
- Conditionals: `age >= 18 ? "adult" : "minor"`
- Array ops: `map(items, {.price * .quantity})`, `filter(items, {.active})`
- Nested access: `record.address.city`, `items[0].name`

**Needed for Tier 2 recipes:**
- `/csv-to-json` -- transform CSV rows to JSON structure
- `/validate-json` -- expression-based validation rules
- `/format-json` -- structural transforms

**Browser migration path:** JS-based expression evaluator (e.g., `expr-eval`, `filtrex`, or a custom safe evaluator). The `expr-lang/expr` Go library has no direct JS equivalent, but the subset used in recipes is small.

### `edit-fields` -- Template-Based Field Setting

**Go source:** `archive/engine-go/pkg/node/library/editfields/editfields.go` (178 lines)
**Go dependency:** `text/template` (Go standard library)

```
Parameters:
  values (map[string]interface{}, required): Field name -> value pairs
    Static values passed through as-is
    Template values (containing "{{") evaluated against _context
  _context (map, optional): Data from previous nodes

Returns:
  map[string]interface{}: Processed field values
```

**Template syntax (Go `text/template`):**
```
"{{.record.name}}"              -- Nested field access
"output-{{.record.id}}.png"     -- Mixed static + template
"{{index .items 0}}"            -- Array index
"{{if .active}}yes{{end}}"      -- Conditionals
```

**Key patterns:**
- `isTemplate()` checks for `{{` presence -- only templates are evaluated
- Non-string values pass through unchanged
- Template errors include the field name for debugging
- Stateless -- new instance per execution

**Browser migration path:** JS template literals or a lightweight template engine (Handlebars subset, Mustache). The Go `text/template` syntax is unique to Go -- a migration should define the browser template syntax (likely `{{variable}}` Mustache-style for compatibility with existing `.bnto.json` files).

---

## Not Migrated -- Server-Only Nodes (M4 Pro)

### `http-request` -- External API Client

**Go source:** `archive/engine-go/pkg/node/library/http/http.go` (356 lines)

```
Parameters:
  url (string, required): Target URL
  method (string, required): GET | POST | PUT | DELETE | PATCH | HEAD
  headers (map, optional): Custom headers (including Authorization)
  body (map, optional): JSON request body
  timeout (int, optional): Seconds, default 30
  saveToFile (string, optional): Path to save response body (skips JSON parsing)
  queryParams (map, optional): URL query parameters

Returns:
  statusCode (int): HTTP status code
  body (map): Parsed JSON response (empty if saveToFile)
  headers (map[string]string): Response headers
  filePath (string): Saved file path (only if saveToFile)
```

**Key patterns:**
- URL validation with `net/url.Parse()`
- Query params appended to URL via `url.Values`
- Custom headers applied (supports Bearer auth, API keys)
- Response body parsed as JSON by default
- `saveToFile` mode for binary downloads (skips JSON, writes directly to disk)
- Timeout via `http.Client.Timeout`

**Browser limitations:** CORS restricts browser HTTP requests. This node is primarily server-side (M4 Pro). Limited browser usage possible for CORS-enabled APIs.

**BYOK pattern (from Notion):** Users bring their own API keys, stored in recipe config, never on bnto servers. The http-request node is the execution layer -- it doesn't manage credentials, just uses them.

### `shell-command` -- OS Command Execution

**Go source:** `archive/engine-go/pkg/node/library/shellcommand/shellcommand.go` (493 lines)

```
Parameters:
  command (string, required): Command to execute (e.g., "ffmpeg", "yt-dlp")
  args ([]string, optional): Command arguments
  timeout (int, optional): Seconds, default 120
  stream (bool, optional): Enable line-by-line output streaming
  _onOutput (func(string), optional): Streaming output callback
  retry (int, optional): Retry attempts, default 0
  retryDelay (int, optional): Seconds between retries, default 5
  stallTimeout (int, optional): Kill if no output for N seconds, default 0 (disabled)

Returns:
  stdout (string): Standard output
  stderr (string): Standard error
  exitCode (int): Process exit code
```

**Advanced features (worth preserving):**
1. **Stall detection** -- monitors output activity; if no data received for `stallTimeout` seconds, kills the process. Uses byte-level reading (not line-level) to detect `\r`-based progress updates (e.g., ffmpeg, curl).
2. **Automatic retry** -- retries on failure or stall, with configurable delay. Respects context cancellation between retries.
3. **Streaming output** -- line-by-line progress via callback. Handles both `\n` and `\r` line endings (important for tools that overwrite the current line).
4. **Context cancellation** -- checked before each retry, propagated to child process.

**Security:** Server-only, Pro-tier only. Never exposed in browser. Desktop execution unrestricted (user's own machine).

**Use cases:** ffmpeg (video), yt-dlp (downloads), ImageMagick, pandoc, custom scripts. The stall detection + retry pattern is particularly valuable for unreliable network operations.

---

## Recipe Composition Patterns

Every Go predefined recipe follows this structure:

```json
{
  "id": "recipe-root",
  "type": "group",
  "config": { "mode": "sequential" },
  "nodes": [
    { "id": "list-files", "type": "file-system", "config": { "operation": "list", "path": "{{.INPUT_DIR}}/*" } },
    { "id": "process-each", "type": "loop", "config": { "mode": "forEach" },
      "nodes": [
        { "id": "process-file", "type": "image", "config": { "operation": "optimize", ... } }
      ]
    }
  ]
}
```

**Pattern:** `group(sequential)` -> `file-system(list)` -> `loop(forEach)` -> `<processing node>`

**With I/O nodes (Sprint 4C migration):** The `file-system(list)` step is replaced by an `input` node declaration. The file listing is handled by the execution environment (browser file drop, desktop file picker, CLI args), not the engine.

---

## Go API Server Reference

**Go source:** `archive/api-go/` (~2.5K LOC)
**Deployment:** Railway (ready for M4)

The API server accepts execution requests via HTTP, pulls files from R2, runs the Go engine, pushes output to R2.

**Key endpoints:**
- `POST /api/executions` -- Create and run an execution
- `GET /api/executions/:id` -- Get execution status
- `GET /api/health` -- Health check

**For M4 migration:** The API server pattern is straightforward. Whether M4 uses Go (existing) or Rust (compiled service) is an open decision. The patterns here (R2 integration, execution lifecycle, error reporting) apply regardless of language.

---

## Test Fixtures

The Go engine has integration test fixtures in `archive/engine-go/examples/` and `archive/engine-go/pkg/menu/data/`:

| Fixture | Nodes Used | Status |
|---------|-----------|--------|
| `compress-images.bnto.json` | group, file-system, loop, image | Reference only (Rust handles) |
| `resize-images.bnto.json` | group, file-system, loop, image | Reference only |
| `convert-image-format.bnto.json` | group, file-system, loop, image | Reference only |
| `clean-csv.bnto.json` | group, spreadsheet | Reference only |
| `rename-csv-columns.bnto.json` | group, spreadsheet | Reference only |
| `rename-files.bnto.json` | group, file-system, loop, file-system | Reference only |

These fixtures demonstrate the `group -> loop -> node` composition pattern. The Rust engine processes files differently (per-file via Web Worker), but these fixtures remain the reference for how multi-step recipes should compose.

---

## External Dependencies (Go)

| Dependency | Used By | Rust Equivalent | Notes |
|-----------|---------|----------------|-------|
| `github.com/disintegration/imaging` | image | `image` crate v0.25 | Migrated |
| `github.com/kolesa-team/go-webp` | image (WebP) | `image` crate WebP feature | Migrated (lossless only) |
| `github.com/xuri/excelize/v2` | spreadsheet (Excel) | `calamine` + `rust_xlsxwriter` | Not migrated |
| `github.com/expr-lang/expr` | transform, loop | JS expression evaluator TBD | Not migrated |
| `text/template` (stdlib) | edit-fields | JS template engine TBD | Not migrated |
| `os/exec` (stdlib) | shell-command | Rust `std::process::Command` | Server/desktop only |
| `net/http` (stdlib) | http-request | `reqwest` crate | Server only |
| `encoding/csv` (stdlib) | spreadsheet (CSV) | `csv` crate | Migrated |

---

## Decision Log

- **Feb 2026:** Go engine archived. Rust WASM is primary engine. All 6 Tier 1 recipes running in Rust.
- **Feb 2026:** Desktop tech changed from Wails (Go) to Tauri (Rust). Unified Rust engine vision confirmed.
- **March 2026:** This document created as pre-deletion audit. All Go node implementations documented.
- **Open:** M4 cloud execution -- Go API on Railway (existing) vs Rust compiled service (new). No decision yet.
- **Open:** Expression evaluation in browser -- which JS library replaces `expr-lang/expr`? Needed for `transform` and `loop` while/break conditions.
- **Open:** Template engine for browser -- Go `text/template` syntax vs Mustache/Handlebars. Affects `.bnto.json` format compatibility.
