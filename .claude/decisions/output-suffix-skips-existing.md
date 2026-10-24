# Decision: A made-up output name skips files already there (October 2026)

## Context

Write mode keeps two results with one name apart by writing the later one as `stem_1.ext`, `stem_2.ext`… (`engine/crates/bnto/src/output_names.rs`). The names were kept apart from each other only, so the made-up `b_1.txt` replaced a user's own `b_1.txt` in the output folder: data loss, found in the ISS-48 review and fixed before it was logged. Write mode's contract is that only a file named like an output is replaced; a name bnto made up must replace nothing.

## Options

1. **Skip any name already in the folder.** Simple, but two things break:
   - A progressive loop copies files out before the final write. The final write then finds the loop's own `b_1.txt` and moves on to `b_2.txt`, leaving two copies. Fixing that means passing the loop's state to the final write, through the CLI reporter, `RunTarget`, `write_output`, `write_results_with_mode` and the TUI's bridge thread.
   - Each rerun of a recipe with clashing names adds another file (`b_2`, `b_3`…).
2. **Skip any name already in the folder, unless that exact name already holds exactly the bytes about to be written** (chosen). Writing those bytes again loses nothing. The loop's copy and the final write then agree with no shared state, and a rerun with the same output writes the same files.

## Decision

Option 2, in `output_names/folder.rs`:

- Only a suffixed name is checked; a result's own name still replaces its namesake.
- An entry whose name differs only in case counts as there, and so does a folder or a link. Only a regular file is ever taken, found with `symlink_metadata`, so a link isn't followed.
- The listing alone isn't trusted. When it shows nothing under the name, `symlink_metadata` on the path must also say `NotFound`. That catches one file the filesystem knows under a spelling the listing doesn't match (APFS takes NFC and NFD `é` as one name), and a folder bnto can write but not list. If that lookup finds a file, the same-bytes rule applies to it.
- A lookup that fails any other way (no search permission on the folder) counts as free: creating the file fails the same way, so nothing is replaced, and the write reports the error. Treating it as taken instead would make every number taken and the claim would never end.
- The folder is checked on every claim, never cached.
- Sizes are compared first, then bytes a chunk at a time, so no large file is read whole.
- In the TUI, the progressive copies are made on the bridge thread by its reporter, as `bnto run`'s reporter does, not on the UI thread when it drains events. Names now depend on what's in the folder, so a copy still being made while the bridge plans the listing and the final write could shift a name (`b_2` to `b_3`), and the listing and the write would disagree. A side effect is that, like `bnto run`, the TUI no longer makes progressive copies in overwrite mode.

## Trade-offs

- A suffix check may read a same-sized file already there. This only happens when names clash and the exact suffixed name exists, which is rare.
- A dry run gives the same names a real run would, as long as the dry run produces the same bytes. `bnto run --dry-run` runs every step, shell steps included, so it does unless a step's output changes from run to run.
- A rerun whose output changed leaves the old suffixed file and writes the next number. bnto can't tell its own earlier output from a user's file, so it keeps both.
- If steps after a progressive loop change the loop's files, the final write's bytes differ from the loop's copies, so the final write skips the loop's suffixed copies and they're left behind. Before, they were replaced. `progressive_output.rs::a_step_after_the_loop_leaves_the_loops_suffixed_copy` pins this.
- The run's own names fold case only, not Unicode normalization. Two results named `café.txt` in NFC and in NFD aren't kept apart, so on APFS the second one replaces the first. That predates this change, and fixing it needs a Unicode normalization dependency.
