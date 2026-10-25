# Decision (open): how a `file-copy` sees the files the run still reads outside its batch (ISS-59)

**Created:** October 25, 2026
**Status:** Options for Ryan to choose from. Nothing is implemented. Recommendation: option 1.
**Applies to:** `engine/crates/bnto-core/src/executor/` (`container.rs`, `flow.rs`, `primitive.rs`, `mod.rs`), `bnto-core/src/context.rs`, `bnto-file/src/copy_batch/refusal.rs`, `bnto-file/src/readers.rs`. Related: ISS-60 (time limit), ISS-58 (loop `{{item.*}}`), [file-copy-batch.md](file-copy-batch.md).

## The defect, reproduced

Built `bnto` from `main` (`12617b3f`) and ran each recipe in a sandbox (temp `HOME` and `BNTO_HOME`, folders under the scratchpad). `F/a.txt` = `AAAA`, `F/b.txt` = `BBBB`. `ren` is `file-rename` `a` → `b`; `cp` is `file-copy` into `F` with `conflict: overwrite`.

| #   | Recipe                                                                                    | Result                                                                                          |
| --- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| 1   | flat: `ren`, `cp` (control)                                                               | Refused, nothing copied. `b.txt` = `BBBB`                                                       |
| 2   | `loop { ren, cp }` on `a`, `b` (the ISS-59 repro)                                         | exit 0, **`b.txt` = `AAAA`**                                                                    |
| 3   | same, files given as `b`, `a`                                                             | exit 0, **`b.txt` = `AAAA`**. A finished iteration's file is exposed too, not only later ones   |
| 4   | `loop { ren, cp }` with `conflict: newer`, `b` older                                      | exit 0, **`b.txt` = `AAAA`**                                                                    |
| 5   | as 3, then output write to `O`                                                            | exit 0, **`O/b.txt` = `AAAA`, `O/b_1.txt` = `AAAA`**: the run's own result for `b` is wrong too |
| 6   | **no loop**: edges `input → filter(a.txt) → ren → cp` and `input → output` (write to `O`) | exit 0, **`F/b.txt` = `AAAA`, `O/b.txt` = `AAAA`**                                              |

Row 6 is new: the same loss happens **without a loop**. A graph level holds files for a branch that hasn't run yet (`executor/routed.rs`, fan-out shares `FileData::Path`), and a branch that filtered those files out copies over them. The cause is the same: a batch node sees only the files handed to it, not the ones the run still holds elsewhere.

Rows 3 and 5 show the set to protect: a finished iteration's file still reads from its source. `file-copy` passes each file on unchanged, so `b`'s result is still a reference to `F/b.txt`. Whatever replaces `F/b.txt` later changes what every step after the loop, and the CLI's output write, read as `b`.

### Other nodes

Every engine node that writes to disk was checked (`grep` for `fs::write|copy|rename|remove_file|write_to|copy_to` in `bnto-*/src`). Only `file-copy`, `file-move` and `file-delete` write the user's files. `shell-run` can do anything and is out of scope.

| #   | Recipe                                                     | Result                                                                                                              |
| --- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| 7   | flat `file-move` into `O`, `a.txt` given twice             | Refused before anything moved ("in the run more than once")                                                         |
| 8   | same in a loop                                             | Iteration 1 moved `a.txt`. Iteration 2 failed "`F/a.txt` doesn't exist". Partial run, nothing lost                  |
| 9   | `loop { ren, file-move into F }`                           | Refused: a move never replaces a file (`relocate::conflicts`)                                                       |
| 10  | `loop { file-copy to O (rename), file-delete }`, `a` twice | Iteration 2's copy failed: iteration 1 had deleted the source. Partial run, nothing lost beyond what was asked      |
| 11  | same, flat                                                 | Deleted once, the second counted as missing                                                                         |
| 12  | `file-delete` (flat or loop), then output write            | Both fail at the output write. The same in both shapes, and already documented in engine-execution.md's "Watch for" |

**`file-move` and `file-delete` can't lose another iteration's bytes.** `file-move` refuses any move onto an existing file, and `file-delete` only deletes the file it was handed. Their loop gap is the same file arriving twice (duplicate arguments, fan-out into a loop, a link). The batch handles that up front, while the loop fails partway with a missing file. That is a worse message, but nothing is lost. So `file-copy` (`overwrite` and `newer`) is the only data-loss case. CLI progressive loop output already leaves occupied destinations to the final write (ISS-48, `bnto/src/progressive_output.rs`).

Side finding (row 10): a `file-copy` whose source is gone fails with "Failed to write `O/a_1.txt`: No such file or directory". The message blames the destination (`copy.rs:144`). Not logged yet.

### What must still be allowed

`loop { image-compress, file-rename "-compressed" → "", file-copy into F overwrite }` on `F/x.jpg`, `F/y.jpg` compresses both in place today, flat and in a loop (sizes 176764 → 77588 and 4851 → 1942). The copy replaces its own file's original, whose bytes the run has already turned into `Bytes`. Nothing reads the original any more, so a fix must not refuse it.

## What the executor holds outside a batch node's call

