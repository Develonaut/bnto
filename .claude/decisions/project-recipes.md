# Decision: Project recipes in `.bnto/recipes/`

**Created:** November 3, 2026
**Status:** Agreed with Ryan on November 3, 2026, after a `/security-engineer` review the same day (4 blocking, 7 should-fix, 9 notes, all taken in: see [Security review](#security-review-november-3-2026)). Ryan settled four questions (below) and chose to ship the work as one feature branch and one PR to `main`, released as a new version ([Delivery](#delivery)). Implements the "Project recipes" section of [dogfooding.md](../strategy/dogfooding.md#project-recipes-7a-7j), and changes its earlier sketch in two places (marked **Changed**).
**Request:** [2026-11-03-project-recipes.md](../requests/2026-11-03-project-recipes.md).
**Applies to:** in `engine/crates/bnto/src/`: `catalog.rs`, `recipe_source.rs`, `inspect.rs`, `recipe_output_dir.rs`, `install_plan.rs`, `trust.rs`, `trust_gate.rs`, `consent.rs`, `consent_lines.rs`, `install.rs`, `install_trust.rs`, `package_manager.rs`, `main.rs`, `list.rs`, `info.rs`, `doctor.rs`, `run.rs`, `dry_run/`, `context.rs`, `storage/paths.rs`, `storage/mod.rs`; the TUI's `app/init.rs`, `bridge.rs`, `bridge/precheck.rs`, `render_detail.rs`, `screens/home.rs`, `render_home_panes.rs`, `render_home_grid.rs`, `footer_hints.rs`, `app_helpers/` (`consent.rs`, `editor.rs`, `navigation.rs`, `library.rs`); `bnto-core/src/editor/io.rs`; `bnto-file`'s `list/`, `delete/`, `move/`, `copy.rs` and `copy_batch/`; the test helpers in `bnto/tests/`.

## Context

A user wants bnto as a project's task runner: `cd` into a repo, run `bnto tui`, and see that repo's own recipes beside their library and the built-ins. Today bnto loads recipes from two places only, the 22 built-ins and `~/.bnto/recipes/` (`RecipeCatalog::load`, `catalog.rs`). A project recipe runs only by path (`bnto run .bnto/recipes/x.bnto.json`), and the TUI opens one by path (`bnto tui <file>`) with no list to pick from.

bnto's own repo is the first user: its eight dogfood recipes live in `recipes/`, each run by path from Taskfile tasks, and six of them say "run it from the repo root".

### What a run sees today (measured November 3, 2026, bnto 0.19.0)

An example project with three recipes in `.bnto/recipes/` (image resize to WebP, CSV to JSON, a `grep` shell step), sandboxed `HOME` and `BNTO_HOME`:

- From the project root, all three work by path: 2 WebP files in `public/images/`, `products.json` in `public/data/`, the TODO lines printed. `bnto check .bnto/recipes/` passes.
- From `src/`, the image recipe lists `assets/raw` under `src/`, finds it missing, writes 0 files, **exits 0**, and leaves an empty `src/public/images/`.

So listing project recipes isn't enough: run from a subfolder, they quietly do nothing, or the wrong thing.

### What a recipe can reach today

The working folder confines nothing. `file-list` takes an absolute `base` and uses it as written, though its docs say absolute paths and `..` are refused (`bnto-file/src/list/mod.rs` `base_dir`, `list/params.rs`). `file-delete` resolves its `base` and `file-move` its `destination` against the process's current folder, not `ctx.work_dir()` (`delete/target.rs` `base_root`, `move/root.rs`, `move/plan.rs`), and the CLI keeps the output folder as a relative string (`run.rs`). So a no-shell recipe can list files anywhere and delete or move them. For a recipe the user wrote, that's their choice. For one a cloned repo ships, **the consent prompt is the only protection**, and it has to show enough to be informed consent.

## Decision

### 1. Layout and loading

```
my-project/
├── .git/
├── .bnto/
│   └── recipes/
│       ├── optimize-images.bnto.json
│       └── ...
└── ...
```

A dot-folder, like `.github/workflows`, so a repo's unrelated `recipes/` folder is never picked up. The rest of `.bnto/` is reserved. No project `config.toml` until there's a concrete need (YAGNI).

Git stores symlinks, so a cloned repo controls what each name points at. A project file loads only when:

- `.bnto` and `.bnto/recipes` are real folders, not symlinks;
- it sits directly in `.bnto/recipes/`, ends in `.bnto.json`, and is a regular file, opened without following a symlink (a link to `/dev/zero` would read forever, a FIFO would hang, and either would hang **every** `bnto` command in the repo, since `main.rs` loads the catalog for all of them);
- it's at most 1 MB, and among the first 200 files read.

Anything else is skipped with a reason that `bnto list` and `doctor` show.

### 2. Discovery — **Changed:** git isn't required

Walk up from the current folder. The **project root** is the first folder holding a `.bnto/recipes/` that passes section 1. The walk stops, loading nothing past it:

- at the first folder holding `.git` (a dir, or a file for worktrees and submodules), after checking that folder itself;
- before the user's home folder, compared after canonicalizing (case, `/var` vs `/private/var`, a trailing slash). Home is never a project root;
- at a candidate whose `.bnto/recipes` canonicalizes to the library folder, or whose `.bnto` is the bnto home;
- on Unix, at a folder or `.bnto` not owned by the current user, or a world-writable folder without the sticky bit. This is git's `safe.directory` lesson (CVE-2022-24765, `C:\.git`): without the `.git` requirement, a walk from `/tmp/x` or a USB stick would otherwise reach a `/tmp/.bnto` any local user can plant. On Windows, the walk never checks a drive root (`C:\.bnto`);
- on any error other than NotFound (permissions, a dead mount);
- at the filesystem root.

No `git` process is spawned. The dogfooding plan required a git repo; this allows a plain folder too, while the `.git` stop keeps a repo nested in another from reaching its parent's recipes.

Discovery runs once, in `main.rs` and at TUI launch, and its result is passed to `RecipeCatalog::load`. The catalog never looks at the current folder itself, so unit tests that load it from inside this repo don't pick up its recipes.

**`BNTO_HOME` and the config's `paths.home` must be absolute.** A repo's `.bnto/` has the bnto home's layout, and today a relative `BNTO_HOME=.bnto` makes a repo's `state/trusted.json` the trust store and its recipes library recipes (`storage/paths.rs` `resolve`, confirmed by the review). bnto refuses a relative or empty one, and the relative `.bnto` fallbacks go: `storage::ensure_ready` (`storage/mod.rs`, run by every command from `main.rs`, which creates folders and runs migrations there) and the TUI's (`tui/app/init.rs`).

### 3. Precedence and slugs: a project never shadows

A project recipe whose slug matches a built-in or a library recipe is **skipped**, and `bnto list`, `bnto doctor` and a run of that slug say so, naming the file. Otherwise a cloned repo could ship `compress-images.bnto.json` with `file-delete` steps. The user renames the project file to fix it.

Slugs from a project are strict, so a near-match can't pass for a built-in:

- the `.bnto.json` suffix is stripped once (today `trim_end_matches` strips it repeatedly, so `x.bnto.json.bnto.json` is also `x`);
- the slug must be ASCII `[a-z0-9-]`: no `.` (a project `recipe.json.bnto.json` would take over `bnto run recipe.json` aimed at a local file, since a slug beats a path in `recipe_source.rs`), no capitals, no Unicode lookalikes;
- two project files with one slug are both skipped.

### 4. Consent: every project recipe asks first, and the prompt says what it does

A project recipe asks for consent on its first run **even without shell steps**: `file-move`, `file-delete` and overwrite-mode output change files with no shell at all. Approval is pinned to the definition's SHA-256 in the global `trusted.json`, as library shell recipes are today (`trust.rs`), so a `git pull` that changes a recipe asks again.

The source kind (`Bundled`, `Library`, `Project`, `Path`) is decided from how the recipe was resolved, never from the argument string (today `consent.rs` asks `catalog.is_bundled(recipe_arg)`).

The prompt, CLI and TUI alike, shows:

- **where it came from:** "Project recipe `optimize-images` from `<root>/.bnto/recipes/optimize-images.bnto.json`";
- **each step by its node type,** not the author's `name` for it;
- **every path-like param** (`file-list` `base` and `paths`, `file-delete` `base`, `file-move` and `file-copy` `destination`, the output `directory` and overwrite mode, `shell-run` `searchPath`), with any value that's absolute, holds `..` or `~`, or leaves the project root **flagged**. `shell-run`'s `cwd` already refuses absolute paths and `..` (`bnto-shell/src/cwd.rs`), so it's shown, not flagged;
- the shell commands, as today, when it has shell steps;
- **the project files it runs** (a relative `searchPath`, `cwd` or script path), with: "changes to these files won't ask again" (settled #4).

**Author text is escaped everywhere bnto prints it:** the prompt, `bnto list`, `info`, `doctor` and the TUI show C0/C1 control characters (ESC, `\r`) visibly and flag bidi overrides.

`--yes` approves, as now. Running any recipe **by path** keeps today's rule (consent only for shell steps), so Task and CI never hit a prompt they can't answer.

### 5. Working folder — **Changed:** project recipes run at the project root

A project recipe, run by slug from the CLI or opened from the TUI's project list, runs with the project root as its working folder, wherever the user started. Every relative path in it then means the same thing from any subfolder, and the root's `.env` is the one loaded (`context.rs` loads `.env` from the working folder).

- **Every node resolves relative params against `ctx.work_dir()`,** never the process's current folder: `file-list` `base`, `file-delete` `base`, `file-move` and `file-copy` `destination`, and the output `directory`. Today `file-list` without a `base` and `shell-run`'s `cwd` and `searchPath` do (`bnto-shell/src/run.rs`, `cwd.rs`, `search_path.rs`); the others resolve against the process's folder, and they follow `shell-run`'s pattern. bnto never changes the process's folder: the TUI's bridge runs on a thread beside the UI, and the folder is shared by the whole process.
- **Command-line paths stay where the user meant them:** input files and `-o` are made absolute against the current folder first, so `bnto run optimize-images ./photo.png` from `src/` still means `src/photo.png`.
- **The preview runs where the run does:** `bnto dry-run` with inputs (`dry_run/files.rs`) uses the same root.

Prior art: npm runs `scripts` at the package root, Task runs in the Taskfile's folder, `just` in the justfile's folder. `make` uses the current folder, and needing `make -C` is exactly what this avoids.

A recipe run by path keeps today's behaviour (the current folder), so Task's `bnto run recipes/x.bnto.json` is unchanged.

### 6. Surfaces

- **`bnto list`** prints a `PROJECT (<root folder name>)` section first, then the library, then the built-ins. Skipped files are listed under it with the reason.
- **`bnto run <slug>`, `dry-run`, `info`** resolve project slugs like the rest.
- **`bnto doctor`** reports the project root found (or none) and anything skipped. It lists a project recipe's `requires` but doesn't run `<binary> --version` for them: that would run binaries a repo picks, without consent.
- **`bnto install --all`** leaves project recipes out: with `--yes` it would install whatever packages a cloned repo names, with no prompt, and a name with `/` (`owner/tap/formula`) makes Homebrew tap and run a third-party repo. `bnto install <project-slug>` always asks, `--yes` or not.
- **The TUI home screen** gets a Project pane above Library, shown only when a project is found (mock-up below). The status bar names the project. Project entries open, run and show detail like library ones; the detail and consent screens show the file's path. Delete and rename stay library-only: the TUI doesn't edit a repo's files.
- **The TUI editor** keeps a project recipe's source (`EditorSource::Project(path)`), so a run from the editor asks for consent by the project rule, not the library one. Saving a project-sourced recipe never writes into the repo: it saves to the library, and refuses (until the user renames it) a slug that a built-in has or a library file already holds, so it neither overrides a built-in by accident nor replaces the user's own recipe. Saving any other recipe works as today, built-in overrides included. Today a project recipe named "Compress Images" saved once would become `~/.bnto/recipes/compress-images.bnto.json`: a no-shell library recipe that overrides the built-in in every folder and never asks (`editor/io.rs` `save_path`).
- **Telemetry** sends a project recipe's slug as `custom`, as for library recipes (#734), and never the project's folder name or path.

```
╭ P R O J E C T · acme-site ───╮╭ R E C I P E S ─────────────────╮
│  List TODOs                   ││  FILE                          │
│  Optimize Images              ││   Flatten Folders              │
│  Products to JSON             ││   ...                          │
╰───────────────────────────────╯│                                │
╭ L I B R A R Y ────────────────╮│                                │
│  keep-awake                   ││                                │
╰───────────────────────────────╯│                                │
╭ N E W ────────────────────────╮│                                │
╭ S E T T I N G S ──────────────╮╰────────────────────────────────╯
 Tab pane  ↑↓ navigate  Enter select  q quit   acme-site · 22 recipes
```

### 7. Tests are sandboxed from this repo's own recipes

Once bnto's recipes move to `.bnto/recipes/`, every test that runs `bnto` with a working folder under `engine/.tmp` (the TUI journeys, the CLI integration tests) would walk up and find them. The test harnesses create an empty `.git` folder at their sandbox root, which stops the walk there, through the same rule users get, with no test-only switch. Many CLI integration tests don't set a working folder at all (`Command::new(bnto_bin())` in about 30 files under `bnto/tests/`), so `bnto` runs in `engine/crates/bnto` and its walk would reach the repo root. Those calls move to one helper that starts `bnto` in a sandbox folder holding the marker, and a test fails on a bare `Command::new(bnto_bin())` left anywhere. This overlaps ISS-8 (tests that touch the real `~/.bnto`), which the same helper moves closer to fixing. RED tests pin both: a sandboxed `bnto list` inside this repo shows no project section, and the bare-command check fails on a fixture that has one.

## Won't build

- **A project recipe overriding a built-in or library slug,** even inside the project: section 3.
- **A `--project <dir>` flag or a `BNTO_PROJECT` variable.** `cd` does it, and `bnto run <path>` covers scripts. Add one when a real use needs it.
- **A project `config.toml`** (theme, output folder, variables) in this work. `.bnto/` leaves room; a need comes first.
- **Searching above the first `.git`,** or merging several projects' recipes. One project root, the nearest.
- **Editing project recipes from the TUI** (delete, rename, save into `.bnto/recipes/`) in this work. A follow-up once read and run are solid.
- **A sandbox that confines a project recipe to its root.** The prompt flags paths that leave it (section 4); refusing them would break legitimate recipes (a project that writes to `~/Downloads`). Revisit if consent proves not enough.

## Settled with Ryan (November 3, 2026)

1. **Approval is pinned to the bytes, not the project.** Approving a recipe in one repo approves an identical file in another: the same bytes do the same thing.
2. **A project's `.env` needs no consent of its own.** It already loads when you run from the root today (and a library recipe run in a cloned repo already reads that repo's `.env`). Running project recipes at the root only makes that the same from any subfolder.
3. **A recipe run by path keeps the current folder,** even from inside a project's `.bnto/recipes/`, so Task and CI are untouched.
4. **A recipe that runs the repo's own files says so in its prompt.** The review found that a project recipe can run files from the repo (`searchPath: ["bin"]`, `script: ./tools/x.sh`, a `cwd`), and the hash pins the recipe, not those files: a `git pull` that changes `tools/x.sh` never asks again, and identical recipe bytes in another repo run that repo's script. The prompt names the files and says changes to them won't ask again (§4). npm, Task and `just` run repo scripts with no consent at all, and pinning every file a script might touch can't be done. Rejected: pinning such recipes to (project root, hash), which still misses a `git pull` that changes the script.

## Security review (November 3, 2026)

A fresh `/security-engineer` agent reviewed the first draft against the code, running bnto 0.19.0 in a sandbox for several findings. Where each landed:

| #     | Severity   | Finding                                                                                                                                                                                                                                                                          | Where it's handled                                                                                                     |
| ----- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| 1     | blocking   | Nodes resolve relative params against the process's folder, so switching the context's folder makes a mixed-root run                                                                                                                                                             | §5; step 7f                                                                                                            |
| 2     | blocking   | `file-list` takes an absolute `base` despite its docs, and delete and move take any folder, so the prompt is the only protection, and step names aren't informed consent                                                                                                         | §4; ISS-123; steps 7c, 7f                                                                                              |
| 3     | blocking   | `bnto install --all --yes` in a cloned repo would install packages it names, unchecked, a Homebrew tap among them                                                                                                                                                                | §6; ISS-124; step 7e                                                                                                   |
| 4     | blocking   | The TUI editor would save a project recipe into the library or the repo, and run it under the library's consent rule                                                                                                                                                             | §6; step 7h                                                                                                            |
| 5     | should-fix | Author text reaches the terminal unescaped                                                                                                                                                                                                                                       | §4; ISS-125; step 7c                                                                                                   |
| 6     | should-fix | The prompt doesn't say a recipe is from a project; lookalike slugs pass                                                                                                                                                                                                          | §3, §4; steps 7c, 7e                                                                                                   |
| 7     | should-fix | The hash doesn't pin repo files a recipe runs                                                                                                                                                                                                                                    | Settled #4; §4; step 7c                                                                                                |
| 8     | should-fix | Without git, the walk can reach folders other users control                                                                                                                                                                                                                      | §2; step 7d                                                                                                            |
| 9     | should-fix | A relative `BNTO_HOME` makes a repo's `.bnto/` the bnto home                                                                                                                                                                                                                     | §2; ISS-126; step 7d                                                                                                   |
| 10    | should-fix | The loader follows symlinks and reads any file type, unbounded                                                                                                                                                                                                                   | §1; step 7e                                                                                                            |
| 11    | should-fix | Two files can share a slug; a dotted slug takes over a path                                                                                                                                                                                                                      | §3; step 7e                                                                                                            |
| 12–20 | note       | By-path runs decide trust from the argument string; `.env` values aren't pinned; `doctor` probes project binaries; `http-request` will need consent; submodule origin; CLI and TUI may read different trust files; telemetry of the root name; dry-run's folder; extra RED tests | §4, §5, §6, settled #2; the trust-file split is ISS-127 (step 7b); `http-request`'s consent goes to its own design doc |

## Found while evaluating

Logged in [ISSUES.md](../ISSUES.md) with repros. All but ISS-122 are fixed inside this work (batch 5e there):

- ISS-122: `file-collect`'s `pattern` treats `{a,b}` literally, so `*.{png,jpg}` matches nothing and the run exits 0. Not part of this work (batch 7).
- ISS-123: `file-list` uses an absolute `base` as written, though its docs say it's refused. Step 7f.
- ISS-124: `bnto install` hands a recipe's `binary` to the package manager unchecked. Step 7e.
- ISS-125: recipe text reaches the terminal unescaped. Step 7c.
- ISS-126: a relative `BNTO_HOME` resolves against the current folder. Step 7d.
- ISS-127: the CLI's trust file ignores the config's `paths.home`. Step 7b.

## Delivery

Ryan's call (November 3, 2026): **one feature branch, `feat/project-recipes`, merged to `main` as one squash PR after his approval, and released as a new version.** That overrides, for this body of work, the single-concern rule ([pre-commit.md](../rules/pre-commit.md#planning-for-pr-sizing)) and, for ISS-123 to ISS-127, the defect-track rules: they're live on `main` but fixed inside the feature (feature-requests.md says defects don't wait for a feature), and their batch, 5e, runs whenever this branch does, not in the work order's sequence ([ISSUES.md](../ISSUES.md#work-order)); Ryan sets when the branch runs against batches 5b and 5c. Each needs the feature's design to fix well (the source kind, the prompt, discovery).

- **The PR to `main` opens as a draft with step 7a,** so CI (which runs only on PRs to `main`) runs on every push to the branch.
- **Each step is its own commit or commits on the branch,** through the full flow before it's pushed: RED tests, `task check`, a cold review (and a `/security-engineer` review where the table says), fixes. The PR description keeps a checklist of steps with each one's review outcome.
- **The branch merges `main` in** when `main` moves, rather than rebasing, so pushed history stays put.
- **When all ten steps are in,** a final cold review of the whole branch, Ryan's approval, squash merge, then a release per [releases.md](../scopes/process/releases.md). A feature like this is a minor version bump.

## Plan — 10 steps on one branch

Numbers continue the dogfooding tracker. Each step is one sentence, as a PR would be, so each can be reviewed alone.

**What changes:** `bnto list`, `run`, `dry-run`, `info`, `doctor`, `install` and the TUI learn a third recipe source; consent, discovery, path resolution in four `bnto-file` nodes, and the editor's save change to suit. **What doesn't:** no node, param or built-in recipe is added or removed (22 recipes, 8 dogfood recipes), the WASM build and the web app are untouched, and a recipe run by path behaves as today.

| Step | One sentence                                                                                |
| ---- | ------------------------------------------------------------------------------------------- |
| 7a   | Record this design, the tracker rows and the defects it found.                              |
| 7b   | Decide consent by where a recipe came from, so a project recipe always asks until approved. |
| 7c   | Make the consent prompt show a recipe's origin, step types and flagged paths, escaped.      |
| 7d   | Find a project root by walking up, refusing folders the user doesn't own.                   |
| 7e   | Load a project's recipes into the catalog, skipping anything unsafe or colliding.           |
| 7f   | Resolve every node's relative paths against the run's working folder.                       |
| 7g   | Run a project recipe at its project root from any subfolder.                                |
| 7h   | Keep a project recipe's source through the TUI editor.                                      |
| 7i   | Show the project's recipes in their own home-screen pane.                                   |
| 7j   | Make this repo the first project by moving `recipes/` to `.bnto/recipes/`.                  |

### 7b: Consent by source kind

- **What:** `SourceKind { Bundled, Library, Project(root), Path }` replaces `is_bundled: bool` in `trust_level`, the TUI's consent check (`ConsentCheck.bundled`) and the trust badge (a project recipe shows `project`); `read_recipe` returns the kind with the JSON; the CLI and TUI read one trust file (ISS-127). Nothing loads project recipes yet.
- **Files (~0 new, ~12 modified):** `catalog.rs`, `recipe_source.rs`, `trust.rs`, `trust_gate.rs`, `consent.rs`, `inspect.rs`, `install_trust.rs`, `context.rs`, `storage/paths.rs`, `tui/app_helpers/consent.rs`, `tui/bridge/precheck.rs`, `tui/render_detail.rs`.
- **Key API:** `enum SourceKind { Bundled, Library, Project(PathBuf), Path }` (the project root); `fn trust_level(kind: &SourceKind, has_shell: bool, store: &TrustStore, hash: &str) -> TrustLevel`; `fn read_recipe(arg: &str, catalog: &RecipeCatalog) -> io::Result<(String, SourceKind)>`.
- **RED tests:** a no-shell project recipe needs consent; approved, it doesn't; edited, it asks again; a no-shell library recipe still runs without asking; a path argument that names a built-in's slug gets the kind it resolved to; with `paths.home` set, an approval from one surface counts for the other, and the user `.env` and output folder come from it too.
- **Verification:** `task cli:test && task tui:test && task wasm:lint`.
- **Count changes:** none.

### 7c: A consent prompt that says what the recipe does

- **What:** §4's prompt for the CLI and the TUI consent screen; one escaping function for author text (ISS-125), used by the prompt, `list`, `info`, `doctor` and the TUI.
- **Files (~2 new, ~6 modified):** new `consent_paths.rs` (path params, flagged) and `display_text.rs` (escaping); `trust.rs`, `consent_lines.rs`, `list.rs`, `info.rs`, `doctor.rs`, the TUI consent render.
- **Key API:** `fn path_params(def: &PipelineDefinition, root: Option<&Path>) -> Vec<PathParam>` (node id, type, param, value, `flagged: bool`); `fn display_text(s: &str) -> Cow<str>`.
- **RED tests:** the prompt names the source file; steps show by type, not author `name`; an absolute `file-delete` `base` is flagged; `..` and `~` are flagged; a relative `searchPath` is listed under "project files it runs"; ESC and `\r` in a name or script show escaped in the prompt and `bnto info`.
- **Verification:** `task cli:test && task cli:golden && task tui:test`; review any `.snap` diff.
- **Count changes:** TUI consent snapshots change.
- **As built (step 7c):** the prompt's lines come from one `consent_body` that the CLI (`consent_prompt.rs`) and the TUI (`tui/render_consent.rs`) both render, so the two can't drift. Its top names the risks in one line ("2 flagged paths · deletes files · moves files · overwrites files · runs 1 command", `consent_summary.rs`), which the CLI repeats right above `Allow execution?` and the TUI keeps pinned while the rest scrolls; its heading follows the steps and source, not whether a command line was found. A `PathParam` carries the flag's reason (`path_flag.rs`): absolute on any OS, `..`, a leading `~`, a `.git` or `.bnto` segment (any case), outside the project through a link on disk (the literal folder before any placeholder is followed), or any placeholder that can hold a path, wherever it sits (only `{{ctx.date}}`, `time`, `timestamp`, `started` and `platform` can't, `path_placeholders.rs`). Listed params (`consent_paths.rs`): the folders and files steps list, delete under, move, copy or export to, read (`spreadsheet-lookup` `table`) or find commands in, and output names (`file-render`, `file-join`, `file-decode`, output `filename`; every part `file-rename` builds a name from, with `prefix`, `suffix`, `separator`, `extension` and `replace` flagged for a separator and `replace` for a `$` group). Overwrite mode, `file-copy` `conflict: overwrite` or `newer`, and `spreadsheet-export` `overwrite` (read as the engine reads it) are flagged for what they do, and a placeholder in any of them "may replace files"; `file-move` refuses to replace a file, so it names no such effect. A guard test fails when a node gains a path-like param the listing doesn't name. `image-overlay`'s `overlay` holds base64 image data in a recipe (only `-p` reads a file), so it isn't listed. `shell-run` `env` shows as `NAME=value`. A command or path that reads `{{fields.*}}` shows the value it will run with (the field's default, or a `-p` or form value the run was given; escaped) beside the template: under a command as a `runs:` row, after a path as `= value`, and a path is judged by that value first. A `file-rename` with `find` set is flagged "edits the whole path, may move files to other folders": find and replace act on the whole relative path, so a plain `replace` can build `..` or `.git`. A `file-list` path is judged under its `base`. `SourceKind` carries the recipe file (`Library(Option)`, `Project { root, file }`, `Path`), `None` when what runs isn't that file's text (a TUI edit). The TUI's prompt scrolls one row at a time (`j`/`k`), so its last row is reached only past every row, and `y` (and the app's own check before it records an approval) allows only then; a panel with fewer than three rows under the pinned ones asks for a bigger terminal and can't be allowed. A panel narrower than 20 columns can't be allowed either, and a new panel width starts the prompt over from its first row (rows re-wrap, so a kept row index could skip some). Long words wrap by display width, so wide characters all show. Files a command runs are found from a relative `searchPath`, a `cwd`, and any word of its args, `env` values or script that reads as a relative path (an option's `--opt=path` too), or, for a project recipe, a bare name of a file in the project root or the step's `cwd`; the prompt says the list may be incomplete. `display_text` escapes recipe text everywhere the CLI prints it (prompt, `list`, `info`, `doctor`, `dry-run`, `check`, `install`'s plan, run progress and messages, errors) and in the TUI (consent, detail and its form labels and values, the editor, library, home, picker, results and run errors). A command's output loses its SGR colour codes (ESC [ … m) before the rest is escaped, so colours vanish instead of showing as `\e[32m`; any other control sequence still shows escaped. **The "outside the project" flag is no protection on its own:** it's read when the user approves, and a later `git pull` can turn `assets/` into a symlink to `~` without changing the recipe's hash; step 7f refuses such a path at run time.

### 7d: Finding the project root

- **What:** `project_root` per §2; an absolute `BNTO_HOME` and `paths.home` (ISS-126); both relative `.bnto` fallbacks removed. Both fallbacks now go through `BntoPaths::resolve_or_relative` (`storage/paths.rs`, since 7b), so removing it removes both. Also from 7b's reviews: the TUI Settings screen decides whether the session home moved by comparing path spellings (`config_is_the_sessions`, `tui/app_helpers/settings.rs`); compare the config files by identity (`same-file`) instead, so a symlinked home can't take the "moved" branch and let a later theme save undo a new home. Not wired to anything yet.
- **Files (~1 new, ~3 modified):** new `project_root.rs`; `storage/paths.rs`, `storage/mod.rs`, `tui/app/init.rs`.
- **Key API:** `fn project_root(start: &Path, home: &Path, bnto_home: &Path) -> Option<PathBuf>`.
- **RED tests:** found in the start folder and three levels up; stops at a `.git` dir and a `.git` file; never returns home (trailing slash, symlinked `/var`, other case); skips a root whose recipes are the library; ignores `recipes/` without the dot; ignores a `.bnto` that's a symlink; stops at another user's folder and at an unreadable parent (Unix); `BNTO_HOME=.bnto` and `BNTO_HOME=` are refused.
- **Verification:** `cargo test -p bnto project_root`, then `task cli:test && task wasm:lint`.
- **Count changes:** none.

### 7e: Loading project recipes

- **What:** §1's file rules and §3's slug rules; skipped entries with reasons; `main.rs` and TUI launch pass the root; `list`, `info`, `doctor`, slug runs; `install --all` leaves project recipes out, `install <project-slug>` always asks, and a `binary` is checked before planning (ISS-124); `doctor` skips version probes for them; §7's test helper and `.git` marker.
- **Files (~1 new, ~10 modified, plus the ~30 test files moved to the helper):** new `bnto/tests/helpers/command.rs`; `catalog.rs`, `main.rs`, `list.rs`, `info.rs`, `doctor.rs`, `install.rs`, `install_plan.rs`, `package_manager.rs`, `tui/app/init.rs`, `tests/tui/harness.rs`.
- **Key API:** `fn RecipeCatalog::load(library: &Path, project: Option<&Path>) -> Self`; `fn RecipeCatalog::skipped(&self) -> &[Skipped]` (file, reason).
- **RED tests:** a colliding slug, a case-only collision, a dotted slug, a doubled suffix, two files with one slug, a symlinked file, a FIFO and a 2 MB file are each skipped and `bnto list` says why; `bnto install --all` in a project plans none of its deps; `owner/tap/x` and `-o x` are refused as a `binary`; a sandboxed `bnto list` inside this repo shows no project section; the bare-command check.
- **Verification:** `task cli:test && task cli:golden && task tui:test && task wasm:lint`.
- **Count changes:** no recipe counts; `bnto list` golden output unchanged in sandboxed tests.

### 7f: Nodes use the run's working folder

- **What:** `file-list` `base`, `file-delete` `base`, `file-move` and `file-copy` `destination` and the output `directory` resolve against `ctx.work_dir()`, as `shell-run` already does; `file-list` refuses an absolute or `..` `base` as its docs say (ISS-123). For a project recipe, a `base` or `destination` that resolves (through a symlink) outside the project root is refused at run time: the consent prompt's "outside the project" flag is read when the user approves, and a later `git pull` can turn `assets/` into a link to `~` without changing the recipe's hash. A file name with an absolute path, a drive or a `..` segment is refused wherever it's joined onto a folder: the output write and `file-copy` (ISS-128). No other behaviour change today, since the working folder is the current folder for every run.
- **Files (~0 new, ~7 modified):** `bnto-file` `list/mod.rs`, `list/params.rs`, `delete/target.rs`, `move/root.rs`, `move/plan.rs`, `copy.rs` (and `copy_batch/` if it resolves its own), `bnto/src/recipe_output_dir.rs` and `run.rs` (output folder).
- **Key API:** each resolver takes `ctx.work_dir()`: e.g. `fn base_root(base: &str, work_dir: &Path) -> Result<PathBuf, String>`.
- **RED tests:** per node, a context whose working folder differs from the process's: delete `base`, move and copy `destination`, list `base` and output `directory` land under the context's folder; `file-list` refuses `/abs` and `../x` as `base`, naming the param; a project recipe whose `base` or `destination` is a symlink leaving the project root is refused before the step acts; a `file-render` `filename` of `../x` or an absolute path, a `file-rename` `prefix` of `../x_`, a `find`/`replace` that builds `..` or `.git` from a path's parts (the review's `dotrep` and `hook` recipes), and a `file-copy` of a file so named, refuse the run before any file is written (ISS-128); the check runs on each file's final name, after every rename.
- **Verification:** `task wasm:test:unit && task cli:test && task wasm:lint`.
- **Count changes:** none.

### 7g: Running at the project root

- **What:** §5 for `bnto run` and `dry-run` by slug and the TUI bridge: the context is `NativeContext::new(root, &paths)`, which already takes a working folder and loads its `.env`; inputs and `-o` made absolute first.
- **Files (~0 new, ~4 modified):** `run.rs`, `dry_run/mod.rs`, `dry_run/files.rs`, `tui/bridge.rs`.
- **Key API:** `fn run_dir(kind: &SourceKind, cwd: &Path) -> PathBuf`: the root inside `SourceKind::Project(root)`, else `cwd`.
- **RED tests:** the example image recipe run by slug from `src/` writes to `<root>/public/images`; a relative input and `-o` from `src/` stay under `src/`; the root's `.env` loads from a subfolder, and a library recipe from `src/` still reads `src/.env`; preview and run use the same root; a recipe run by path still uses the current folder.
- **Verification:** `task cli:test && task tui:test`.
- **Count changes:** none.

### 7h: The editor keeps a project recipe's source

- **What:** `EditorSource::Project(path)`; a run from the editor uses the project consent rule; §6's save rules.
- **Files (~0 new, ~4 modified):** `bnto-core/src/editor/io.rs`, `tui/app_helpers/editor.rs`, `tui/app_helpers/consent.rs`, `tui/app/init.rs`.
- **Key API:** `EditorSource::Project(PathBuf)`; `fn save_path(&self, recipes_dir: &Path) -> Result<PathBuf, SaveRefusal>` for project sources.
- **RED tests:** a project recipe opened in the editor and run asks for consent; saving it doesn't write into the repo; saving one named "Compress Images" is refused until renamed; saving one whose slug a library file holds is refused; saving a built-in copy still overrides as today.
- **Verification:** `cargo test -p bnto-core editor && task tui:test`.
- **Count changes:** none.

### 7i: The Project pane

- **What:** §6's pane and status-bar name; open, run and detail from it, with the file path shown; delete and rename refused for project entries.
- **Files (~0 new, ~6 modified):** `screens/home.rs`, `render_home_panes.rs`, `render_home_grid.rs`, `footer_hints.rs`, `app_helpers/navigation.rs`, `app_helpers/library.rs`; new snapshots; a journey in `tests/tui_journeys.rs` and a row in `journeys/tui.md`.
- **Key API:** `HomePane::Project`; `HomeModel { project: Option<ProjectList>, .. }`.
- **RED tests:** snapshots with and without a project (80x24, 120x36); Tab skips the pane without a project; a journey started in a subfolder opens a project recipe, approves its consent, and its output lands under the root.
- **Verification:** `task tui:test`; `task tui:snapshots:update` only for intended diffs, each reviewed, then `task tui:test` clean.
- **Count changes:** four new home-screen snapshots; existing ones unchanged.

### 7j: Move bnto's dogfood recipes

- **What:** `git mv recipes .bnto/recipes`, then every path, found with `git grep -n 'recipes/'` (not `bnto-engine/recipes`): Taskfile tasks and its header comments, `ci.yml`, `scripts/ci/changed-surfaces.sh` and its test, the recipes README, CLAUDE.md, `rules/code-standards.md`, `rules/gotchas.md`, `scopes/process/releases.md`, `dogfooding.md`, ISSUES.md. Drop "run it from the repo root" from the descriptions. Task and CI keep running them by path.
- **Files (~9 moved, ~8 modified).**
- **Key API:** none; paths only.
- **RED tests:** `changed-surfaces.test.ts` fails until the new path is on the lists; `task check:recipes` checks `.bnto/recipes/`.
- **Verification:** `task check`; by hand, `cd engine && bnto tui` shows the eight recipes.
- **Count changes:** none (8 dogfood recipes, 22 built-ins).

Every step runs `task check` and gets a cold review. Steps 7b–7e and 7h also get a `/security-engineer` review. None adds a built-in recipe or a node.

### Dependency chain

```
7a design record
├── 7b consent by source kind
│   └── 7c prompt shows origin and paths
├── 7d find the project root            (independent of 7b-7c)
├── 7f nodes use the working folder     (independent; before 7e: a project's links and output names are refused at run time, ISS-128)
│
7e load project recipes                 (needs 7b, 7c, 7d and 7f: nothing loads before consent and run-time checks are right)
├── 7g run at the project root          (also needs 7f)
├── 7h editor keeps the source          (needs 7b's kinds and 7e's entries)
└── 7i Project pane                     (needs 7g and 7h)
    └── 7j move this repo's recipes
```

7b, 7d and 7f can go in any order, but 7f lands before 7e. The branch takes 7d next, then 7f, then 7e.
