# Inside a Loop, `{{item.*}}` Reads the File's Current Metadata

**Date:** October 31, 2026 · **Issues:** ISS-109, ISS-110 · **Decided by:** Ryan (the precedence, October 30), `/workflow-expert` review (the name and the guard rule)

## Problem

Inside a loop iteration, `{{item.KEY}}` reads the file's metadata with the loop's item on top (`placeholder::file_item`). The loop's item is the iteration's file metadata as the iteration started (`build_loop_context`), so it wins a key both have:

- **ISS-109:** a step inside the iteration that changes a key the row already had (`edit-fields` setting `status: done` over a row's `status: pending`, `file-inspect` refreshing `size` after a compress) is invisible: a later `{{item.status}}` still reads `pending`.
- **ISS-110:** a `when` guard is decided against the loop's item alone (`executor/guard.rs`), so it can't see any key the iteration added, though a per-file step's params can.

## Decision

- **`{{item.KEY}}` reads the file's current metadata, with the loop's item underneath.** The file wins a key both have, so a step's change is seen. A key only the loop's item has still fills, so a file a step made inside the iteration (no row columns of its own) still reads the row. One merge, `placeholder::file_item`, used by per-file nodes and by the nodes that fill their own tokens.
- **`{{loop.KEY}}` reads the loop's item as the iteration started,** whatever later steps did. That's the row, for a loop over CSV rows.
  - **Name:** `loop` names where the value comes from, as `node` and `entry` do. `row` would be wrong for a loop over files, and `iteration` reads as an index or a count.
  - **Nested loops:** the innermost loop, as `{{item.*}}` already does (a nested loop replaces the outer loop's item). No `{{loop.parent.*}}` until a recipe needs it.
  - **Outside a loop:** refused, naming the step, like an `{{item.KEY}}` with no value.
  - **Templates written once per call** (`file-render`'s header, footer and filename; `report`'s `each` and entry lines) have no file, so their `{{item.*}}` stays the loop's item, and they read `{{loop.*}}` too.
- **A `when` guard inside a loop reads the item a per-file step would,** from the files the step is about to get:
  - **One file** (the usual case: the iteration's own file): that file's item.
  - **No files:** the loop's item.
  - **Several files** (a step fanned out): a key the guard reads must have one value across them. Files a step made inside the iteration lack the row's columns, so a guard on a row column still reads the row's value through the merge. Files that disagree fail the step, naming the key and two of the values, and pointing at `{{loop.KEY}}` or `file-filter`. A guard decides one thing for a whole step; picking one file's value would be a silent guess.

**Built in two PRs.** The first (ISS-109) flips the merge and adds `{{loop.*}}`. The second (ISS-110) applies the guard rule above, and the same rule to a batch step the executor fills itself (`summarize`, `file-join`, `file-sort`, `file-list`, `file-move`, `file-delete`, `spreadsheet-merge`, `spreadsheet-export`), which until then fills `{{item.*}}` from the loop's item alone. `shell-run` is left out: inside a loop the executor fills its `each` groups from the loop's item too, so the step rule would refuse a fan-out its groups should read per file (ISS-111): a step's change reaches per-file steps and the nodes that fill their own tokens after the first PR, and those after the second.

## What changes for a recipe

- A key a step inside the iteration changed now reads the new value through `{{item.*}}`. Before, the row's value won. To keep reading the row, use `{{loop.KEY}}`.
- A file a step made inside the iteration, whose own metadata shares a key with the row (a row column named `size` or `filename`, with `file-list` or `file-inspect` inside the loop), now reads its own value for that key.
- A `when` guard inside a loop sees keys the iteration added or changed. A guard over several files that disagree fails the step instead of reading the row.
- No bundled, dogfood or personal-library recipe changes output: the only recipe that reads `{{item.*}}` inside a loop (`bulk-video-download`, in Ryan's library) sets no key inside its loop. `CHANGELOG.md` isn't started ([changelog.md](../strategy/changelog.md)); the PR's description carries the change until it is.

## Alternatives

- **Keep the loop's item winning** (today). A step's change stays invisible inside the iteration, and guards can't see added keys.
- **The file wins, no new namespace.** Simpler, but a recipe whose row column shares a name with a key a step sets loses the row's value with no way back.
- **A guard per file** (skip the step for some files). That's `file-filter`'s job; a guard skips a whole step.
- **A guard over several files reads the loop's item.** Silently ignores what the step's files say, which is ISS-110 again for fan-out.

## Left open

- `{{loop.parent.*}}` for an outer loop's item in nested loops: no recipe needs it yet.
