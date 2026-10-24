# Decision: A made-up output name skips files already there

**Created:** October 24, 2026
**Status:** Decided while fixing the data-loss bug below (branch `fix/suffix-skips-existing-files`), with cold reviews on October 24, 2026: two of the whole branch, then one each of its third and fourth commits. Option 2 was chosen over the "skip anything there" fix the bug report suggested.
**Applies to:** `engine/crates/bnto/src/output_names.rs` and `output_names/folder.rs`, and every caller: the CLI's final write and dry run (`io.rs`), progressive copies (`progressive_output.rs`, from `bnto run`'s reporter and the TUI bridge's), and the TUI results listing (`tui/bridge/output.rs`).

## Context

Write mode keeps two results with one name apart by writing the later one as `stem_1.ext`, `stem_2.ext`… (`engine/crates/bnto/src/output_names.rs`). The names were kept apart from each other only, so the made-up `b_1.txt` replaced a user's own `b_1.txt` in the output folder. That was data loss, found in the ISS-48 review and fixed before it was logged. Write mode's contract is that only a file named like an output is replaced; a name bnto made up must replace nothing.

## Options

1. **Skip any name already in the folder.** Simple, but two things break:
   - A progressive loop copies files out before the final write. The final write then finds the loop's own `b_1.txt` and moves on to `b_2.txt`, leaving two copies. Fixing that means passing the loop's state to the final write, through the CLI reporter, `RunTarget`, `write_output`, `write_results_with_mode` and the TUI's bridge.
   - Each rerun of a recipe with clashing names adds another file (`b_2`, `b_3`…).
2. **Skip any name already in the folder, unless a regular file by exactly that name already holds exactly the result's bytes** (chosen). That file is the result already, so it's left as is. The loop's copy and the final write then agree with no shared state, and a rerun with the same output writes the same files.

## Decision

Option 2, in `output_names/folder.rs` (`spot` returns `Free`, `Held` or `Taken`):

- Only a suffixed name is checked; a result's own name still replaces its namesake, even over the same bytes.
- **Held means left as is.** A held file isn't written again: `io.rs` skips it and counts it with the files "already there, left as is" (as it does for a source written back to its own folder, ISS-36), the dry run says it would be left, and progressive output skips it too. The rename warning names held files on their own line (`already there under a new name, holding the same bytes, left as is`), so it agrees with the summary. Rewriting it would have given it the source's mode and a new mtime, and copied a large file again after comparing it. The TUI lists a held file like a written one, since it is this run's result.
- An entry whose name differs only in case counts as taken, and so do a folder and a link. Only a regular file is ever held. It's found with `symlink_metadata`, so a link isn't followed.
- **The listing alone isn't trusted.** When it shows nothing under the name, `symlink_metadata` is asked about the path itself:
  - A file it finds is held or taken like a listed one. That catches one file the filesystem knows under a spelling the listing doesn't match (APFS takes NFC and NFD `é` as one name), and a folder bnto can write but not list.
  - Any error, `NotFound` or not, counts as free. For `NotFound` that's plain. Any other error (no search permission on the folder) means creating the file fails the same way, so nothing is replaced and the write reports the error. Treating those errors as taken would make every number taken, and the claim would never end.
- **Checked once per claim, with one cache.** Each made-up name is checked against the folder when it's tried. A name a file there takes is remembered in that run's `OutputNames` (`occupied`), so later claims move past it without listing the folder or looking it up again. A folder with many clashing names (`b_1` through `b_100`) is listed about once per name, not once per name per claim. Free and held answers aren't cached: they're claimed, and a claimed name is never tried again.
- Sizes are compared first, then bytes a chunk at a time, so no large file is read whole.
- **In the TUI, the progressive copies are made on the bridge thread by its reporter**, as `bnto run`'s reporter does, not on the UI thread when it drains events. Names now depend on what's in the folder, so a copy still being made while the bridge plans the listing and the final write could shift a name (`b_2` to `b_3`), and the listing and the write would disagree. A side effect is that, like `bnto run`, the TUI no longer makes progressive copies in overwrite mode.

## Trade-offs

- A suffix check may read a same-sized file already there, once per run per name thanks to the `occupied` cache. It happens only when names clash and that exact suffixed name exists.
- A dry run gives the same names a real run would, as long as the dry run produces the same bytes. `bnto run --dry-run` runs every step, shell steps included, so it does unless a step's output changes from run to run.
- A rerun whose output changed leaves the old suffixed file and writes the next number. bnto can't tell its own earlier output from a user's file, so it keeps both.
- A progressive run's suffixed copies are already there when the final write runs, so it leaves them and reports them as "already there, left as is", although this run made them. The final write can't tell them from an earlier run's identical files without the loop's state, which is what option 1 would have needed.
- If steps after a progressive loop change the loop's files, the final write's bytes differ from the loop's copies, so the final write skips the loop's suffixed copies and they're left behind. Before, they were replaced. `progressive_output.rs::a_step_after_the_loop_leaves_the_loops_suffixed_copy` pins this.
- The `occupied` cache means a file that takes a name keeps it taken for the rest of the run, even if it's deleted mid-run. That can only send a later result to a higher number.
- The run's own names fold case only, not Unicode normalization. Two results named `café.txt` in NFC and in NFD aren't kept apart, so on APFS the second one replaces the first. That predates this change, and fixing it needs a Unicode normalization dependency.
