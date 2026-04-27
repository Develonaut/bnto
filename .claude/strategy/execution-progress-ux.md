# Execution Progress UX — Design Spike

**Last Updated:** April 27, 2026
**Status:** Strategy doc (no code)
**Scope:** CLI (`bnto run`) and TUI execution screen progress display

---

## Problem Statement

Recipe execution currently works but doesn't feel alive. The CLI shows a basic indicatif progress bar and step counter. The TUI shows node/file markers with an elapsed timer. Both are functional but don't communicate _what's happening_ — throughput, time remaining, current file name, or whether the process is healthy or stalled.

Long-running recipes like `download-video` (10+ minutes) are the worst case: a mostly-static display with occasional stderr output. The user can't tell if it's progressing, stalled, or about to finish.

**Goal:** Define a phased plan to make execution feel responsive, informative, and visually polished — inspired by the best terminal UX in the ecosystem.

---

## Competitive Audit

### Claude Code CLI

**Pattern:** Whimsical verb spinner + elapsed timer + token counter.

```
✳ Perambulating… (3m 7s · ↓ 4.6k tokens)
```

- Rotating Unicode spinner character (✳ and variants)
- Playful, rotating verbs ("Cogitating...", "Embellishing...", "Perambulating...")
- Elapsed time in parentheses, always visible
- Downstream token count as throughput metric
- Inline progress updates ("still thinking", "thinking more", "almost done")
- Single-line, non-intrusive — doesn't scroll the terminal

**Takeaway:** The verb spinner + elapsed time pattern makes waiting feel intentional. The throughput metric (tokens) gives a sense of velocity even when the task has no deterministic completion percentage.

### cargo (Rust package manager)

**Pattern:** Verb + package name + progress bar + count.

```
   Compiling bnto-core v0.12.0
   Compiling bnto-image v0.12.0 (4/12)
    Finished `release` profile [optimized] target(s) in 45.23s
```

- Colored verb prefix (`Compiling`, `Downloading`, `Finished`) — left-aligned, verb in green
- Package name + version as context
- Count format: `(N/M)` for parallel downloads
- Summary line with total duration on completion
- Download phase has a separate progress bar with bytes/total

**Takeaway:** The colored verb prefix is extremely readable. Each line communicates exactly one thing: what's happening and to what. The `(N/M)` pattern is simple and universally understood.

### docker pull

**Pattern:** Per-layer parallel progress bars.

```
5f70bf18a086: Pull complete
3e9f4d0b5c12: Downloading  45.2MB/123.4MB
a1234567890b: Extracting   [====>     ]  12.3MB/45.6MB
```

- Per-layer ID prefix (short hash)
- Status verb per layer (Downloading, Extracting, Pull complete)
- Bytes downloaded/total with progress bar
- Multiple layers update in parallel (ANSI cursor repositioning)
- No overall progress or ETA

**Takeaway:** The per-item parallel display works for Docker's layer model but is overkill for sequential pipelines like bnto. The bytes/total format is useful. The lack of overall progress is a well-known UX complaint (GitHub issue #4022, 2014).

### Charm Bubbles (Go TUI framework)

**Pattern:** Configurable spinner + customizable progress bar.

**Spinner:**

- Multiple built-in frame sets (dots, line, minidots, jump, pulse, points, globe, moon, monkey, meter, hamburger)
- Custom frame arrays supported
- Configurable tick rate

**Progress bar:**

- Solid and gradient fill styles
- Customizable empty/filled rune characters
- Optional percentage readout
- Optional animation via Harmonica (spring physics)
- `progress.New(progress.WithDefaultGradient())` for gradient fills

**Takeaway:** The breadth of spinner frame sets is useful as a reference. The gradient fill is visually distinctive. Bnto should pick 1-2 spinner styles that match the brand, not offer a spinner gallery.

### pnpm / npm

**Pattern:** Package count + progress bar + current package name.

```
Progress: resolved 847, reused 845, downloaded 2, added 847, done
```

- Resolution phase: running count of packages resolved
- Download phase: progress bar with bytes
- Final summary: resolved/reused/downloaded/added counts