A node in a chain gets every file its level holds, so it needs no more. Files sit outside the node's call in two places:

- **Loop** (`container.rs::execute_loop`): the files not yet iterated, and the files finished iterations output (`all_output_files`).
- **Graph level** (`flow.rs::run_graph`, `routed.rs`): files held for consumers that run later (fan-out clones, other branches' inputs).

Both nest: a loop inside a branch inside a loop. Auto-iteration needs nothing extra: batch nodes and containers are its barriers, so they always get the whole level.

## Options

### 1. The executor tracks the paths it holds elsewhere; `file-copy` asks through `ProcessContext` (recommended)

**How it works.**

- **The check.** A new default method on `ProcessContext`, `held_elsewhere(&self, path: &Path) -> Option<String>`. It returns the name of a file the run still reads from `path`, outside the files handed to this node. The default is `None`, for the browser and test doubles.
- **The index.** The executor keeps a chain of "held" frames on `PipelineContext`, one per loop or graph level. Each frame is an index of the `FileData::Path`s that level holds outside the running step:
  - **Loop frame.** At the start, every loop file. When iteration `i` starts, its own file comes out: it's in the iteration's hands now, so an in-place copy over it stays allowed. When the iteration ends, its outputs go in.
  - **Graph frame.** Before each segment, the paths still in `Routed` for later consumers.
- **The lookup.** The index is keyed by stamp (size and mtime, as `bnto_file::readers` does) and confirmed with `FileData::is_at`, so links and case-only spellings count. It is built lazily on the first query, so a loop with no `file-copy` costs nothing.
- **The wrapper.** `primitive.rs::process_batch` hands the node a `RunScope` instead of the consumer's context. `RunScope` forwards every `ProcessContext` method and answers `held_elsewhere` from the frames.
- **The refusal.** `copy_batch/refusal.rs` adds a check after its batch check: a copy that would replace a file (`replaces()`) whose destination is `held_elsewhere` is refused, naming the file that still reads it.

**Files.**

- `bnto-core`: `context.rs` (one default method), new `executor/held.rs` (frames and the index), new `executor/run_scope.rs` (the forwarding wrapper), `executor/mod.rs` (a field on `PipelineContext`, passed on by `layer()`), `container.rs` (the loop frame), `flow.rs` (the graph frame), `primitive.rs` (wrap the context for batch nodes).
- `Stamp`/`stamp` move from `bnto-file/src/readers.rs` to `bnto-core`, since the core can't depend on `bnto-file`. `readers.rs` imports them back.
- `bnto-file/src/copy_batch/refusal.rs`.

**Size.** About 250–300 production lines and 250 test lines over about 10 files. One PR, one concern: "a copy refuses to replace a file the run still reads".

**What it fixes.**

- **ISS-59:** both iteration orders, `newer`, nested loops, and row 6 (fan-out), which isn't logged yet.
- **ISS-60:** the same `RunScope` is where the run's deadline goes. Add `time_is_up(&self) -> Result<(), BntoError>` (default `Ok`) answered from `ctx.time_limit` and `live_clock`, and have `file-copy`/`file-move`/`file-delete` check it before each file. That's a small second PR on this one.
- **ISS-58:** not fixed. It's param resolution (`resolve_node_params`), not something a node asks the run for. Its own fix (fill only the loop's keys for a `resolves_items` node) is independent and can land in any order.
- **Later:** `file-move` and `file-delete` could also ask `held_elsewhere` and refuse rows 8 and 10 up front instead of failing partway. That's optional, since nothing is lost there.

**Risks.**

- **The refusal is no longer atomic inside a loop.** Earlier iterations have already copied when iteration `i` is refused, so the message can't say "nothing copied". Flat runs keep that wording. In a loop, the refusal should say "this iteration copied nothing". `onError: "continue"` skips just that iteration, and nothing is lost either way.
- **Forwarding.** `RunScope` must forward every `ProcessContext` method. If a new method with a default is added to the trait later and not to `RunScope`, nodes silently get the default. Mitigate with a test double that overrides every method, plus a test asserting `RunScope` returns each override, and a note on the trait.
- **False refusals.** Few. Only a destination the run still reads is refused, which is exactly the loss. A finished iteration's output is counted as read even when nothing after the loop reads it (say, `output: none`). That matches what a flat batch does today.
- **Cost.** One `stat` per path when a frame's index is first built, then one per output added. Nothing for a run with no `file-copy` that would replace a file. A lookup is one stat of the destination plus `is_at` on the stamp matches.
- **Stale stamps.** A held file changed in place mid-run by `shell-run`, and then reached through a hard link, could slip past the stamp lookup. Rare. A path-key lookup alongside the stamp catches the plain-path case.
- **WASM.** None. The browser has no `FileData::Path`, so the frames stay empty and `file-copy` is CLI-only. `RunScope` wraps `NoopContext` the same way.

**Tests (written Red first).**

- **Executor**, in `executor/tests`, with a fake batch node that records what `held_elsewhere` answers for given paths:
  - in a loop it sees pending and finished iterations' paths, not its own iteration's file;
  - in a graph it sees another branch's held files;
  - in a flat chain it sees nothing;
  - a nested loop sees both frames.
