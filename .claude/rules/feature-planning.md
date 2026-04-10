# Feature Planning: Multi-PR Plan Format

When a feature spans multiple PRs, agents MUST produce a structured plan document before writing any code. The plan is the design phase -- it forces you to think through the full dependency chain, file impact, test strategy, and count changes before touching a single line.

---

## When This Applies

Use this format when ANY of these are true:

- Feature touches more than one crate or package
- Work will be split across 2+ PRs
- Changes ripple through the codegen pipeline (engine -> snapshot -> TypeScript)
- Multiple test count registries need updating

**When it doesn't apply:** Single-file fixes, one-PR features with obvious scope, pure research tasks.

---

## Plan Structure

Every multi-PR plan follows this exact structure. No sections are optional.

### 1. Phase Header

Top of the plan. One-paragraph context: what, why, how many PRs.

```markdown
# Phase N: Feature Name -- N PRs

## Context

What this feature does, why it exists, which strategy doc it implements.
Link to the strategy doc.

**What changes:** Concrete impact summary -- processor counts, node type counts,
recipe counts. Call out what does NOT change to prevent false assumptions.

Split into N sequential PRs -- each describable in one sentence,
each independently reviewable and mergeable.
```

**The "what changes" line is critical.** It anchors the entire plan against the current state. Reviewers can immediately see the blast radius.

### 2. Per-PR Sections

Each PR gets its own section with these subsections in order:

```markdown
## PR N: One-sentence description

**Branch:** `<type>/<short-description>` from `main`
**One sentence:** Describe the PR in exactly one sentence without "and."
```

#### Required subsections (in order)

| Subsection             | What it contains                                                                                  |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| **What**               | 2-3 sentences: what this PR delivers and why it's a separate PR                                   |
| **Files**              | New and modified files with approximate counts. Group by new/modified                             |
| **Key function / API** | Code signature or data structure that defines the PR's contract. Pseudo-code or Rust/TS signature |
| **RED tests**          | Bullet list of failing tests to write BEFORE implementation. These define the acceptance criteria |
| **Verification**       | Exact shell commands to run. Copy-pasteable                                                       |
| **Count changes**      | Which test count registries change (or explicitly "no count changes")                             |

#### Optional subsections

| Subsection              | When to include                                                       |
| ----------------------- | --------------------------------------------------------------------- |
| **Process flow change** | When the PR modifies an existing pipeline or data flow                |
| **Codegen**             | When the PR triggers the codegen pipeline                             |
| **Web overlays**        | When the PR adds entries to web-facing registries (recipes, nav, SEO) |
| **Test count updates**  | Table of specific test files and what changes (for count-heavy PRs)   |

### 3. Dependency Chain

End of the plan. ASCII diagram showing PR ordering and dependencies.

```markdown
## Dependency Chain

PR 1 (description)
|-- PR 2 (description) -- depends on PR 1
|-- PR 3 (description) -- depends on PR 2
```

---

## Example: Per-PR Section

```markdown
## PR 1: Add `bnto-vector` SVG rasterization crate

**Branch:** `feat/bnto-vector-crate` from `main`
**One sentence:** Add a pure Rust SVG rasterization library using resvg/usvg/tiny-skia.

### What

New crate `engine/crates/bnto-vector/` -- a focused SVG->pixels library
with zero coupling to the image processing pipeline.

### Files (~6 new, ~1 modified)

**New:**

- `test-fixtures/images/small.svg` -- 100x100 SVG test fixture
- `engine/crates/bnto-vector/Cargo.toml` -- deps: resvg, usvg, tiny-skia
- `engine/crates/bnto-vector/src/lib.rs` -- pub mod + error enum
- `engine/crates/bnto-vector/src/rasterize.rs` -- rasterize_svg() function

**Modified:**

- `engine/Cargo.toml` -- add workspace member

### Key function

pub fn rasterize_svg(svg_bytes: &[u8], options: RasterizeOptions)
-> Result<Pixmap, VectorError>

### RED tests (write first)

- Default DPI (96) -> 100x100 output from 100x100 SVG
- Custom DPI (192) -> 200x200 output (2x scale)
- Invalid SVG -> VectorError::ParseError
- Empty data -> error

### Verification

cargo test -p bnto-vector
task wasm:lint

### No count changes

No processor, node type, or recipe counts change.
```

---

## Rules

1. **Plan before code.** The plan document must exist and be approved before any implementation begins. The plan IS the design phase.

2. **One sentence per PR.** If you can't describe the PR in one sentence without "and," split it further. This is the same test from pre-commit.md's PR sizing rules, applied at planning time.

3. **RED tests are acceptance criteria.** The test bullets in the plan define what "done" looks like. When all RED tests turn green, the PR is complete. Don't add scope during implementation.

4. **Count changes are explicit.** Every plan must state which test count registries change. "No count changes" is a valid and important statement -- it prevents agents from accidentally bumping counts that shouldn't change.

5. **Files are enumerated.** List every new and modified file. Approximate counts (`~6 new, ~3 modified`) in the header, specific paths in the body. This makes the blast radius reviewable before any code exists.

6. **Verification is copy-pasteable.** The verification section contains exact commands. No prose like "run the tests." Exact commands: `cargo test -p bnto-vector`, `task wasm:lint`.

7. **Dependency chain is explicit.** If PR 2 depends on PR 1, say so. If PRs can be parallel, say so. The chain determines merge order and prevents agents from starting work on a blocked PR.

8. **What changes vs what doesn't.** The phase header must state both. "Processor count stays 11" is as important as "Recipe count goes 15 -> 17." False assumptions about unchanged counts cause cascading test failures.

---

## Anti-Patterns

| Anti-Pattern                                | Fix                                                                 |
| ------------------------------------------- | ------------------------------------------------------------------- |
| Vague scope: "Add SVG support"              | Concrete: "Add `bnto-vector` crate with `rasterize_svg()` function" |
| No file list: "Modify some engine files"    | Enumerate: "~6 new, ~1 modified" with paths                         |
| Tests as afterthought: "Add tests"          | RED tests as acceptance criteria with specific assertions           |
| Missing count changes                       | Explicit: "Node type count: 20 -> 20 (unchanged)"                   |
| Monolith PR: one PR for everything          | Split by concern: crate, integration, recipes                       |
| Prose verification: "Make sure it compiles" | Commands: `task wasm:lint && task wasm:test`                        |
| No dependency chain                         | Always state: "PR 2 depends on PR 1" or "PRs 1-2 are parallel"      |

---

## Relationship to Other Rules

- **[pre-commit.md](pre-commit.md#pr-sizing--single-concern-prs-mandatory)** -- PR sizing rules apply to each planned PR. The planning format ensures compliance upfront.
- **[code-standards.md](code-standards.md)** -- TDD Red-first applies within each PR's RED tests section.
- **[engine-node-patterns.md](engine-node-patterns.md)** -- The checklists there define WHAT must happen. This format defines HOW to plan it across PRs.
