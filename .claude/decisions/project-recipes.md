# Decision: Project recipes in `.bnto/recipes/`

**Created:** November 3, 2026
**Status:** Agreed with Ryan on November 3, 2026, after a `/security-engineer` review the same day (4 blocking, 7 should-fix, 9 notes, all taken in: see [Security review](#security-review-november-3-2026)). Ryan settled five questions (below) and chose to ship the work as one feature branch and one PR to `main`, released as a new version ([Delivery](#delivery)). Implements the "Project recipes" section of [dogfooding.md](../strategy/dogfooding.md#project-recipes-7a-7k), and changes its earlier sketch in two places (marked **Changed**).
**Request:** [2026-11-03-project-recipes.md](../requests/2026-11-03-project-recipes.md).
**Applies to:** in `engine/crates/bnto/src/`: `catalog.rs`, `recipe_source.rs`, `inspect.rs`, `recipe_output_dir.rs`, `install_plan.rs`, `trust.rs`, `trust_gate.rs`, `consent.rs`, `consent_lines.rs`, `install.rs`, `install_trust.rs`, `package_manager.rs`, `main.rs`, `list.rs`, `info.rs`, `doctor.rs`, `run.rs`, `dry_run/`, `context.rs`, `storage/paths.rs`, `storage/mod.rs`; the TUI's `app/init.rs`, `bridge.rs`, `bridge/precheck.rs`, `render_detail.rs`, `screens/home.rs`, `render_home_panes.rs`, `render_home_grid.rs`, `footer_hints.rs`, `app_helpers/` (`consent.rs`, `editor.rs`, `navigation.rs`, `library.rs`); `bnto-core/src/editor/io.rs`; `bnto-file`'s `list/`, `delete/`, `move/`, `copy.rs` and `copy_batch/`; the test helpers in `bnto/tests/`; for §8 (step 7k): `bnto-core/src/context.rs`, `confine.rs`, `executor/run_scope.rs`, `bnto-file`'s `relocate/`, `copy_newer.rs`, `bnto-spreadsheet`'s `export/` (and `lookup/` for step 7f), and in `bnto` `io.rs`, `ordered_write/`, `progressive_output.rs`, `overwrite/`, `tui/bridge.rs`.

## Context

A user wants bnto as a project's task runner: `cd` into a repo, run `bnto tui`, and see that repo's own recipes beside their library and the built-ins. Today bnto loads recipes from two places only, the 22 built-ins and `~/.bnto/recipes/` (`RecipeCatalog::load`, `catalog.rs`). A project recipe runs only by path (`bnto run .bnto/recipes/x.bnto.json`), and the TUI opens one by path (`bnto tui <file>`) with no list to pick from.

bnto's own repo is the first user: its eight dogfood recipes live in `recipes/`, each run by path from Taskfile tasks, and six of them say "run it from the repo root".

### What a run sees today (measured November 3, 2026, bnto 0.19.0)

An example project with three recipes in `.bnto/recipes/` (image resize to WebP, CSV to JSON, a `grep` shell step), sandboxed `HOME` and `BNTO_HOME`:

- From the project root, all three work by path: 2 WebP files in `public/images/`, `products.json` in `public/data/`, the TODO lines printed. `bnto check .bnto/recipes/` passes.
- From `src/`, the image recipe lists `assets/raw` under `src/`, finds it missing, writes 0 files, **exits 0**, and leaves an empty `src/public/images/`.

So listing project recipes isn't enough: run from a subfolder, they quietly do nothing, or the wrong thing.

### What a recipe can reach today

The working folder confines nothing. `file-list` takes an absolute `base` and uses it as written, though its docs say absolute paths and `..` are refused (`bnto-file/src/list/mod.rs` `base_dir`, `list/params.rs`). `file-delete` resolves its `base` and `file-move` its `destination` against the process's current folder, not `ctx.work_dir()` (`delete/target.rs` `base_root`, `move/root.rs`, `move/plan.rs`), and the CLI keeps the output folder as a relative string (`run.rs`). So a no-shell recipe can list files anywhere and delete or move them. For a recipe the user wrote, that's their choice. For one a cloned repo ships, **the consent prompt is the only protection** (until step 7k's run-time confinement, §8), and it has to show enough to be informed consent.

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
- at a folder, `.bnto` or `.bnto/recipes` not owned by the current user, or, on Unix, one others can write in: world-writable, or **group-writable** (macOS puts every user in `staff`) unless the group is the user's **private group** (the user's primary group, named as the user, listing no other members and no other account's primary group, found by scanning the passwd database: Debian's, Ubuntu's and Fedora's default; any lookup error counts as shared). The sticky bit excuses shared write only on a folder the walk climbs through, as on `/tmp`; on `.bnto` and `.bnto/recipes`, whose files load, it doesn't, since others can still add files there. This is git's `safe.directory` lesson (CVE-2022-24765, `C:\.git`): without the `.git` requirement, a walk from `/tmp/x` or a USB stick would otherwise reach a `/tmp/.bnto` any local user can plant. On Windows the owner is read as git reads it (the folder's owner SID against the process token's user; the Administrators group counts only while the process holds it, elevated), so a planted `C:\ProgramData\.bnto` reached from `C:\ProgramData\Vendor\tool` stops the walk; like git, Windows checks only the owner, not the folder's DACL; and the walk never checks a drive root (`C:\.bnto`). Any error reading an owner counts as not owned: the user just gets no project recipes. A folder whose filesystem doesn't record real owners is unsafe too: on macOS a mount marked `noowners` (`MNT_IGNORE_OWNERSHIP`, the default for disk images and external drives) or of type msdos, exfat, ntfs, smbfs, afpfs, webdav, cd9660, udf, nfs (FUSE-T reports as it), macFUSE (`macfuse`, `osxfuse`, `osxfusefs`, `fusefs`) or `lifs`; on Linux FAT, exFAT, NTFS (both drivers), HFS, SMB/CIFS, ISO 9660, UDF, 9p (WSL2's `/mnt/c`, VM shares), VirtualBox shared folders, NFS and any FUSE mount, read with `statfs`. Such mounts take every file's owner from their options, so the owner check alone would pass for files anyone put there; NFS (AUTH_SYS) takes whatever uid a client machine's root sends, so a home on NFS gets no project recipes, a cost accepted since step 7e's doctor says why. On Windows, FAT and exFAT drives have no owner SID and most network shares report none usable. So project recipes don't load from any of these; Home or the bnto home that can't be opened also finds nothing, since the walk couldn't recognise them;
- at another filesystem than the start folder's (`st_dev` on Unix), git's default without `GIT_DISCOVERY_ACROSS_FILESYSTEM`: a project root is found only on the filesystem the walk started on. Windows doesn't check this yet: std's volume serial number is unstable;
- on any error other than NotFound (permissions, a dead mount);
- at the filesystem root.

**Accepted gap, as git's:** the owner check keeps out files someone else planted, not edits to the user's own. Another local user with write access to a project folder the user owns (a group the user shares, a Windows DACL entry) can edit a recipe or a script it runs in place, and the owner stays the user, the same as for git's `.git/config`. Likewise only mode bits and owners are read, as git reads them: a POSIX ACL on Linux, a macOS ACL or a Windows DACL entry that lets someone else write isn't seen. Nor is a disk someone else made that keeps owners: an ext4, squashfs or erofs image or USB stick mounted by udisks on Linux, or an APFS/HFS disk image with ownership turned on, carries the uids its maker chose, and uid 1000 or 501 matches the usual first user. git has the same gap, and the consent prompt still asks before such a recipe runs. The device stop ends the walk at any `st_dev` change, so a btrfs subvolume boundary stops it too (as git's does): fails closed, just fewer projects found.

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

### 8. Project recipes are confined at run time

For a project recipe (`SourceKind::Project`), every place where **the recipe chooses** a file to write, overwrite, move or delete is checked when it happens, against the project root, and refused if it fails. Library, built-in and by-path recipes are unchanged.

- **The recipe's choices are checked; the user's and bnto's aren't.** Checked: a destination named in the recipe (`file-copy` and `file-move` `destination`, `spreadsheet-export` `path`, the output node's `directory`), what a step deletes or moves (`file-delete` targets, `file-move` sources), every file name joined onto a folder (output names, `file-copy` names), and overwrite mode replacing a file the recipe listed itself (`file-list`). Not checked:
  - **the user's choices:** `bnto run -o <dir>`, and files the user passed in or picked (overwrite mode may replace those, wherever they are);
  - **bnto's own:** the TUI's default output folder (`<temp>/bnto-tui-<slug>`, used when a recipe names no `directory`) and every scratch file a node writes for itself (`ctx.temp_file`: `file-join`'s stream, `shell-run`'s file mode).
    `file-render` writes nothing itself; its name is placed by the output write.
- **The rule, for a target path:**
  1. Every component of a joined name is `Normal`: no root, prefix (a drive or UNC share), `..` or `.` component, and on Windows no root-relative `\x`. This half applies to **every** recipe (ISS-128).
  2. For a project recipe: walk the target from its base with `symlink_metadata` (lstat), component by component. A link anywhere along it, the last component included, is resolved with `read_link` and the walk continues from its target; a link whose target leaves the allowed folder is refused, so a dangling link (`out/x -> ~/Library/LaunchAgents/evil.plist`, which git can ship) can't pass. The walk follows at most 40 links (as `SYMLOOP_MAX`) and refuses past that, so two links pointing at each other can't hang the run. Components that don't exist yet are joined as written. The allowed folder is the project root for a base the recipe chose, and the base itself for one the user or bnto chose (see below).
  3. For a project recipe: no component under the root may name `.git`, matched as git does on every OS (any case; on Windows also with trailing dots or spaces, the 8.3 short name `GIT~1`, and `::$INDEX_ALLOCATION` streams; git's `is_ntfs_dotgit`). A `.git` segment in a joined name isn't refused for other recipes: copying a repo with `flatten: false` legitimately carries `.git/...` names.
  4. Deleting or moving a link acts on the link itself, so for `file-delete` targets and `file-move` sources the last component is checked with lstat, not resolved: a link inside the root that points outside may be deleted.
  5. Any error while resolving (permission denied on an ancestor, an unreadable link) refuses.
- **Where a base came from travels with it.** A check is always of a base folder and a name joined onto it, never of a bare path, because the two can have different owners: with `bnto run -o /tmp/out`, the folder is the user's and the file name is the recipe's. The write path takes an `OutputPlace { dir, chosen_by }`, with `chosen_by` one of `User` (`-o`, a picked folder), `Bnto` (the TUI's temp output, the CLI's default when neither `-o` nor a recipe `directory` is set: the run's working folder, in full, since step 7f) and `Recipe` (the output node's `directory`); `run.rs`'s `resolve_effective_output_dir` already knows which source won and returns it with the folder. For a `Recipe` base the whole target is checked against the project root (rules 2 and 3). For a `User` or `Bnto` base, the name is checked under that base: it must stay inside the base (rule 2 with the base as the allowed folder), and if the base sits inside the project, rule 3 applies too, so `-o .` at the root plus a recipe-built name of `.git/hooks/post-checkout` is refused. Under step 7g, a project recipe's default folder is the project root.
- **Files the user passed in travel with the run.** Overwrite mode replaces a result's source file, and a result carries no record of whether the user passed that file in or the recipe listed it. So the CLI and TUI hand the write path the set of files the user gave (command-line inputs, picked files), matched by file identity (`same-file`), beside the `Confinement`; overwrite mode may replace one of those wherever it is, and any other source is checked like a `Recipe` target.
- **Reads aren't confined.** A project recipe may read from outside its root (`file-list` `base`, `spreadsheet-lookup` `table`); the prompt flags those (§4).
- **Shell steps aren't confined** (see Won't build). Their `cwd` already stays under the working folder.
- **Where it lives:** a `bnto_core::Confinement { root }` value, built once per run with the root canonicalized, with `fn check_write(&self, base: &Path, chosen_by: ChosenBy, name: &Path)` and `fn check_remove(&self, target: &Path)` (rule 4), each returning the refusal; a node's own destination (`file-copy` `destination`, export `path`) is a `Recipe` base. `ProcessContext` gains `fn confinement(&self) -> Option<&Confinement>` (default `None`, so WASM and tests are unchanged); `NativeContext` holds one for a project recipe; `RunScope` forwards it (`executor/run_scope.rs`, with its forwarding test extended, since a missed forward would fail open). The CLI's and TUI's write path has no context, so `io::write_results`, `overwrite_in_place` and `relocate` take the `Option<&Confinement>`, the `OutputPlace` and the user's input set explicitly. Callers add the step and param to the refusal's message.
- **Every write point** (from a grep of non-test `std::fs` writes, renames, removes, copies and folder creation, November 3, 2026): `bnto-file` `delete/`, `relocate/` (`file-move` and overwrite mode), `copy.rs`, `copy_newer.rs`, `copy_batch/`; `bnto-spreadsheet` `export/`; `bnto` `io.rs`, `ordered_write/`, `progressive_output.rs`, `overwrite/`, `run.rs` (`progressive_dir` creates the output folder before any write, so it checks first), `tui/bridge.rs` (creates the output folder). `FileData::copy_to`/`write_to` (`bnto-core/src/file_data.rs`) are where most of these land; the check goes at the callers, which know whose choice the destination is, not inside `FileData`.
- **The guard is behavioural:** `consent_paths`' list gains a read/write mark on each entry (today one `Kind::Folder` covers both `file-list` `base` and `file-copy` `destination`). A test takes every write-marked param, and every node with a delete or move effect (`effect_param`), runs it under a `Confinement` against a target outside the root, and expects a refusal with nothing changed on disk. A new writing node or param lands in that list (its own guard test already requires it), so it can't skip confinement unnoticed. A `task check` script also fails on a raw `std::fs` write, rename, remove, copy or folder creation in a node crate outside the modules above and the named temp writers (`bnto-file/src/join/write.rs`, `bnto-shell/src/file_mode.rs`, which write only under `ctx.temp_file`).
- **The error** names the step, the param and both paths: "project recipe can't write outside `<root>`: `<target>`" or "… can't write inside `.git`: `<target>`". `bnto dry-run` reports the refusal the same way.
- **What it doesn't stop:** a race where a folder is swapped for a link between the check and the write, and hard links into the root (git can't create them). Accepted: both need someone changing the user's disk mid-run, not just a shipped recipe. The project root's own parent isn't checked either: a user-owned project inside a world-writable, non-sticky folder can be renamed away and another put in its place mid-run. Accepted, as git does. Nor does the owner check stop another local user with write access editing the user's own recipe or script in place (§2's accepted gap).

## Won't build

- **A project recipe overriding a built-in or library slug,** even inside the project: section 3.
- **A `--project <dir>` flag or a `BNTO_PROJECT` variable.** `cd` does it, and `bnto run <path>` covers scripts. Add one when a real use needs it.
- **A project `config.toml`** (theme, output folder, variables) in this work. `.bnto/` leaves room; a need comes first.
- **Searching above the first `.git`,** or merging several projects' recipes. One project root, the nearest.
- **Editing project recipes from the TUI** (delete, rename, save into `.bnto/recipes/`) in this work. A follow-up once read and run are solid.
- **Confining what a project recipe's shell commands do.** A `shell-run` step runs a program with the user's full access; bnto can't fence it in. Its consent prompt shows the commands, their env values and the project files they run (§4), and that is the protection.

## Settled with Ryan (November 3, 2026)

1. **Approval is pinned to the bytes, not the project.** Approving a recipe in one repo approves an identical file in another: the same bytes do the same thing.
2. **A project's `.env` needs no consent of its own.** It already loads when you run from the root today (and a library recipe run in a cloned repo already reads that repo's `.env`). Running project recipes at the root only makes that the same from any subfolder.
3. **A recipe run by path keeps the current folder,** even from inside a project's `.bnto/recipes/`, so Task and CI are untouched.
4. **A recipe that runs the repo's own files says so in its prompt.** The review found that a project recipe can run files from the repo (`searchPath: ["bin"]`, `script: ./tools/x.sh`, a `cwd`), and the hash pins the recipe, not those files: a `git pull` that changes `tools/x.sh` never asks again, and identical recipe bytes in another repo run that repo's script. The prompt names the files and says changes to them won't ask again (§4). npm, Task and `just` run repo scripts with no consent at all, and pinning every file a script might touch can't be done. Rejected: pinning such recipes to (project root, hash), which still misses a `git pull` that changes the script.
5. **Project recipes are confined at run time** (November 3, 2026, replacing "won't build a sandbox"). Three rounds of security review of the consent prompt each found a new way for a project recipe to write outside the project, or into `.git`, while the prompt looked harmless. The prompt informs; the engine now also enforces (§8). The accepted cost: a project recipe can't write to `~/Downloads` or another folder outside its project; the user copies it to their library, or runs it by path, for that.

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
- **When all eleven steps are in,** a final cold review of the whole branch, Ryan's approval, squash merge, then a release per [releases.md](../scopes/process/releases.md). A feature like this is a minor version bump.

## Plan — 11 steps on one branch

Numbers continue the dogfooding tracker. Each step is one sentence, as a PR would be, so each can be reviewed alone.

**What changes:** `bnto list`, `run`, `dry-run`, `info`, `doctor`, `install` and the TUI learn a third recipe source; consent, discovery, path resolution in four `bnto-file` nodes, and the editor's save change to suit; a project recipe's file writes are confined to its project (§8). Step 7k was added on November 3, 2026, after the 7c reviews; it keeps the next free letter, and lands before 7e. **What doesn't:** no node, param or built-in recipe is added or removed (22 recipes, 8 dogfood recipes), the web app is untouched and the WASM build's behaviour doesn't change (it compiles the new `bnto-core` and `bnto-file` code, with no confinement set), and a recipe run by path behaves as today.

| Step | One sentence                                                                                |
| ---- | ------------------------------------------------------------------------------------------- |
| 7a   | Record this design, the tracker rows and the defects it found.                              |
| 7b   | Decide consent by where a recipe came from, so a project recipe always asks until approved. |
| 7c   | Make the consent prompt show a recipe's origin, step types and flagged paths, escaped.      |
| 7d   | Find a project root by walking up, refusing folders the user doesn't own.                   |
| 7e   | Load a project's recipes into the catalog, skipping anything unsafe or colliding.           |
| 7f   | Resolve every node's relative paths against the run's working folder.                       |
| 7k   | Confine a project recipe's file writes, moves and deletes to its project, outside `.git`.   |
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
- **As built (step 7c):** the prompt's lines come from one `consent_body` that the CLI (`consent_prompt.rs`) and the TUI (`tui/render_consent.rs`) both render, so the two can't drift. Its top names the risks in one line ("2 flagged paths · deletes files · moves files · overwrites files · runs 1 command", `consent_summary.rs`), which the CLI repeats right above `Allow execution?` and the TUI keeps pinned while the rest scrolls; its heading follows the steps and source, not whether a command line was found. A `PathParam` carries the flag's reason (`path_flag.rs`): absolute on any OS, `..`, a leading `~`, a `.git` or `.bnto` segment (any case), outside the project through a link on disk (the literal folder before any placeholder is followed), or any placeholder that can hold a path, wherever it sits (only `{{ctx.date}}`, `time`, `timestamp`, `started` and `platform` can't, `path_placeholders.rs`). Listed params (`consent_paths.rs`): the folders and files steps list, delete under, move, copy or export to, read (`spreadsheet-lookup` `table`) or find commands in, and output names (`file-render`, `file-join`, `file-decode`, output `filename`; every part `file-rename` builds a name from, with `prefix`, `suffix`, `separator`, `extension` and `replace` flagged for a separator and `replace` for a `$` group). Overwrite mode, `file-copy` `conflict: overwrite` or `newer`, and `spreadsheet-export` `overwrite` (read as the engine reads it) are flagged for what they do, and a placeholder in any of them "may replace files"; `file-move` refuses to replace a file, so it names no such effect. A guard test fails when a node gains a path-like param the listing doesn't name. `image-overlay`'s `overlay` holds base64 image data in a recipe (only `-p` reads a file), so it isn't listed. `shell-run` `env` shows as `NAME=value`. A command or path that reads `{{fields.*}}` shows the value it will run with (the field's default, or a `-p` or form value the run was given; escaped) beside the template: under a command as a `runs:` row, after a path as `= value`, and a path is judged by that value first. A `file-rename` with `find` set is flagged "edits the whole path, may move files to other folders": find and replace act on the whole relative path, so a plain `replace` can build `..` or `.git`. A `file-list` path is judged under its `base`. `SourceKind` carries the recipe file (`Library(Option)`, `Project { root, file }`, `Path`), `None` when what runs isn't that file's text (a TUI edit). The TUI's prompt scrolls one row at a time (`j`/`k`), so its last row is reached only past every row, and `y` (and the app's own check before it records an approval) allows only then; a panel with fewer than three rows under the pinned ones asks for a bigger terminal and can't be allowed. A panel narrower than 20 columns can't be allowed either, and a new panel width starts the prompt over from its first row (rows re-wrap, so a kept row index could skip some). Long words wrap by display width, so wide characters all show. Files a command runs are found from a relative `searchPath`, a `cwd`, and any word of its args, `env` values or script that reads as a relative path (an option's `--opt=path` too), or, for a project recipe, a bare name of a file in the project root or the step's `cwd`; the prompt says the list may be incomplete. `display_text` escapes recipe text everywhere the CLI prints it (prompt, `list`, `info`, `doctor`, `dry-run`, `check`, `install`'s plan, run progress and messages, errors) and in the TUI (consent, detail and its form labels and values, the editor, library, home, picker, results and run errors). A command's output loses its SGR colour codes (ESC [ … m) before the rest is escaped, so colours vanish instead of showing as `\e[32m`; any other control sequence still shows escaped. **The "outside the project" flag is no protection on its own:** it's read when the user approves, and a later `git pull` can turn `assets/` into a symlink to `~` without changing the recipe's hash; step 7k refuses such a write at run time (§8).

### 7d: Finding the project root

- **What:** `project_root` per §2; an absolute `BNTO_HOME` and `paths.home` (ISS-126); both relative `.bnto` fallbacks removed. Both fallbacks now go through `BntoPaths::resolve_or_relative` (`storage/paths.rs`, since 7b), so removing it removes both. Also from 7b's reviews: the TUI Settings screen decides whether the session home moved by comparing path spellings (`config_is_the_sessions`, `tui/app_helpers/settings.rs`); compare the config files by identity (`same-file`) instead, so a symlinked home can't take the "moved" branch and let a later theme save undo a new home. Not wired to anything yet.
- **Files (~1 new, ~3 modified):** new `project_root.rs`; `storage/paths.rs`, `storage/mod.rs`, `tui/app/init.rs`.
- **Key API:** `fn project_root(start: &Path, home: &Path, bnto_home: &Path) -> Option<PathBuf>`.
- **RED tests:** found in the start folder and three levels up; stops at a `.git` dir and a `.git` file; never returns home (trailing slash, symlinked `/var`, other case); skips a root whose recipes are the library; ignores `recipes/` without the dot; ignores a `.bnto` that's a symlink; stops at another user's folder and at an unreadable parent (Unix); `BNTO_HOME=.bnto` and `BNTO_HOME=` are refused.
- **Verification:** `cargo test -p bnto project_root`, then `task cli:test && task wasm:lint`.
- **Count changes:** none.
- **As built (step 7d):** `project_root/` (`mod.rs`; the owner checks in `folder_owner/`: `unix.rs`, `windows.rs`, `private_group.rs`, `primary_users.rs` and `filesystem.rs`) canonicalizes the start (`dunce`), then climbs its parents. Each folder is checked in this order: the filesystem root stops the walk unchecked on every OS (not only Windows: `/.bnto` is no more a project than `C:\.bnto`); on Unix a folder on another device than the start (`st_dev`) stops it; home stops it; a folder that isn't safe stops it; then `.bnto` and `.bnto/recipes` are read with `symlink_metadata`, and last `.git`. **Safe** (`folder_owner::is_safe_folder`, checked on every visited folder, on `.bnto` and on `.bnto/recipes`): on Unix the owner uid is the effective uid, nobody else may write (world write never; group write only for the user's private group, `private_group.rs`, read with `getpwuid_r`/`getgrgid_r` into a fixed 16 KiB buffer that fails closed on ERANGE, plus a `getpwent` scan behind a mutex for other accounts with that primary group, `primary_users.rs`), the filesystem records owners (`filesystem.rs`, `statfs`), and the sticky bit excuses shared write only on a visited folder (`Role::Visited`), never on `.bnto` or `.bnto/recipes` (`Role::Loaded`); on Windows the owner SID (`GetNamedSecurityInfoW`, `OWNER_SECURITY_INFORMATION`) equals the process token's user SID (`EqualSid`), or is the Administrators group (`IsWellKnownSid`) while `CheckTokenMembership` says the token holds it, as git's check does. Any error is unsafe. The decisions are pure functions (`owned_by(owner, me)`, `writers_allowed(mode, role, …)`, the private group's `decide`, `apple_records_ownership(flags, fstype)`, `linux_records_ownership(magic)`, `accepts_owner`); an ignored macOS test mounts a disk image `-owners off` with `hdiutil` and finds no project on it; the device stop is a plain comparison with no test of its own, unit-tested with other ids, since a real `chown` needs root; the Windows test of a new folder accepts either owner branch and prints which matched, since GitHub's runners are elevated. If home or the bnto home can't be opened, `project_root` returns `None`: the walk couldn't recognise them. Home, the bnto home and the library are compared by identity (`same-file` handles), which settles case, `/var` vs `/private/var`, a trailing slash and a symlinked spelling at once; opening a folder for that is what makes an unreadable folder (no read permission) stop the walk, along with any `symlink_metadata` error other than NotFound. A `.bnto` or `.bnto/recipes` that isn't a real folder (a symlink, a file) is passed over and the walk goes on up, as §1's "first folder that passes" reads; one that isn't safe, a `.bnto` that is the bnto home, or a `recipes` that is the library stops it. The bnto-home stop does work the library one can't: a bnto home with no `recipes` folder (a test pins it). A `.git` of any kind, a symlink too, stops the walk. Nothing calls `project_root` yet: `mod project_root` carries `#[cfg_attr(not(test), expect(dead_code, …))]` until step 7e wires it (a plain `expect` is unfulfilled in every test build, since the tests use the module). ISS-126: `BntoPaths::resolve`, `with_config_override`, `configured` and `resolve_configured` return `Result<_, HomeError>` (`storage/absolute_home.rs`); `BNTO_HOME`, `paths.home` (empty included: it used to be ignored) and a platform home that `dirs` reports relative are refused (its error shows the `$HOME` value, not the joined `.bnto`), the v1 → v2 migration skips itself unless the old home and its folders are absolute (it read `dirs` folders that follow a relative `$HOME`), and so do the other `dirs` uses (the upgrade's install check, the picker's `~`, telemetry's old file), a value starting with `~` adds that `~` isn't expanded, and no home at all is an error, not a relative `.bnto`. `resolve_or_relative` is gone. `main` resolves the home and checks the config's `paths.home` before `storage::ensure_ready(&paths)`, so a relative one is refused even when setting up storage fails, and exits with `Error: BNTO_HOME must be an absolute path, got ".bnto"` (or `paths.home in <config> …`); only then does a failed setup warn, and the catalog loads with no library (`RecipeCatalog::without_library`), not an empty path. The TUI's `AppModel::new` and the run loop's model return the error, so `bnto tui` restores the terminal and prints `TUI error: …`. The CLI's consent and `NativeContext::current_dir` treat an unusable home as none (an empty trust store, a temp folder); `main` exits before any command reaches them that way. The Settings screen compares the two `config.toml` files by identity; when either can't be opened it compares the two homes, and spellings when either home can't be.

### 7e: Loading project recipes

- **What:** §1's file rules and §3's slug rules; skipped entries with reasons; `main.rs` and TUI launch pass the root; `list`, `info`, `doctor`, slug runs; `install --all` leaves project recipes out, `install <project-slug>` always asks, and a `binary` is checked before planning (ISS-124); `doctor` skips version probes for them; §7's test helper and `.git` marker.
- **Files (~1 new, ~10 modified, plus the ~30 test files moved to the helper):** new `bnto/tests/helpers/command.rs`; `catalog.rs`, `main.rs`, `list.rs`, `info.rs`, `doctor.rs`, `install.rs`, `install_plan.rs`, `package_manager.rs`, `tui/app/init.rs`, `tests/tui/harness.rs`.
- **Key API:** `fn RecipeCatalog::load(library: &Path, project: Option<&Path>) -> Self`; `fn RecipeCatalog::skipped(&self) -> &[Skipped]` (file, reason).
- **RED tests:** a colliding slug, a case-only collision, a dotted slug, a doubled suffix, two files with one slug, a symlinked file, a FIFO and a 2 MB file are each skipped and `bnto list` says why; `bnto install --all` in a project plans none of its deps; `owner/tap/x` and `-o x` are refused as a `binary`; a sandboxed `bnto list` inside this repo shows no project section; the bare-command check.
- **From 7d's reviews:** load a project recipe file only when the current user owns it (Unix uid; Windows owner SID, reusing `project_root`'s `folder_owner`), so a file another user planted in the user's folder (they own it) is skipped with a reason. Have `project_root` report why it stopped or passed a folder over (an unsafe folder: group- or world-writable, or not owned; no usable owner; an error; home; a `.git`), so `bnto doctor` can tell the user why no project was found; 7d returns only `Option<PathBuf>`.
- **From 7d's security review:** pass `project_root` the account's home folder from the user database (`getpwuid` on Unix, the profile folder on Windows), not `$HOME`, which `dirs::home_dir` reads first and anyone starting bnto can override, and check both the default bnto home and the configured one (`paths.home`), so the library is recognised whichever home the session uses.
- **Verification:** `task cli:test && task cli:golden && task tui:test && task wasm:lint`.
- **Count changes:** no recipe counts; `bnto list` golden output unchanged in sandboxed tests.

### 7f: Nodes use the run's working folder

- **What:** `file-list` `base`, `file-delete` `base`, `file-move` and `file-copy` `destination`, `spreadsheet-export` `path`, `spreadsheet-lookup` `table` and the output `directory` resolve against `ctx.work_dir()`, as `shell-run` already does; `file-list` refuses an absolute or `..` `base` as its docs say (ISS-123). A joined file name must be all `Normal` components (§8 rule 1) wherever it's joined onto a folder: the output write and `file-copy` (ISS-128). No other behaviour change today, since the working folder is the current folder for every run.
- **Files (~0 new, ~7 modified):** `bnto-file` `list/mod.rs`, `list/params.rs`, `delete/target.rs`, `move/root.rs`, `move/plan.rs`, `copy.rs` (and `copy_batch/` if it resolves its own), `bnto-spreadsheet` `export/processor.rs` and `lookup/source.rs`, `bnto/src/recipe_output_dir.rs` and `run.rs` (output folder).
- **Key API:** each resolver takes `ctx.work_dir()`: e.g. `fn base_root(base: &str, work_dir: &Path) -> Result<PathBuf, String>`.
- **RED tests:** per node, a context whose working folder differs from the process's: delete `base`, move and copy `destination`, list `base`, export `path`, lookup `table` and output `directory` land under the context's folder; `file-list` refuses `/abs` and `../x` as `base`, naming the param; a `file-render` `filename` of `../x` or an absolute path, a Windows root-relative `\x`, a `file-rename` `prefix` of `../x_`, a `find`/`replace` that builds `..` from a path's parts (the review's `dotrep` recipe), and a `file-copy` of a file so named, refuse the run before any file is written (ISS-128); the check runs on each file's final name, after every rename. (The `hook` recipe, which builds `.git/...` with no `..`, is refused only for project recipes, in 7k.)
- **Verification:** `task wasm:test:unit && task cli:test && task wasm:lint`.
- **Count changes:** none.
- **As built (step 7f):** two `bnto-core` helpers hold the rules. `in_work_dir(param, value, ctx)` keeps an absolute value as written and joins a relative one onto `ctx.work_dir()`, less its `.` segments (so `.` is the working folder itself, not `<work>/.`); only a relative value asks for the working folder, so an absolute one still works where there is none (WASM's `NoopContext`, which errs: every node below is CLI-only). `leaves_folder(name)` is §8 rule 1, and `file-list`'s entry check now calls it too. Resolved through `in_work_dir`: `file-list` `base` (refused first, in `ListParams::read`, so `validate` names `'base'` as well), `file-delete` `base`, `file-move` `destination` (`Root::resolve` takes the joined path), `file-copy` `destination` (`dest_dir`/`dest_path`, which `copy_batch`'s replace check and `newer` mode use too), `spreadsheet-export` `path`, `spreadsheet-lookup` `table` (its cache is keyed by the joined path), and the output node's `directory` in `recipe_output_dir`, which `bnto run` and the TUI's precheck share. Names: `file-copy` refuses its batch before any copy (`copy_batch/names.rs`, every bad name listed, by the same `bnto_file::relocate::name_refusal` the output write uses), the CLI's and TUI's output write refuses in `result_names::check_result_names` (write mode, real run and dry run alike, on the result's final names), and a progressive copy skips such a name, leaving the refusal to the final write. Left alone, with reasons: `-o` (the user's, made absolute by 7g) and the TUI's temp output (bnto's); overwrite mode, which keeps only a name's last segment and already refuses `..`; `file-move`'s own `segments` (the same rule plus platform names, with its own messages); `copy_newer.rs` and `relocate/`, which take the destination their callers resolved; `file-collect`, whose folder is its input's path; file paths the run carries (`FileData::Path`, from CLI inputs or `file-list`), which are files rather than params, and which 7g makes absolute. **Decided here:** a `.` segment passes, against rule 1's letter: it stays in the folder, and `find . -print0` lists `./a`, which `file-list` keeps as the name, so refusing it would break a documented input. `/` and `\` both separate and a drive letter counts on every OS (as `file-list`'s entries and `file-move`'s names already did), so `\x` is refused everywhere, not only on Windows, and a recipe means the same on every OS. A relative recipe `directory` now shows as its full path ("1 file written to /…/out/"), and with a working folder that isn't UTF-8 it's refused with a hint to pass `-o`, since the write path keeps the folder as text. In a loop each iteration checks its own batch, so earlier iterations' copies may stand ("this step copied nothing", as `file-move` says). Tests that gave `file-list` an absolute `base` now run in that folder with a relative one. **From 7f's review:** a value that starts at a root or names a drive without being a full path on this OS (`\x` and `C:x` on every OS, `/x` on Windows, read as `bnto_core::is_absolute_anywhere` reads them, which `path_flag` now shares) is refused by `in_work_dir`, naming the param, rather than joined: on Windows joining `\x` replaces the working folder's path below its drive and `C:x` resolves against the process's folder on C; every caller already returned a node error, so none needed a new path. With no `-o` and no recipe `directory`, `bnto run` writes to the run's working folder in full (`default_output_dir`), not the literal `.`, so the default follows the working folder too and "written to" shows the full path; the TUI's temp output stays bnto's choice. The output write (`result_names`, CLI and TUI) and `file-copy` (`copy_batch/names.rs`) run `relocate::check_platform_segments` on each segment, as `file-move` and overwrite mode do: on Windows `CON`, `COM1`, a trailing dot or space (so `.. ` can't act as `..`) and reserved characters are refused; elsewhere only NUL, so `x/.. /y` is a plain folder under the output there. Names lose their `.` and empty segments before planning (`bnto_core::without_dot_segments`, in the output's destinations and progressive copies and in `file-copy`'s `dest_path`), so `./a.txt` and `a.txt` are one name (the second is written as `a_1.txt`), and a name of only `.` or empty segments (`.`, `./.`, an empty name) is refused as having no file name, by the output write and `file-copy` alike. **From 7f's second review:** in `file-copy` such a name was the destination folder itself, so `conflict: rename` took that folder's stem and parent and wrote `<parent>/<folder>_1` beside it (with the default destination, beside the working folder), with no prompt and exit 0. Besides the refusal, `resolve_conflict` now finds a free name in the name's own folder under `destination`, never above it, and debug-asserts its path sits under it. `file-move` already refused it ("has no file name"). A name refusal after a progressive loop names the files the iterations wrote into the output folder (`ordered_write::refusal_in`), worded as a conflict refusal; those copies stand, as do earlier iterations' `file-copy` copies. In message or none mode with a progressive loop (ISS-45), a bad name is skipped by the progressive copy and nothing refuses it: the safe direction, since nothing is written for it. Recipe text in an error is escaped where the error is built: `display_text` moved to `bnto-core` (`bnto::display_text` re-exports it, so there's one escape), and the names `name_refusal` refuses, `in_work_dir`'s refused value, `file-list`'s `base` and refused entries, `file-move`'s name refusals and resolved `destination`, `file-delete`'s `base`, `file-copy`'s destination folder and `spreadsheet-lookup`'s `table` all go through it, as does the recipe `directory` in `recipe_output_dir`'s refusals. The TUI splits an error into rows at each line break, so a raw newline could forge a row: escaped text has none, and `display_text` leaves escaped text as it is, so the CLI's second pass changes nothing (a backslash isn't escaped, so it isn't doubled).

### 7k: Confine project recipes at run time

- **What:** §8: `bnto_core::Confinement`, `ProcessContext::confinement`, the `RunScope` forward, and the check at every write point §8 lists, with an explicit `Option<&Confinement>` through the CLI's and TUI's write path. Nothing sets one yet except tests (7g sets it for project recipes), so no user-visible change lands with this step.
- **Files (~3 new, ~16 modified):** new `bnto-core/src/confine.rs` (+ tests), a `task check` script for raw fs writes in node crates (+ test); `bnto-core/src/context.rs`, `executor/run_scope.rs`; `bnto-file` `delete/`, `relocate/`, `copy.rs`, `copy_newer.rs`, `copy_batch/`; `bnto-spreadsheet` `export/`; `bnto/src/context.rs`, `io.rs`, `ordered_write/`, `progressive_output.rs`, `overwrite/`, `run.rs` (`resolve_effective_output_dir` returns an `OutputPlace`), `dry_run/`, `tui/bridge.rs`, `consent_paths.rs` (the read/write mark); the behavioural guard test beside `consent_paths`.
- **Key API:** `pub struct Confinement` with `fn new(root: &Path) -> io::Result<Self>`, `fn check_write(&self, base: &Path, chosen_by: ChosenBy, name: &Path) -> Result<(), Refusal>` and `fn check_remove(&self, target: &Path) -> Result<(), Refusal>`; `enum ChosenBy { User, Bnto, Recipe }`; `struct OutputPlace { dir, chosen_by }`; `fn confinement(&self) -> Option<&Confinement>` on `ProcessContext`.
- **RED tests:** with a confinement set: `file-delete` of a file outside the root, `file-move` out of or into a place outside it, `file-copy` (and `conflict: newer`) to a folder outside it, `spreadsheet-export` to a path outside it, overwrite mode replacing a listed file outside it, and an output `directory` of `~/x` are each refused before anything changes; each of those into `.git/`, `.GIT/`, and (Windows) `.git.`, `GIT~1` is refused; a `destination` that is a link leaving the root, and a dangling link in the last position pointing outside, are refused; a link inside the root pointing outside may itself be deleted; a write to a missing folder under the root is allowed; `-o` outside the root, overwrite mode on a file the user passed in, and a TUI run with no `directory` (temp output) are allowed; `-o .` at the root with a recipe-built name `.git/hooks/x` is refused; a name that climbs out of a `-o` folder through a link inside it is refused; two links pointing at each other refuse rather than hang; a `RunScope`-wrapped context still reports the confinement (forwarding test); an unreadable ancestor refuses; dry-run reports the refusal; the review's `hook` and `dotrep` recipes, given a confinement, write nothing; with none set, everything behaves as today; the behavioural guard fails for a writing param whose node skips the check.
- **Verification:** `task wasm:test:unit && task cli:test && task tui:test && task wasm:lint`, plus a run on Windows CI (the `.git` aliases).
- **Count changes:** none.

### 7g: Running at the project root

- **What:** §5 for `bnto run` and `dry-run` by slug and the TUI bridge: the context is `NativeContext::new(root, &paths)` with its `Confinement` set (§8, from 7k), passed to the write path too, which already takes a working folder and loads its `.env`; inputs and `-o` made absolute first. The default output folder needs no change: since 7f it is the context's working folder (`recipe_output_dir::default_output_dir`), so it becomes the root with the context.
- **Files (~0 new, ~4 modified):** `run.rs`, `dry_run/mod.rs`, `dry_run/files.rs`, `tui/bridge.rs`.
- **Key API:** `fn run_dir(kind: &SourceKind, cwd: &Path) -> PathBuf`: the root inside `SourceKind::Project { root, .. }`, else `cwd`.
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

Every step runs `task check` and gets a cold review. Steps 7b–7f, 7h and 7k also get a `/security-engineer` review. None adds a built-in recipe or a node.

### Dependency chain

```
7a design record
├── 7b consent by source kind
│   └── 7c prompt shows origin and paths
├── 7d find the project root            (independent of 7b-7c)
├── 7f nodes use the working folder     (independent; ISS-123, ISS-128)
│   └── 7k confine project recipes      (needs 7f's resolvers)
│
7e load project recipes                 (needs 7b, 7c, 7d, 7f and 7k: nothing loads before consent and run-time checks are right)
├── 7g run at the project root          (sets the confine root)
├── 7h editor keeps the source          (needs 7b's kinds and 7e's entries)
└── 7i Project pane                     (needs 7g and 7h)
    └── 7j move this repo's recipes
```

The branch takes 7d next, then 7f, 7k, then 7e.
