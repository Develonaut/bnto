# Decision: Monorepo Build Tooling

**Date:** 2026-02-06
**Status:** Decision Required
**Context:** Choosing between Taskfile.dev, Turborepo, or a hybrid for Bnto's polyglot monorepo

---

## The Problem

Bnto's monorepo will contain both Go and TypeScript code:

| Component | Language | Build Tool |
|-----------|----------|-----------|
| CLI/Engine | Go | `go build`, `go test` |
| HTTP API Server | Go | `go build`, `go test` |
| Wails Desktop | Go + Vite/React | `wails build`, pnpm |
| Next.js Web App | TypeScript/React | `next build`, pnpm |
| @bnto/core | TypeScript | `tsc`, pnpm |
| @bnto/ui | TypeScript | `tsc`, pnpm |
| @bnto/editor | TypeScript | `tsc`, pnpm |

We need a build orchestrator that handles both ecosystems. The previous strategy chose Taskfile.dev. The user prefers Turborepo from past experience. This document evaluates the options.

---

## Options Evaluated

### Option A: Turborepo for Everything

**How it works:** Turborepo manages all packages. Go directories get a `package.json` wrapper that calls `go build`/`go test` via npm scripts.

**Pros:**
- Single tool to learn
- Excellent caching for TS packages (local + remote)
- Dependency graph awareness (builds in correct order automatically)
- Familiar to the user from past projects
- `turbo run build` builds everything in parallel with caching

**Cons:**
- Go is a second-class citizen — requires `package.json` wrappers in Go directories
- Turborepo doesn't understand `go.mod` dependencies natively
- Requires Node.js installed everywhere (including CI for Go-only changes)
- Turborepo's own team closed a request for native Go support: "not on our roadmap" (issue #5004)
- Can conflict with Go's own build cache (`GOCACHE`)

### Option B: Taskfile.dev for Everything

**How it works:** Task orchestrates both Go and TS builds. pnpm workspaces handles TS package linking. Task calls `pnpm -r build` for TS side.

**Pros:**
- Polyglot native — no language is second-class
- No Node.js dependency for Go builds
- Wails v3 chose Task as its official build system
- Simple YAML configuration
- Zero dependencies (single binary)

**Cons:**
- No remote caching (local fingerprinting only)
- No automatic dependency graph for TS packages (must declare order manually, or delegate to `pnpm -r`)
- Less familiar to the user
- No built-in parallel task orchestration based on dependency graph

### Option C: Turborepo for `ui/` + Task for Root

**How it works:** Root `Taskfile.yml` orchestrates everything. The `ui/` directory has its own `turbo.json` and Turborepo manages TS packages. Task calls `pnpm turbo run build` for the frontend.

**Pros:**
- Best-in-class caching for TS packages
- Go uses native tooling directly
- Each ecosystem gets its ideal tool

**Cons:**
- Two build systems to learn and maintain
- More complex CI configuration
- Extra dependency (Task + Turborepo)
- Overkill for 3-5 TS packages

### Option D: pnpm Workspaces Only (No Orchestrator for TS)

**How it works:** pnpm handles all TS package management natively. `pnpm -r build` builds in topological order. A root Taskfile or Makefile handles Go + calls pnpm.

**Pros:**
- Simplest setup — no additional tool beyond pnpm
- pnpm already handles dependency ordering
- For 3-5 packages, build times are seconds anyway

**Cons:**
- No caching at the orchestrator level
- No remote cache sharing in CI
- Still needs something (Task/Make) for Go orchestration

---

## Recommendation: Turborepo for `ui/` + Taskfile for Root (Option C)

**But with an important nuance:** Start with Option D (pnpm workspaces + Task) and add Turborepo to `ui/` when build times justify it. The migration from pnpm scripts to Turborepo inside `ui/` is trivial — you add `turbo.json` and change `pnpm -r build` to `pnpm turbo run build`.

### Why This Path

1. **User familiarity matters.** Turborepo is comfortable territory. When the `ui/` packages exist, Turborepo should manage them.

2. **Go shouldn't be wrapped in `package.json`.** Forcing Go through Turborepo's JS-centric model creates friction. Task handles Go natively.

3. **Pragmatic phasing.** Phase 0 is engine solidification (Go only, no TS at all). Phase 1 introduces the first TS packages. Turborepo isn't needed until Phase 1.

4. **pnpm does the heavy lifting.** Even without Turborepo, `pnpm -r build` handles dependency ordering. Turborepo adds caching on top — valuable but not critical at small scale.

### Concrete Setup (Implemented)

```
bnto/
├── package.json              # Turborepo root workspace
├── pnpm-workspace.yaml       # pnpm workspace config
├── turbo.json                # Turborepo task config
├── Taskfile.yml              # Go orchestration
├── apps/
│   ├── web/                  # @bnto/web (Next.js)
│   └── desktop/              # @bnto/desktop (Wails frontend)
├── packages/
│   └── @bnto/               # Scoped internal packages (n8n pattern)
│       ├── core/             # @bnto/core
│       ├── ui/               # @bnto/ui
│       └── editor/           # @bnto/editor
└── engine/                   # All Go code
    ├── go.mod
    ├── cmd/bnto/
    ├── pkg/
    └── tests/
```

**Root Taskfile.yml:**
```yaml
version: '3'

tasks:
  # Go tasks (engine/)
  build:
    dir: ./engine
    cmds: [go build ./cmd/bnto]
  test:
    dir: ./engine
    cmds: [go test -race ./...]

  # Frontend tasks (Turborepo at root)
  ui:build:
    cmds: [pnpm turbo run build]
  ui:dev:
    cmds: [pnpm --filter @bnto/web dev]
  ui:test:
    cmds: [pnpm turbo run test]

  # Cross-cutting
  build:all:
    cmds:
      - task: build
      - task: ui:build
  test:all:
    cmds:
      - task: test
      - task: ui:test
```

**turbo.json (at root):**
```json
{
  "$schema": "https://turbo.build/schema.json",
  "tasks": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**"]
    },
    "test": {
      "dependsOn": ["build"]
    },
    "lint": {},
    "dev": {
      "cache": false,
      "persistent": true
    }
  }
}
```

### When to Upgrade

| Trigger | Action |
|---------|--------|
| Phase 1 starts (first TS packages created) | Add `turbo.json` to `ui/` |
| CI frontend builds exceed 3 minutes | Enable Turborepo remote caching |
| Team grows to 3+ developers | Remote caching becomes essential |
| Wails v3 goes stable | Task is already compatible (Wails v3 native) |

---

## Decision Summary

| Aspect | Choice |
|--------|--------|
| Root orchestrator | **Taskfile.dev** (polyglot, Go-native, Wails-aligned) |
| Frontend orchestrator | **Turborepo** (caching, dependency graph, user familiarity) |
| Package management | **pnpm workspaces** (linking, dependency resolution) |
| Go builds | **Native Go toolchain** via Task (never wrapped in package.json) |

This gives us the best of both worlds: Turborepo where it shines (TypeScript packages) and Task where Turborepo doesn't (Go, cross-language orchestration, Wails).
