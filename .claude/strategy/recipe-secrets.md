# Recipe Secrets & Environment Variable Management

**Last Updated:** April 27, 2026
**Status:** Strategy doc (no code)
**Depends on:** `{{env.*}}` template namespace (PR #468, shipped)

---

## Problem Statement

Recipes that wrap external tools often need credentials: API keys, auth cookies, tokens, database URLs. Today, the only mechanism is `{{env.*}}` which reads raw system environment variables. This works for power users who set vars in their shell profile, but has no security boundary, no redaction, no dotenv support, and no per-target resolution strategy.

The `download-video` recipe is the concrete example: yt-dlp supports `--cookies-from-file` and `--username`/`--password` flags, but the recipe has no way to accept credentials without the user hardcoding them into a custom recipe file or setting system env vars manually.

As recipes grow more powerful (AI API calls, authenticated HTTP requests, database queries), a clear secrets story becomes essential.

---

## Design Principles

1. **Recipes never contain secrets.** A `.bnto.json` file must be safe to commit, share, and publish. Credentials are always resolved at execution time from external sources.
2. **Resolution is target-specific.** The CLI reads from the host environment. The browser prompts the user. A future server reads from a vault. The recipe doesn't know or care which.
3. **Least surprise.** `{{env.API_KEY}}` works exactly like `$API_KEY` in a shell script. No magic prefix, no bnto-specific syntax for basic env vars.
4. **Defense in depth.** Secrets are redacted in logs, dry-run output, and execution history. Accidental exposure in a terminal scrollback or shared screenshot is the most common secret leak vector.
5. **Opt-in complexity.** Basic env var access works with zero config. Dotenv and vault support are progressive enhancements for users who want them.

---

## Current State

### What exists today

| Feature                        | Status                                 | Location                             |
| ------------------------------ | -------------------------------------- | ------------------------------------ |
| `{{env.*}}` template namespace | Shipped (PR #468)                      | `bnto-core/src/executor/template.rs` |
| `ProcessContext::env_var()`    | Shipped                                | `bnto-core/src/context.rs:38`        |
| `NativeContext` (CLI)          | Reads `std::env::var()`                | `bnto/src/context.rs:158-160`        |
| `NoopContext` (browser)        | Returns `None`                         | `bnto-core/src/context.rs:61-62`     |
| `bnto dry-run`                 | Shows resolved templates               | `bnto/src/commands/dry_run.rs`       |
| Shell env sanitization         | Strips dangerous vars from `env` param | `bnto-shell/src/execute.rs:106-112`  |

### What's missing

| Gap                                 | Impact                                                                            |
| ----------------------------------- | --------------------------------------------------------------------------------- |
| No dotenv support                   | Users must export vars in shell profile; no project-local `.env` files            |
| No secret redaction                 | `bnto dry-run` and execution logs expose resolved `{{env.*}}` values in cleartext |
| No browser secret prompt            | `{{env.*}}` silently resolves to empty string in WASM — no user feedback          |
| No recipe-level secret declarations | Recipes can't declare "I need `API_KEY`" — users discover missing vars at runtime |
| No yt-dlp auth integration          | `download-video` has no cookie/credential fields                                  |
| Shell arg bypass                    | `{{env.*}}` in shell-command args bypasses the `env` param sanitization           |

---

## Resolution Per Target

The key insight: `ProcessContext` is already the abstraction boundary. Each target implements `env_var()` differently. Secrets management extends this pattern.

### CLI (NativeContext)

**Resolution order** (first match wins):

1. **System environment** — `std::env::var(key)` (current behavior)
2. **Project `.env` file** — `.env` in the recipe's working directory (new)
3. **User dotenv** — `$XDG_CONFIG_HOME/bnto/.env` or `~/.config/bnto/.env` (new, uses existing `BntoPaths`)

**Why this order:** System env vars take priority because they're the most explicit (the user actively exported them). Project `.env` is next for per-project credentials. User dotenv is the fallback for global defaults like `OPENAI_API_KEY`.

**Implementation sketch:**

```rust
// In NativeContext::env_var()
fn env_var(&self, key: &str) -> Option<String> {
    // 1. System environment (current behavior)
    if let Ok(val) = std::env::var(key) {
        return Some(val);
    }
    // 2. Project .env (loaded once at context creation)
    if let Some(val) = self.project_env.get(key) {
        return Some(val.clone());
    }
    // 3. User dotenv (~/.config/bnto/.env)
    if let Some(val) = self.user_env.get(key) {
        return Some(val.clone());
    }
    None
}
```

**Dotenv loading:** Parse `.env` files at `NativeContext` construction time (not per-resolution). Use a simple `KEY=VALUE` parser — no need for a full dotenv crate. Lines starting with `#` are comments. No shell expansion, no multiline values, no `export` prefix support. Keep it dead simple.

### Browser (NoopContext → PromptContext)

**Resolution:** Prompt the user via the UI. When a recipe declares required secrets (see "Recipe-Level Declarations" below), the browser adapter shows input fields before execution.

**Phased approach:**

- **Phase 1 (no change):** `{{env.*}}` returns empty string in browser. Recipes that need env vars are CLI-only (the `download-video` pattern — `requires` already blocks browser execution for these).
- **Phase 2 (future):** If a recipe declares `secrets`, the browser execution adapter shows a credentials form before running. Values are held in memory only — never persisted to localStorage, IndexedDB, or Convex.
- **Phase 3 (future):** Browser could read from a password manager integration or browser extension. Deep backlog.

### Server (future SandboxedContext)

**Resolution:** Server-side execution reads from a secrets vault (infrastructure TBD). The recipe author never manages server credentials directly.

**Candidates:** Environment variables injected by the hosting platform (Railway, Fly.io), HashiCorp Vault, AWS Secrets Manager. Decision deferred to the cloud execution sprint (see `cloud-execution.md`).

**Constraints:**

- Server recipes run in sandboxed containers — `env_var()` only sees vars explicitly injected by the platform
- User-specific secrets (API keys) are stored per-user in the vault, injected at container startup
- Built-in recipes (e.g., AI node calling OpenAI) use platform-managed keys, not user keys

### Desktop (future TauriContext)

**Resolution:** Same as CLI. Tauri links the engine natively, so `NativeContext` (or a thin `TauriContext` wrapper) applies. Desktop can additionally integrate with system keychains (macOS Keychain, Windows Credential Manager, Linux Secret Service) in the future.

---

## Recipe-Level Secret Declarations

Recipes should declare what secrets they need. This enables:

- **Pre-flight validation:** `bnto run` can check for missing secrets before starting execution
- **`bnto doctor` integration:** Report missing secrets alongside missing binaries
- **Browser prompting:** The web adapter knows what to ask for
- **Documentation:** Users know what env vars to set

### Proposed schema

Add an optional `secrets` array to `PipelineDefinition`:

```json
{
  "id": "ai-summarize",
  "secrets": [
    {
      "key": "OPENAI_API_KEY",
      "description": "OpenAI API key for GPT-4 calls",
      "required": true
    },
    {
      "key": "OPENAI_ORG_ID",
      "description": "OpenAI organization ID (optional)",
      "required": false
    }
  ],
  "nodes": [...]
}
```

**Key rules:**

- `key` matches the `{{env.KEY}}` placeholder used in node params
- `required: true` means the pipeline refuses to start without it
- `required: false` means the pipeline runs with a fallback (empty string or default)
- The `secrets` array is purely declarative — it doesn't change how `{{env.*}}` resolves, it just enables pre-flight checks

**Validation at execution time:**

```
$ bnto run ai-summarize input.txt
Error: Missing required secret: OPENAI_API_KEY
  Set it via: export OPENAI_API_KEY=sk-...
  Or add to ~/.config/bnto/.env
```

---

## Threat Model

### Attack surfaces

| Vector                          | Risk                                                                                   | Mitigation                                                                                                                                                                                      |
| ------------------------------- | -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Secret in recipe file**       | User hardcodes `API_KEY=sk-...` in `.bnto.json` and commits it                         | Recipes never contain secret values — only `{{env.*}}` references. Lint/validate could warn if a recipe param looks like a literal API key                                                      |
| **Secret in logs**              | `bnto dry-run` or execution logs show resolved `{{env.API_KEY}}` as `sk-abc123...`     | Redact `{{env.*}}` values in dry-run output and execution logs. Show `{{env.API_KEY}}` or `***` instead of the resolved value                                                                   |
| **Secret in shell args**        | `{{env.DB_PASSWORD}}` in shell-command args passes the raw value to the child process  | This is intentional — the external tool needs the real value. The risk is in logging, not in arg passing. Redact in logs, not in execution                                                      |
| **Secret in process env**       | Child processes inherit the full parent environment                                    | Already mitigated: `bnto-shell` sanitizes the `env` param. But `{{env.*}}` in `args` is intentionally passed through. Consider `--no-inherit-env` for shell-command (future)                    |
| **Secret in shared recipe**     | User shares a recipe referencing `{{env.MY_SECRET}}` — recipient doesn't have that var | Pre-flight validation (see declarations above) catches this before execution. Clear error message                                                                                               |
| **Secret in browser**           | User enters API key in browser prompt, key visible in memory/devtools                  | Browser secrets are held in JS memory only, never persisted. Cleared after execution. Standard browser security model applies — if the user's browser is compromised, they have bigger problems |
| **Secret in execution history** | Convex stores execution logs with resolved param values                                | Don't store resolved `{{env.*}}` values in execution history. Store the template, not the resolved value                                                                                        |

### Accepted risks

- **Terminal scrollback:** If a user runs `echo $API_KEY` or a verbose tool prints credentials to stderr, that's visible in the terminal. Bnto can redact its own output but can't control what external tools print. This is the same risk as running any CLI tool.
- **Dotenv file permissions:** `.env` files are regular files. Users are responsible for setting appropriate permissions (`chmod 600`). Bnto could warn if permissions are too open (like SSH does for keys), but this is a Phase 2 concern.
- **Memory-resident secrets:** Secrets loaded from dotenv or system env are held in process memory. This is standard for every CLI tool. Memory-safe Rust helps (no dangling pointers to stale secrets), but a core dump could expose them. Zeroize-on-drop is a Phase 3 concern if bnto ever handles payment credentials.

---

## Redaction Strategy

**Rule: Never show resolved `{{env.*}}` values in user-visible output.**

### Where redaction applies

| Surface                    | Current behavior               | Target behavior                                            |
| -------------------------- | ------------------------------ | ---------------------------------------------------------- |
| `bnto dry-run`             | Shows resolved values          | Show `{{env.KEY}}` placeholder or `$KEY` — never the value |
| CLI execution log          | Shows full command args        | Redact args that came from `{{env.*}}` with `***`          |
| TUI execution screen       | Shows stderr/stdout from tools | No change — bnto can't control what tools print            |
| Execution history (Convex) | Stores resolved params         | Store template params, not resolved values                 |
| Error messages             | May include resolved values    | Scrub env values from error context                        |

### Implementation approach

The template resolver already knows which values came from `{{env.*}}` (the `resolve_placeholder` function branches on the `"env"` prefix). A `ResolvedParam` wrapper could track provenance:

```rust
enum ParamSource {
    Literal,      // hardcoded in recipe
    Field,        // from user-provided field value
    Env,          // from {{env.*}} — REDACT in logs
    Ctx,          // from {{ctx.*}}
    NodeOutput,   // from {{node.*}}
}
```

This is a future implementation detail — the key decision is that `Env`-sourced values are always redacted in output surfaces.

---

## Phased Implementation

### Phase 1 — Dotenv support + pre-flight validation (next sprint, ~1 PR)

- Load `.env` from working directory and `~/.config/bnto/.env` in `NativeContext`
- Simple `KEY=VALUE` parser (no crate dependency — keep it < 50 lines)
- `secrets` array in `PipelineDefinition` schema
- Pre-flight check: fail with clear message if required secrets are missing
- `bnto doctor` shows secret status alongside binary status

### Phase 2 — Redaction (follow-up PR)

- Redact `{{env.*}}` values in `bnto dry-run` output
- Redact env-sourced values in CLI execution log lines
- Don't store resolved env values in Convex execution history
- Warn if `.env` file has overly permissive permissions

### Phase 3 — Browser prompting (future, with cloud execution)

- Browser adapter shows credential form when recipe declares `secrets`
- Values held in JS memory only, cleared after execution
- No persistence — user re-enters credentials each session (or uses a browser extension)

### Phase 4 — Advanced (deep backlog)

- System keychain integration (macOS Keychain, Windows Credential Manager)
- Secret rotation detection (warn if a secret hasn't been rotated in N days)
- Vault integration for server-side execution
- `zeroize`-on-drop for sensitive values in memory

---

## Decisions

| Decision            | Choice                                    | Rationale                                                                                           |
| ------------------- | ----------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Dotenv parser       | Hand-rolled, no crate                     | < 50 lines, no `dotenvy` dependency. Bnto has zero runtime deps beyond std+serde — keep it that way |
| Resolution order    | system > project .env > user .env         | Most explicit wins. Matches Docker, Railway, and most CLI tools                                     |
| Secret syntax       | `{{env.*}}` (existing)                    | No new syntax. Secrets are just env vars with a declaration layer. KISS                             |
| Recipe declarations | `secrets[]` array on `PipelineDefinition` | Declarative, validates before execution, enables browser prompting                                  |
| Redaction scope     | Env-sourced values only                   | Field values and ctx values are not secrets — only env vars need redaction                          |
| Browser Phase 1     | No change (empty string)                  | CLI-only recipes already gated by `requires`. Browser prompting is Phase 3                          |
| Zeroize             | Deferred                                  | Standard CLI trust model. Revisit if bnto handles payment credentials                               |

---

## Non-Goals

- **Encrypted recipe files.** Recipes are plaintext JSON. Encryption adds complexity with no benefit — secrets aren't in the recipe.
- **Per-user secret namespacing.** The CLI runs as the current OS user. There's only one user. Multi-user secrets are a server concern (Phase 4).
- **Secret injection via flags.** `bnto run --secret API_KEY=sk-...` would put secrets in shell history. Use env vars or dotenv instead.
- **Automatic secret detection in recipes.** Linting for hardcoded API keys (regex for `sk-`, `ghp_`, etc.) is nice but unreliable. Out of scope for this design.

---

## Open Questions

1. **Should `bnto dry-run` show secret _names_ but not values?** e.g., `--api-key {{env.OPENAI_API_KEY}}` vs `--api-key ***`. Showing the name helps debugging; showing `***` is more conservative. Recommendation: show the name (it's not a secret — the name is in the recipe file).

2. **Should dotenv files support `export` prefix?** Some users write `export API_KEY=value` in `.env` for shell compatibility. The simple parser could strip `export ` prefix. Low effort, high compatibility. Recommendation: yes, strip it.

3. **Should `bnto doctor` check secrets?** If a recipe declares `secrets`, `bnto doctor` could report which are set and which are missing. This is useful but changes `doctor` from "check binaries" to "check prerequisites." Recommendation: yes, extend doctor. It already checks `requires` (binaries) — `secrets` is the same concept for env vars.