- **`file-copy` units**, with a context whose `held_elsewhere` names a path: `overwrite` and `newer` onto that path are refused, while `skip`, `rename` and a copy onto its own source are not.
- **CLI**, in `bnto/tests/write_into_source_folder.rs`:
  - rows 2 to 4 and 6 keep `BBBB` and fail with the refusal;
  - the in-place compress loop above still runs. That's the guard against over-refusal.
- **`RunScope`:** the forwarding test.

### 2. Nodes declare the files they'd replace; the executor refuses

**How it works.**

- A new `NodeProcessor` method, `replaced_files(&self, input: &BatchInput, ctx) -> Result<Vec<Replaced>, BntoError>` (default empty), returns each path the node would replace and which file would replace it.
- Before `process_batch`, the executor checks those paths against the batch's own sources and against the same held frames as option 1. It refuses with one message format for every node.
- `file-copy`'s `refusal.rs` and the `readers` lookup move into `bnto-core`.

**Files.** Option 1's executor files (without `run_scope.rs`), plus:

- `processor.rs`;
- a new `executor/replacements.rs`;
- `copy_batch/` (implement `replaced_files` and drop `refusal.rs`);
- `readers.rs` moved to the core;
- the CLI's `ordered_write/readers.rs` imports.

**Size.** About 1.5–2 times option 1: around 450 production lines over about 14 files.

**What it fixes.**

- ISS-59 and row 6, the same as option 1. One policy covers any future node that replaces files.
- It doesn't serve ISS-60: a deadline still needs a channel to the node, so you'd add option 1's `ProcessContext` method anyway.
- It doesn't fix ISS-58.

**Risks.**

- The node plans twice: fill each file's params and compute destinations in `replaced_files`, then again in `process_batch`. Caching that between calls is awkward.
- Only `file-copy` would use it. `file-move` never replaces a file, and the CLI output write isn't a node. That's an abstraction with one user (YAGNI).
- The CLI's `ordered_write` imports change.

**Tests.** Option 1's, with the refusal asserted from the executor (a fake node declaring a replaced path).

### 3. Plan a loop's batch nodes over the whole loop first

**How it works.** Inside a loop, a writing batch child becomes a barrier, the way it is under auto-iteration:

1. Run every iteration's steps up to it.
2. Run the node once over every iteration's files.
3. Resume each iteration after it.

**Files.** A rewrite of `execute_loop` and how loop results are merged. About 500+ lines.

**What it fixes.**

- ISS-59, with an atomic "nothing copied" refusal.
- Not row 6 (fan-out), not ISS-60 and not ISS-58. The batch node would also need a loop item per file, which makes ISS-58 harder.

**Risks.** It changes what a loop means:

- iterations stop being independent;
- `{{item.*}}`, `{{node.*}}` isolation, `onError: "continue"`, `IterationStarted`/`Completed` events and progressive output all need redefining.

A dry-run planning pass instead can't work: a step whose output a later step names (a `shell-run` that makes files) can't be predicted.

**Tests.** Every loop test would need revisiting. **Not recommended.**

### 4. Refuse an `overwrite`/`newer` copy onto any file the run has read

**How it works.**

- One run-wide, append-only index of every `Path` that entered the run: the root input, then every node's outputs, recorded in `execute_node`. It's exposed the same way as option 1.
- **Variant 4b:** the CLI's `NativeContext` reports only its argument files, with no executor change.

**Size.** About 120–150 production lines (4b: about 40).

**What it fixes.** Rows 2 to 6 (4b misses any file that entered through `file-list`, which is common). ISS-60 is helped only if the same wrapper is built. It doesn't fix ISS-58.

**Risks.** False refusals, including in flat runs that work today:

- **In place.** The compress-in-place recipe above would be refused: `x.jpg` was read as an input, though the run has already replaced its bytes. That's a regression for flat runs, not just loops.
- **Filtered out.** A file listed and then dropped (say, a recursive `file-list` with a `file-filter` that excludes `backup/`, then a copy into `backup/` with `overwrite`) would be refused. Nothing reads it any more.

**Tests.** As option 1, plus the in-place guard, which would fail by design. **Not recommended.**

## Recommendation

**Option 1.** It protects exactly the files the run still reads, so the in-place case keeps working. It fixes the fan-out case as well as the loop. Its `RunScope` is also the channel ISS-60 needs. It stays inside the executor and `file-copy`, with nothing for the browser to change.

Order:

1. **PR 1, ISS-59:** the held frames, `RunScope` and `held_elsewhere`, used by `file-copy`. Update ISS-59 in `ISSUES.md` to cover the fan-out case, and update the "Watch for" note in engine-execution.md.
2. **PR 2, ISS-60:** `time_is_up` on `RunScope`, checked by `file-copy`, `file-move` and `file-delete` before each file.
3. **ISS-58:** independent; any time.

Decide with the PR 1 plan whether `file-move`/`file-delete` should also refuse a source `held_elsewhere` (rows 8 and 10). Nothing is lost there today, so it can wait.
