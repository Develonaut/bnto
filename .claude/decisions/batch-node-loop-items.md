# A Batch Node Fills Its Own `{{item.*}}`, in a Loop Too

**Date:** October 29, 2026 · **Issues:** ISS-58, ISS-35 (its loop-item part)

## Problem

Five batch nodes fill `{{item.*}}` (and `{{file}}`) themselves, once per file: `report`, `assert`, `edit-fields`, `file-render` and `file-copy` (`NodeProcessor::resolves_items`). Outside a loop the executor handed those to them as written. Inside a loop it filled them first, from the loop's item, which caused two bugs:

- **ISS-58:** the loop's item is the file's metadata when the iteration started (`build_loop_context`). A key that a step inside the iteration adds (`file-inspect`'s `extension`, an `edit-fields` value) was refused with "has no value" before the node ran. A per-file node reads it fine: its item is the file's current metadata with the loop's item on top (`file_item`).
- **ISS-35:** the executor filled a loop value into the template, and the node then filled the template again. A CSV cell `{{file}}` read through `{{item.note}}` printed the file's name inside a loop and `{{file}}` outside one.

## Decision

- **The node fills them, inside a loop too.** For these nodes the executor leaves `{{item.*}}` as written in a loop as well. The node reads the loop's item from `ProcessContext::loop_item()`, which the run's batch context (`executor/run_scope.rs`) answers, like `held_elsewhere`.
- **One fill, shared.** `bnto_core::control::placeholder` fills `{{file}}`, `{{item.KEY}}`, `{{entry}}` and `{{entry.KEY}}` in one pass over the recipe's template (`Scope`, `fill_text`, `fill_value`, built on the executor's `try_interpolate`). A filled value is never scanned again, so a value that reads like a token comes out as written. All five nodes use it; `file-render` adds its `{{count}}` and escaping through `fill_with`.
- **Two scopes.**
  - **Per file:** anything a node fills once per file. It reads the file's metadata with the loop's item on top (`placeholder::file_item`). That's the same merge and the same precedence as a per-file node: the loop's item wins a shared key.
  - **Once:** templates written once per node call (`file-render`'s `header`, `footer` and `filename`; `report`'s `each` and its entry lines) read the loop's item only, as the executor used to fill them. `{{file}}` there is still refused, and so is `{{item.*}}` outside a loop.
- **`shell-run` opts out** (`resolves_loop_items` returns false). It fills items only inside `each` groups; elsewhere the executor still fills the loop's item, and SEC-27's `reserved_tokens` still refuses a value that carries one.
- **A dry run fills too.** `report` prints each filled line and `assert` each filled check, so a missing key fails a dry run as it used to inside a loop. A `report` whose `each` isn't a list yet (its upstream step didn't run) prints the message as written.

## What changes for a recipe

- A key neither the file nor the loop's item has fails during the node, naming the file and every key it has, instead of before the node starts.
- A `report` message or `assert` value that reads only loop keys now runs once per file of the iteration, not once per iteration. This matches what `{{item.*}}` means outside a loop. An iteration that ends with no files prints nothing and checks nothing.
- `file-render` escapes a row value filled from the loop's item (the executor filled it unescaped before). Header and footer values still go in unescaped.
- `bnto check` no longer refuses an entry line that reads `{{item.*}}`: `validate` can't tell whether the node sits in a loop. The run refuses it outside one.

## Alternatives

- **Fill only the loop's keys in the executor** (ISS-58's own suggestion). That fixes ISS-58 but keeps the second fill, so ISS-35 stays.
- **Refuse a value that carries a node's token** (SEC-27's `check_reserved`, applied to every node). That blocks real data such as a cell that documents a template, and grows a list node by node.

## Left open

- `{{fields.*}}`, `{{env.*}}` and `{{node.*}}` values are still filled by the executor first, so one carrying `{{file}}` or `{{item.*}}` is filled again by these nodes (ISS-35). The fix is for the node to fill every namespace in its one pass; it belongs with the template-filters plan's unified interpolator.
- Inside an iteration the loop's item still wins over a value a step set (`edit-fields` setting `status` over a row's `status`), for per-file and batch nodes alike (ISS-109).
- `when` guards read only the loop's item (`executor/guard.rs`), so they can't see a key added during the iteration (ISS-110).