**Takeaway:** The summary line with multiple metrics is compact and informative. The "reused" count communicates cache efficiency — analogous to showing "skipped" files in bnto.

---

## Unicode Indicator Inventory

### Spinner Characters

Candidates for bnto's brand:

| Set              | Frames       | Feel                          |
| ---------------- | ------------ | ----------------------------- |
| **Braille dots** | `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏` | Technical, developer-friendly |
| **Box drawing**  | `◐◓◑◒`       | Clean, geometric              |
| **Simple dots**  | `⠁⠂⠄⡀⢀⠠⠐⠈`   | Minimal, subtle               |
| **Arrows**       | `←↖↑↗→↘↓↙`   | Directional, dynamic          |
| **Circle fill**  | `◴◷◶◵`       | Smooth, modern                |
| **Aesthetic**    | `✶✸✹✺✹✸`     | Playful, matches bnto brand   |

**Recommendation:** Braille dots for the CLI (technical feel, widely supported). Circle fill for the TUI (cleaner at small sizes). Aesthetic star variants for special states (completion).

### Progress Bar Characters

| Style                   | Empty | Filled | Cursor | Example       |
| ----------------------- | ----- | ------ | ------ | ------------- |
| **Current (indicatif)** | `─`   | `━`    | `╸`    | `━━━━━╸─────` |
| **Block fill**          | `░`   | `█`    | `▓`    | `█████▓░░░░░` |
| **Thin line**           | `·`   | `─`    | `›`    | `─────›·····` |
| **Arrow**               | ` `   | `=`    | `>`    | `=====>     ` |
| **Smooth**              | `░`   | `▓`    | `▒`    | `▓▓▓▓▒░░░░░`  |

**Recommendation:** Keep the current `━╸─` style for CLI (consistent with existing output). Consider block fill `█▓░` for TUI where the higher resolution is visible.

### Status Markers (TUI)

Current TUI markers are good. Possible refinements:

| Status    | Current | Alternative    | Notes                          |
| --------- | ------- | -------------- | ------------------------------ |
| Pending   | `○`     | `○`            | Keep — universal hollow circle |
| Active    | `◉`     | `◉` or spinner | Spinner would show activity    |
| Completed | `●`     | `✓` or `●`     | Checkmark is more explicit     |
| Failed    | `✗`     | `✗`            | Keep — clear error signal      |

---

## Metrics Design

### What to Surface

| Metric                | When                      | Format                             | Source                                             |
| --------------------- | ------------------------- | ---------------------------------- | -------------------------------------------------- |
| **Elapsed time**      | Always during execution   | `0.3s`, `1m 23s`, `12m 5s`         | Wall clock (already tracked in TUI `elapsed_ms`)   |
| **File count**        | During node execution     | `3/10 files`                       | `FileProgress.file_index` / `total_files`          |
| **Node count**        | Always                    | `[1/3]` step N of M                | `NodeStarted.node_index` / `total_nodes`           |
| **Current file**      | During processing         | `photo.jpg`                        | `FileProgress.message` (already contains filename) |
| **Node type**         | During node execution     | `image-compress`                   | `NodeStarted.node_type`                            |
| **Per-node duration** | After node completes      | `done 10 files in 2.3s`            | `NodeCompleted.duration_ms` (already shown in CLI) |
| **Throughput**        | Large file batches (10+)  | `4.2 files/s`                      | Computed: `files_processed / elapsed_per_node`     |
| **ETA**               | When throughput is stable | `~8s remaining`                    | Computed: `(total - done) / throughput`            |
| **Output bytes**      | For size-aware recipes    | `12.4 MB → 3.1 MB (75% reduction)` | Requires new event (see Architecture)              |

### What NOT to Surface

- **Per-file byte progress** — Too granular. Files process in sub-second for most recipes. Only useful for `download-video` where yt-dlp already streams its own progress via `CommandOutput`.
- **CPU/memory usage** — Not actionable for the user. Belongs in `--verbose` diagnostics.
- **Estimated total time** — Unreliable for heterogeneous pipelines (a 3-step recipe where step 1 takes 0.1s and step 2 takes 30s). Only show ETA within a single node's file batch where throughput is uniform.

