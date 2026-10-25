# Decision: file-copy runs as a batch and refuses a copy over a file the batch reads

**Created:** October 25, 2026
**Status:** Decided with Ryan while fixing ISS-52 (branch `fix/file-copy-unread-input`). He chose option 3 below. After the second cold review he chose to ship with the two gaps it found logged as follow-ups (ISS-58: loop `{{item.*}}`; ISS-60: the time limit), and to keep `{{file}}` unfilled as before.
**Applies to:** `engine/crates/bnto-file/src/copy.rs`, `copy_batch/`, `copy_newer.rs` and `readers.rs`, and the CLI's `ordered_write/readers.rs`, which now shares the lookup.

## Context

ISS-52: `F/a.txt` = `AAAA`, `F/b.txt` = `BBBB`, a `file-rename` (`a` → `b`) then `file-copy` into `F` with `conflict: overwrite`. `BBBB` was lost. `newer` lost it the same way when `a` was the newer file. The issue suggested reusing the output node's ordered write (#760, `ordered_write/`), which waits to replace a file until the writes that read it have run.

Two things rule that out for `file-copy`:

- **Ordering can't save the repro.** Both files come out named `b.txt`, so both copies target `F/b.txt`. Whatever the order, one replaces the other. The output node avoids this because `output_names` gives the second file `b_1.txt`. `file-copy` has no such names: `overwrite` means replace.
- **Steps after `file-copy` read the file too.** The node passes each file on unchanged, so `b`'s data is still a reference to `F/b.txt`. Once `a` is copied over it, every later step reads `AAAA` as `b`, whatever order the copies ran in.

`file-copy` was a per-file node, so it couldn't see the rest of the batch. Under auto-iteration (`implicit-iteration.md`), each file even went through rename → copy before the next file was renamed.

## Options

1. **Order the copies (the issue's first suggestion).** Ruled out, for the two reasons above.
2. **Batch, and drop per-file `destination`.** `file-copy` becomes a batch node, so the executor resolves its params once. `{{item.*}}` in `destination` outside a loop then fails, as it does for other batch nodes. Nothing in the repo or Ryan's recipes uses it.
3. **Batch, and keep per-file `destination` (chosen).** As 2, but the node fills `{{item.*}}` in its params itself, once per file, from the file's metadata (`resolves_items`, with `bnto_core::control::placeholder`'s resolver, made public for this). That's what the executor did for it as a per-file node. `{{file}}` isn't filled: the executor never filled it for a per-file node either, so it stays as written, as before.

## Decision

Option 3:

- **Plan, then copy.** `process_batch` fills each file's params, then `refuse_replacing_read_files` checks every copy before any runs, and only then runs the per-file copy (`process`) on each file in order.
- **What the executor did for a per-file node, the node now does itself** (`copy_batch/`): fill `{{item.*}}` (a value that is one whole placeholder keeps its JSON type, so `createDirs: "{{item.flag}}"` still reads a boolean), give each output file the metadata it came in with under the node's own (so `{{item.*}}` after `file-copy` still reads what earlier steps attached), and hand the files copied before a failure to the run's closing steps.
- **What's refused.** A copy that replaces an existing file (`overwrite` onto a file that isn't the copy's own source, or `newer` onto an older one) is refused when that file is another file's source in the same batch: the files reaching the step together. Inside an explicit `loop`, that's one iteration's files, so the repro inside a loop still lost `BBBB` (ISS-59). [file-copy-held-files.md](file-copy-held-files.md) later closed that gap: a copy is also refused over a file another iteration or a later branch still reads. The order doesn't matter: a reader whose copy already ran still passes the file on. The refusal names each destination, what would be copied over it, and the file read from it, then suggests another folder, `skip` or `rename`. A dry run is refused the same way, like `file-move`.
- **Not refused.** `skip` and `rename` never replace a file. A file already at its destination is left as is (ISS-36). Two files of the batch copied onto one new name with `overwrite` still end with the later one: that replaces a copy this run made, not a user's file.
- **One lookup, two writers.** Finding which files are read from a destination (stamp first, then identity, so links and case-only spellings count) moved from `ordered_write/readers.rs` to `bnto_file::readers`, so the output node and `file-copy` share it, as they already share `bnto_file::relocate`.
- **Counts.** `newer` mode's `copied` / `up_to_date` / `skipped_newer` totals are added up by the node (`combined`), since a batch returns one node-level record.

## Trade-offs

- `file-copy` is now an auto-iteration barrier. Every file goes through the steps before it, and then all are copied. The files, names and metadata that come out are the same, but a long recipe copies nothing until every file has reached the copy.
- Progress is reported per batch, as for other batch nodes, not per file.
- **The run's time limit wasn't checked between copies (closed since by ISS-60).** The executor checks it before each file of a per-file node only, and a node can't see it. After the limit, the batch keeps starting copies until the CLI's watchdog ends the process (`run_watchdog.rs`, about 2 s later). A copy cut off then can leave its destination truncated: `overwrite` copies with `std::fs::copy`, and `newer` truncates the target and writes in place. A per-file node never starts a copy after the limit, though a single long copy could be cut off the same way. Logged as ISS-60, which later closed the gap: a batch node's context answers `check_time_limit`, and `file-copy` asks it before each copy (see engine-execution.md's [Run Time Limit](../strategy/engine-execution.md#run-time-limit-settingstimeout)).
- **Some overwrites a user means are refused.** The check can't tell an intended replacement from a lost input: a recursive `file-list` over a folder that holds the destination, rerun with `overwrite`, is refused because the old copy is one of the inputs. Copying into a folder outside the inputs, or `skip`/`rename`, avoids it.
- A param a file's `{{item.*}}` can't fill fails without the executor's `Node '<id>':` prefix. The CLI names the step (`FAILED cp:`) anyway.
- **Inside a loop**, the executor still resolves a batch node's `{{item.*}}` against the loop's item (the file's metadata when the iteration started) before the node sees it. A key that a step inside the iteration added to the file, before `file-copy`, used to fill and now fails with the executor's "has no value" error. Logged as ISS-58.
- **Planning cost.** Every destination is stat'ed once (`is_file`). An existing one is also compared with its own source (`is_at`, which opens both), whatever the mode. Only when some copy would replace a file does `readers_of` stat that destination again and every source. A copy into an empty folder costs one stat per file.
- **A value can be filled twice.** The executor fills `{{fields.*}}`, `{{env.*}}` and `{{node.*}}` first. If such a value itself contains `{{item.KEY}}`, the node then fills that too, where the per-file path never re-scans a filled value. Every node that fills items itself does the same (ISS-35 covers the general fix).
- **Counts with a per-file `conflict`.** With `conflict: "{{item.mode}}"` varying by file, a non-`newer` file that came in with no metadata gets the batch's totals from the executor (it falls back to the node's record for a file without metadata), where a per-file node gave it nothing.
