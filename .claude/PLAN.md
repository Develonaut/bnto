# Bnto — Build Plan

**Last Updated:** April 9, 2026
**This is the active build plan.** For completed sprint history, see [PLAN-HISTORY.md](PLAN-HISTORY.md). For backlog, tabled, and frozen items, see [PLAN-BACKLOG.md](PLAN-BACKLOG.md).

---

## How This Works

Tasks are organized into **sprints** (features) and **waves** (dependency groups within a sprint). All tasks in a wave can be picked up in parallel by agents. Waves must complete in order before the next wave starts.

```
- [ ]              → available, grab it
- [ ] **CLAIMED**  → an agent is working on this, pick something else
- [x]              → done
```

**Scope rule:** Each task targets ONE package. Don't touch files outside the tagged package unless the task explicitly says so.

**Branching:** Feature branches target `main` directly. Create a branch from `main` (`git checkout -b <type>/<short-description> main`), do the work, PR into `main`, squash merge.

---

## Current State

**CLI is the product.** `cargo install bnto` gets you 15 recipes. The web is a landing page.

- **v0.5.0 released (April 2026):** 15 recipes, video-download node (yt-dlp), dependency system, ProcessContext, `bnto list/info/run/doctor` commands. Published to crates.io
- **Engine (Rust):** Library crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-video, bnto-engine), WASM entry point (bnto-wasm), CLI binary (bnto)
- **CLI/TUI-first pivot (April 2026):** Web reduced to landing page. Editor frozen. Auth stripped. Focus: engine, CLI, TUI
- **Next: TUI (Sprint 10)** — `bnto tui` via ratatui + crossterm
- **Open source (MIT):** Monetization tabled. Focus on engine power and community traction
- **Homepage complete (April 2026):** Developer-facing landing page with Motorways animations, kawaii sushi mascots, code editor section, recipe showcase marquee
- **Frozen:** Editor, auth, premium features, frontend investment — maintained but not actively developed

**Revenue:** Tabled (April 2026). CLI is free, open-source. Revenue strategy revisited when the tool has community traction.

---

## What's Next

**Sprint 9 complete.** CLI is solid: 15 recipes, dependency system, video node, `bnto list/info/run/doctor`. v0.5.0 shipped to crates.io.

**Next up: TUI (Sprint 10).** `bnto tui` launches an interactive terminal UI via ratatui + crossterm — recipe browser, file picker, progress display, results panel. Same engine, richer interface.

**After TUI:** More node types (Excel, PDF, shell), recipe expansion. Desktop (Tauri) and monetization are deep backlog. See [engine-expansion.md](strategy/engine-expansion.md).

---

## Sprint 10: TUI — ACTIVE

**Goal:** `bnto tui` launches an interactive terminal UI — recipe browser, file picker, progress display, results panel. Same engine, richer interface than raw CLI.

**Framework:** `ratatui` + `crossterm`

**Dependencies:** `ratatui`, `crossterm` added to `bnto` Cargo.toml

**Persona ownership:**

| Package  | Persona        |
| -------- | -------------- |
| `engine` | `/rust-expert` |

### Wave 1 (parallel — foundation)

- [ ] `engine/crates/bnto` — `/rust-expert` — TUI module scaffolding (`src/tui/`). App struct, event loop, terminal setup/teardown
- [ ] `engine/crates/bnto` — `/rust-expert` — Recipe browser panel (list all recipes with categories, search/filter, selection)
- [ ] `engine/crates/bnto` — `/rust-expert` — Basic navigation (tab between panels, keyboard shortcuts, help overlay, quit)

### Wave 2 (parallel — execution flow)

- [ ] `engine/crates/bnto` — `/rust-expert` — File picker panel (browse filesystem, multi-select files for recipe input)
- [ ] `engine/crates/bnto` — `/rust-expert` — Progress display (per-file progress bars, node status, live update during execution)
- [ ] `engine/crates/bnto` — `/rust-expert` — Recipe config editing (param overrides in TUI before execution)

### Wave 3 (sequential — polish + test)

- [ ] `engine/crates/bnto` — `/rust-expert` — Results panel (output files, sizes, timing, open-in-finder/copy-path)
- [ ] `engine/crates/bnto` — `/rust-expert` — Integration tests for TUI mode (headless terminal testing)
- [ ] `engine/crates/bnto` — `/rust-expert` — `bnto tui` documentation + README update

---

## Reference

| Document                                                         | Purpose                                                                        |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| [PLAN-HISTORY.md](PLAN-HISTORY.md)                               | Completed sprint history (Phase 0 through Sprint 9, Homepage)                  |
| [PLAN-BACKLOG.md](PLAN-BACKLOG.md)                               | Backlog, tabled sprints, frozen items                                          |
| `.claude/strategy/bntos.md`                                      | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers                 |
| `.claude/strategy/engine-execution.md`                           | Engine execution architecture — pipeline executor, progress events             |
| `.claude/strategy/cloud-desktop-strategy.md`                     | Architecture, cost analysis, cloud execution topology                          |
| `.claude/strategy/core-principles.md`                            | Trust commitments, key principles                                              |
| `.claude/strategy/expression-input-ux.md`                        | Expression input UX (frozen — reference for future)                            |
| `.claude/rules/`                                                 | Auto-loaded rules (architecture, code-standards, engine-node-patterns, etc.)   |
| `.claude/skills/`                                                | Agent skills (pickup, project-manager, code-review, pre-commit)                |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Pricing strategy, revenue projections, SEO monetization (historical — on hold) |