---

## Layout Mockups

### CLI: `bnto run` — Enhanced Single-Line Progress

**Current output:**

```
  Pipeline: 2 steps
  [1/2] image-compress
        ━━━━━━━━━━━━━━━╸───── 3/10 Compressing photo.jpg...
        done 10 files in 2.3s
  [2/2] file-rename
        ━━━━━━━━━━━━━━━━━━━━ 5/5 Renaming batch_001.jpg...
        done 5 files in 0.1s
```

**Proposed output (Phase 1 — spinner + elapsed):**

```
  Pipeline: 2 steps

  [1/2] image-compress
        ━━━━━━━━━━━━━━━╸───── 3/10 photo.jpg (1.2s)
        done 10 files in 2.3s

  [2/2] file-rename
        ━━━━━━━━━━━━━━━━━━━━ 5/5 batch_001.jpg
        done 5 files in 0.1s

  Completed 10 files in 2.4s
```

Changes from current:

- Elapsed time per-node in parentheses on the progress line
- Summary line at the end with total files and total duration
- Blank line between steps for readability

**Proposed output (Phase 2 — throughput + summary):**

```
  Pipeline: 2 steps

  [1/2] image-compress
        ━━━━━━━━━━━━━━━╸───── 3/10 photo.jpg (1.2s · 4.2 files/s)
        done 10 files in 2.3s

  [2/2] file-rename
        ━━━━━━━━━━━━━━━━━━━━ 5/5 batch_001.jpg
        done 5 files in 0.1s

  Completed 10 files in 2.4s (12.4 MB → 3.1 MB, 75% smaller)
```

Changes from Phase 1:

- Throughput metric after elapsed time (only shown when batch >= 5 files)
- Size summary on completion line (requires `FileSummary` event — see Architecture)

**Proposed output (shell-command / long-running):**

```
  Pipeline: 1 step

  [1/1] shell-command (download-video)
        ⠙ Downloading… (2m 14s)
        [download]  67.2% of ~350MiB at 4.12MiB/s ETA 01:23
        done 1 file in 5m 32s

  Completed 1 file in 5m 32s
```

