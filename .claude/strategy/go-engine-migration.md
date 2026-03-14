# Go Engine Node Migration Reference

**Last Updated:** March 2026
**Status:** Reference tables for Rust migration. Full Go source preserved in `archive/engine-go/`.

---

## Migration Status

| Node Type       | Go Source (LOC)               | Rust Crate               | Status                           | Priority              |
| --------------- | ----------------------------- | ------------------------ | -------------------------------- | --------------------- |
| `image`         | `library/image/` (679)        | `bnto-image` (224 tests) | Migrated                         | --                    |
| `file-system`   | `library/filesystem/` (529)   | `bnto-file` (32 tests)   | **Partial** — rename only        | Medium                |
| `spreadsheet`   | `library/spreadsheet/` (343)  | `bnto-csv` (42 tests)    | **Partial** — CSV only, no Excel | Medium                |
| `http-request`  | `library/http/` (356)         | --                       | Not migrated                     | M4 (server-only)      |
| `transform`     | `library/transform/` (129)    | --                       | Not migrated                     | Medium (Tier 2)       |
| `edit-fields`   | `library/editfields/` (178)   | --                       | Not migrated                     | Low                   |
| `shell-command` | `library/shellcommand/` (493) | --                       | Not migrated                     | M4 (Pro, server-only) |
| `loop`          | `library/loop/` (219)         | --                       | Not migrated                     | High (orchestration)  |
| `parallel`      | `library/parallel/` (408)     | --                       | Not migrated                     | M4                    |
| `group`         | `library/group/` (174)        | --                       | Not migrated                     | High (orchestration)  |

---

## Unmigrated Node Parameters

### `file-system` — 7 operations beyond rename

| Operation | Parameters                  | Returns                                  |
| --------- | --------------------------- | ---------------------------------------- |
| `read`    | `path` (string)             | `content` (string), `size` (int)         |
| `write`   | `path`, `content` (strings) | `path` (string), `size` (int)            |
| `copy`    | `source`, `dest` (strings)  | `source`, `dest` (strings), `size` (int) |
| `delete`  | `path` (string/glob)        | `deleted` (int), `paths` ([]string)      |
| `mkdir`   | `path` (string)             | `path` (string), `created` (bool)        |
| `exists`  | `path` (string)             | `exists` (bool), `isDir` (bool)          |
| `list`    | `path` (string/glob)        | `files` ([]string), `count` (int)        |

Go-specific: custom glob with `**` recursive, `.bntoignore` support, buffered copy with progress.

### `spreadsheet` — Excel (.xlsx) not migrated

Go dep: `excelize/v2`. Read first sheet (row 1 = headers), write from map array. Rust path: `calamine` (read) + `rust_xlsxwriter` (write).

### `group` — Container node

| Parameter | Type                           | Description                       |
| --------- | ------------------------------ | --------------------------------- |
| `mode`    | `"sequential"` \| `"parallel"` | Execution mode                    |
| `nodes`   | `[]Definition`                 | Child node definitions (nestable) |

Context flows forward in sequential mode. Every Go recipe wraps nodes in `group(sequential)`.

### `loop` — Iteration node

| Mode      | Parameters                                         | Returns                                   |
| --------- | -------------------------------------------------- | ----------------------------------------- |
| `forEach` | `items` (array), `breakCondition` (expr, optional) | `iterations`, `results`, `broken`         |
| `times`   | `count` (int)                                      | `iterations`, `results`                   |
| `while`   | `condition` (expr)                                 | `iterations`, `results` (max 1000 safety) |

Go dep: `expr-lang/expr` for conditions. `forEach` is the workhorse — every image/file recipe uses it.

### `parallel` — Concurrent worker pool

| Parameter       | Type                           | Description                          |
| --------------- | ------------------------------ | ------------------------------------ |
| `tasks`         | `[]interface{}`                | Task definitions                     |
| `maxWorkers`    | `int`                          | Max concurrent (default: task count) |
| `errorStrategy` | `"failFast"` \| `"collectAll"` | Error handling mode                  |

### `transform` — Expression evaluation

| Parameter    | Type                     | Description                                              |
| ------------ | ------------------------ | -------------------------------------------------------- |
| `expression` | `string`                 | Single expression (arithmetic, string ops, conditionals) |
| `mappings`   | `map[string]interface{}` | Field name → expression pairs                            |

Go dep: `expr-lang/expr`. Browser migration needs a JS expression evaluator.

### `edit-fields` — Template-based field setting

| Parameter | Type                     | Description                                   |
| --------- | ------------------------ | --------------------------------------------- |
| `values`  | `map[string]interface{}` | Field name → value (static or `{{template}}`) |

Go dep: `text/template`. Browser migration needs a JS template engine (Mustache-style).

### `http-request` — External API client (server-only)

| Parameter     | Type     | Description                    |
| ------------- | -------- | ------------------------------ |
| `url`         | `string` | Target URL                     |
| `method`      | `string` | GET/POST/PUT/DELETE/PATCH/HEAD |
| `headers`     | `map`    | Custom headers (incl. auth)    |
| `body`        | `map`    | JSON request body              |
| `timeout`     | `int`    | Seconds (default 30)           |
| `saveToFile`  | `string` | Path to save binary response   |
| `queryParams` | `map`    | URL query parameters           |

### `shell-command` — OS command execution (server/desktop only)

| Parameter      | Type       | Description                     |
| -------------- | ---------- | ------------------------------- |
| `command`      | `string`   | Command (e.g., "ffmpeg")        |
| `args`         | `[]string` | Arguments                       |
| `timeout`      | `int`      | Seconds (default 120)           |
| `stream`       | `bool`     | Line-by-line output streaming   |
| `retry`        | `int`      | Retry attempts (default 0)      |
| `stallTimeout` | `int`      | Kill if no output for N seconds |

Notable: stall detection (byte-level, handles `\r` progress), auto-retry with backoff, streaming output.

---

## Go Dependencies → Rust Equivalents

| Go Dependency            | Used By             | Rust Equivalent                | Status              |
| ------------------------ | ------------------- | ------------------------------ | ------------------- |
| `disintegration/imaging` | image               | `image` v0.25                  | Migrated            |
| `kolesa-team/go-webp`    | image (WebP)        | `image` WebP feature           | Migrated (lossless) |
| `xuri/excelize/v2`       | spreadsheet (Excel) | `calamine` + `rust_xlsxwriter` | Not migrated        |
| `expr-lang/expr`         | transform, loop     | JS evaluator TBD               | Not migrated        |
| `text/template` (stdlib) | edit-fields         | JS template TBD                | Not migrated        |
| `os/exec` (stdlib)       | shell-command       | `std::process::Command`        | Server/desktop only |
| `net/http` (stdlib)      | http-request        | `reqwest`                      | Server only         |
| `encoding/csv` (stdlib)  | spreadsheet         | `csv` crate                    | Migrated            |

---

## Open Decisions

- **M4 cloud execution:** Go API on Railway (existing) vs Rust compiled service. No decision yet.
- **Expression evaluation in browser:** Which JS library replaces `expr-lang/expr`?
- **Template engine for browser:** Go `text/template` syntax vs Mustache/Handlebars.
