# Bento Engine

**Go execution engine for Bento workflow automation.**

This directory contains the core Go CLI and execution engine. It handles workflow parsing, validation, orchestration, and all node type execution.

## Packages

| Package | Purpose |
|---------|---------|
| `engine` | Workflow orchestration and execution |
| `registry` | Node type registration and lookup |
| `storage` | Persistent storage layer |
| `validator` | Workflow definition validation |
| `paths` | Path resolution and config management |
| `logger` | Structured logging |
| `secrets` | Encrypted secrets management |
| `tui` | Terminal UI (Bubble Tea) |
| `node` | All workflow node type implementations |

## Node Types

| Type | Purpose |
|------|---------|
| `edit-fields` | Field editing with Go templates |
| `http-request` | HTTP client with retry support |
| `file-system` | File operations (copy, move, mkdir, list) |
| `shell-command` | Shell execution with stall detection |
| `group` | Sequential/parallel node execution |
| `loop` | Iteration (forEach, times, while) |
| `parallel` | Advanced parallel execution |
| `spreadsheet` | Excel/CSV reading and processing |
| `image` | Image resize, export, composite |
| `transform` | Data transformation via expressions |

## Build & Test

```bash
# From repo root (via Taskfile)
task build    # Build CLI binary
task test     # Run tests with race detector
task vet      # Run go vet

# Or directly
cd engine
go build ./cmd/bento
go test -race ./...
```

## Module

```
module github.com/Develonaut/bento
```

The module path is unchanged from the original repo. Go resolves all imports relative to `go.mod` in this directory.