For shell-command recipes without deterministic file counts, replace the progress bar with a spinner + elapsed timer. `CommandOutput` lines from the child process (yt-dlp's own progress) display below the spinner.

**Narrow terminal degradation (< 60 cols):**

```
  [1/2] image-compress
        ━━━━━━━━━━╸──── 3/10
        done 10 in 2.3s
```

Truncate filename and throughput. Keep the bar, count, and duration.

### TUI: Execution Screen — Enhanced Layout

**Current layout:**

```
┌─ Execution ─────────────────────────────┐
│  Running  0.3s                          │
│                                         │
│  NODES                                  │
│  ◉ image-compress                       │
│  ○ file-rename                          │
│                                         │
│  FILES                                  │
│  ● photo.jpg 100%                       │
│  ◉ photo2.jpg 45%                       │
│  ○ photo3.jpg                           │
│                                         │
│  OUTPUT                                 │
│  [download] 34.2% of ~150MiB           │
└─────────────────────────────────────────┘
```

**Proposed layout (Phase 1 — inline progress bars + elapsed per node):**

```
┌─ Execution ─────────────────────────────┐
│  Running  12.3s                         │
│                                         │
│  STEPS                                  │
│  ◉ image-compress  3/10  ━━━━━╸──── 1.2s│
│  ○ file-rename                          │
│                                         │
│  FILES                                  │
│  ● photo.jpg                            │
│  ◉ photo2.jpg 45%                       │
│  ○ photo3.jpg                           │
│                                         │
│  OUTPUT                                 │
│  [download] 34.2% of ~150MiB           │
└─────────────────────────────────────────┘
```

Changes from current:

- "NODES" renamed to "STEPS" (user-facing language)
- Inline mini progress bar per node (compact `━━╸──` style)
- File count `3/10` next to active node
- Per-node elapsed time right-aligned

**Proposed layout (Phase 2 — throughput + animated spinner):**

```
┌─ Execution ─────────────────────────────┐
│  Running  12.3s                         │
│                                         │
│  STEPS                                  │
│  ⠙ image-compress  3/10  ━━━━━╸── 4.2/s│
│  ○ file-rename                          │
│                                         │
│  FILES                                  │
│  ● photo.jpg                            │
│  ◉ photo2.jpg 45%                       │
│  ○ photo3.jpg                           │
│                                         │
│  OUTPUT                                 │
│  [download] 34.2% of ~150MiB           │
└─────────────────────────────────────────┘
```

Changes from Phase 1:

- Braille spinner replacing `◉` for active node (animated on tick)
- Throughput metric right-aligned (files/s)
- Spinner rotates on each tick (200ms interval, already exists for TUI timer)

---

## Architecture Review

### What Exists Today

**Two-tier progress system:**

1. **Per-node progress** (`bnto-core/src/progress.rs`): `ProgressReporter` — closure-based, percent + message. Used by individual processors (image-compress, csv-clean, etc.). Fine-grained, per-file.

2. **Pipeline-level progress** (`bnto-core/src/events.rs`): `PipelineReporter` — 8 event types (`PipelineStarted`, `NodeStarted`, `FileProgress`, `NodeCompleted`, `NodeFailed`, `PipelineCompleted`, `PipelineFailed`, `CommandOutput`). Used by the executor.

**CLI rendering** (`bnto/src/progress.rs`): `stderr_reporter()` — matches on `PipelineEvent`, uses indicatif for progress bars, colored output to stderr.

**TUI rendering** (`bnto/src/tui/render_execution.rs`): `draw_execution()` — fixed header (status, nodes, files, errors) + scrolling command output. Uses `ExecutionModel` state struct.

### What's Missing

| Gap                               | Severity | Phase   |
| --------------------------------- | -------- | ------- |
| **No pipeline summary event**     | Medium   | Phase 1 |
| **No per-file output size**       | Low      | Phase 2 |
| **No throughput computation**     | Low      | Phase 2 |
| **No animated spinner in CLI**    | Low      | Phase 1 |
| **No spinner in TUI active node** | Low      | Phase 2 |

### New Event Types Needed

**Phase 1 — None.** All Phase 1 improvements use existing events. The CLI already receives `PipelineCompleted.duration_ms` and `PipelineCompleted.total_files_processed` — it just doesn't print a summary line.

**Phase 2 — One new event:**

```rust
/// Emitted after PipelineCompleted with aggregate statistics.
/// Optional — only emitted when size data is available.
#[serde(rename_all = "camelCase")]
PipelineSummary {
    total_input_bytes: u64,
    total_output_bytes: u64,
}
```

This requires processors to report output sizes. Currently, `NodeProcessor::process()` returns `ProcessResult` with output bytes but this data isn't aggregated into events. The executor would need to accumulate sizes and emit the summary.

**Alternative to a new event:** Compute the summary in the CLI/TUI rendering layer by inspecting output files on disk after completion. This avoids changing the event protocol but only works for CLI (not WASM/browser). Given that progress UX is CLI/TUI-focused, this is the simpler path.

### Rendering Architecture

**No changes to the event system in Phase 1.** All improvements are in the rendering layer:

- `bnto/src/progress.rs` — CLI rendering changes (summary line, elapsed per node, spinner for indeterminate nodes)
- `bnto/src/tui/render_execution.rs` — TUI rendering changes (inline progress bars, throughput, spinner animation)

**Phase 2** adds throughput computation in the rendering layer. The renderer tracks `Instant::now()` when `NodeStarted` is received and computes `files/elapsed` on each `FileProgress` tick. No engine changes needed — this is pure presentation logic.

---

## Phased Scope Recommendation

### Phase 1: Polish What Exists (single PR, ~3-5 tests)

**Scope:** Improve the CLI and TUI rendering with zero engine changes. All data is already available in existing events.

**CLI changes (`bnto/src/progress.rs`):**

- Add blank lines between steps for readability
- Show elapsed time per progress line: `3/10 photo.jpg (1.2s)`
- Add completion summary line: `Completed 10 files in 2.4s`
- For indeterminate shell-command nodes (no file count), use a braille spinner + elapsed instead of a progress bar

**TUI changes (`bnto/src/tui/render_execution.rs`):**

- Rename "NODES" header to "STEPS"
- Add inline file count next to active node: `◉ image-compress  3/10`
- Add per-node elapsed time (right-aligned)

**Tests:**

- `test_cli_summary_line_on_completion`
- `test_cli_elapsed_per_node`
- `test_tui_inline_file_count`
- `test_tui_per_node_elapsed`
- `test_cli_spinner_for_indeterminate_node`

**Why this first:** Zero risk, zero engine changes, immediate visual improvement. Unblocks user feedback on whether more metrics are desired.

### Phase 2: Throughput + Animated Indicators (~3-4 tests)

**Scope:** Add computed metrics and animated indicators. Still no engine event changes.

**CLI changes:**

- Throughput metric: `4.2 files/s` appended to progress line when batch >= 5 files
- Braille spinner animation for shell-command nodes (requires `indicatif::ProgressBar` spinner mode)

**TUI changes:**

- Animated braille spinner for active node marker (replaces static `◉`)
- Throughput metric right-aligned per active node
- ETA for large batches (10+ files, computed from throughput)

**Tests:**

- `test_throughput_computation`
- `test_throughput_hidden_for_small_batches`
- `test_eta_computation`
- `test_spinner_frame_rotation`

### Phase 3: Size-Aware Summary (depends on engine change)

**Scope:** Show input/output size comparison on completion. Requires the executor to aggregate output sizes.

**Engine changes (`bnto-core`):**

- Add `PipelineSummary` event (or compute from output files in CLI renderer)
- Accumulate output sizes in executor

**CLI changes:**

- Summary line: `Completed 10 files in 2.4s (12.4 MB → 3.1 MB, 75% smaller)`

**TUI changes:**

- Summary section on completion screen with size comparison

**Alternative:** Skip the engine event and compute sizes from output files on disk in the CLI renderer. Simpler, CLI-only.

---

## Decisions to Make Before Implementation

1. **Spinner style** — Braille dots (`⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏`) or circle fill (`◴◷◶◵`)? Braille is more widely supported across terminal emulators.

2. **"STEPS" vs "NODES" in TUI** — "Steps" is more user-friendly. "Nodes" is the internal term. The CLI already says "steps" (`Pipeline: 2 steps`). Recommend "STEPS" for consistency.

3. **Throughput threshold** — Show throughput only when batch >= N files. Recommendation: N=5. Showing "1.0 files/s" for a 2-file batch is noise.

4. **ETA reliability** — ETA is only meaningful when throughput is stable (same file types, similar sizes). Should we show it at all, or only after N files have completed to establish a baseline? Recommendation: show after 3+ files completed, hide if variance > 50%.

5. **Phase 3 approach** — New `PipelineSummary` event (clean, cross-platform) vs compute-from-disk (simpler, CLI-only)? Recommendation: compute-from-disk first, upgrade to event if browser needs it.

---

## References

- Bnto CLI progress: `engine/crates/bnto/src/progress.rs`
- Bnto TUI execution: `engine/crates/bnto/src/tui/render_execution.rs`
- Bnto pipeline events: `engine/crates/bnto-core/src/events.rs`
- Bnto per-node progress: `engine/crates/bnto-core/src/progress.rs`
- [Evil Martians — CLI UX Best Practices: 3 Patterns for Improving Progress Displays](https://evilmartians.com/chronicles/cli-ux-best-practices-3-patterns-for-improving-progress-displays)
- [Charm Bubbles — TUI components](https://github.com/charmbracelet/bubbles)
- [indicatif — Rust progress bar crate](https://crates.io/crates/indicatif)
- [Docker pull progress UX issue #4022](https://github.com/moby/moby/issues/4022)
