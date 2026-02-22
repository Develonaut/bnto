# Bnto

**The one place small teams go to get things done.**

Compress images, clean a CSV, rename files, call an API -- without the overhead of a platform or the fragility of a script. Simple by default, powerful when you need it.

Bnto lets you define automated workflows as `.bnto.json` files that orchestrate tasks like image processing, file operations, data transformation, and HTTP requests. Workflows are built from composable nodes -- the same tool that compresses a folder of images can power a 20-node pipeline calling external APIs.

## Why Bnto?

- **Fast:** 15-30x faster startup than Node.js, 6x less memory
- **Portable:** Single binary, cross-platform (macOS, Windows, Linux)
- **Powerful:** True parallelism with goroutines, 10 built-in node types
- **Simple:** JSON-based workflow definitions, no code required
- **Open Source:** MIT licensed -- everything runs locally for free, cloud is optional convenience

## Workflow Nodes

| Node Type | Purpose |
|-----------|---------|
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

## Quick Start

**Prerequisites:** Go 1.25+, [Task](https://taskfile.dev)

```bash
# Build from source
git clone https://github.com/Develonaut/bnto.git
cd bnto
task build

# Run a workflow
bnto run engine/examples/csv-to-folders.bnto.json
```

## Example Workflow

Create a file called `resize-images.bnto.json`:

```json
{
  "id": "resize-images",
  "type": "group",
  "name": "Batch Resize Images",
  "nodes": [
    {
      "id": "find-images",
      "type": "file-system",
      "parameters": {
        "operation": "list",
        "source": "./photos",
        "pattern": "*.{jpg,png}"
      }
    },
    {
      "id": "resize-each",
      "type": "loop",
      "parameters": {
        "mode": "forEach",
        "items": "{{index . \"find-images\" \"files\"}}"
      },
      "nodes": [
        {
          "id": "resize",
          "type": "image",
          "parameters": {
            "operation": "resize",
            "source": "{{.item}}",
            "width": 800,
            "dest": "./output/{{basenameNoExt .item}}.webp"
          }
        }
      ]
    }
  ]
}
```

```bash
bnto run resize-images.bnto.json
```

## CLI Usage

```bash
bnto workflow.bnto.json                # Execute a workflow (shorthand)
bnto run workflow.bnto.json            # Execute a workflow (explicit)
bnto run workflow.bnto.json --dry-run  # Preview without running
bnto validate workflow.bnto.json       # Validate a workflow file
bnto list                              # List available bntos
bnto new my-workflow                   # Create a new workflow from template
bnto secrets set KEY VALUE             # Manage secrets
bnto logs                              # View execution logs
bnto docs                              # View documentation
```

File paths support multiple formats:

```bash
bnto examples/workflow.bnto.json   # Full path with extension
bnto examples/workflow             # Full path without extension
bnto workflow                      # Name from ~/.bnto/bntos/
```

## Project Structure

```
bnto/
├── engine/                 # Go CLI + execution engine
│   ├── cmd/bnto/           # CLI binary
│   ├── pkg/                # Go packages
│   │   ├── api/            # Shared service layer
│   │   ├── engine/         # Workflow orchestration
│   │   ├── registry/       # Node type registration
│   │   ├── node/           # Node type implementations
│   │   ├── validator/      # Workflow validation
│   │   ├── storage/        # Persistent storage
│   │   ├── paths/          # Path resolution + config
│   │   ├── logger/         # Logging
│   │   ├── logs/           # Log file management
│   │   └── secrets/        # Secrets management
│   ├── tests/              # Integration tests + fixtures
│   └── examples/           # Example .bnto.json files
├── apps/
│   ├── api/                # Go HTTP API server
│   ├── web/                # Next.js cloud app
│   └── desktop/            # Wails v2 desktop app
├── packages/
│   ├── core/              # Transport-agnostic API layer
│   ├── ui/                # Design system (shadcn)
│   ├── editor/            # Workflow editor components
│   └── @bnto/
│       ├── auth/          # Cloud authentication
│       └── backend/       # Convex schema + functions
├── Taskfile.yml            # Go + cross-cutting orchestration
├── turbo.json              # Turborepo config
├── go.work                 # Go workspace (engine + apps/api)
└── .claude/                # Architecture docs + plan
```

## Development

**Prerequisites:** Go 1.25+, Node.js 18+, pnpm, [Task](https://taskfile.dev)

```bash
# Install frontend dependencies
pnpm install

# Build
task build               # Go engine CLI
task ui:build            # Frontend packages (Turborepo)
task build:all           # Everything

# Test
task test                # Go engine tests (with race detector)
task api:test            # API server tests
task ui:test             # Frontend tests
task test:all            # Everything

# Lint & vet
task vet                 # Go vet
task ui:lint             # Frontend linting

# Dev
task ui:dev              # Frontend dev server

# Quality gate
task check               # Full check (vet + test + build)
```

## Architecture

Bnto follows the **Bento Box Principle** -- every file, function, and package does one thing well.

```
Apps (web/desktop) -> @bnto/editor -> @bnto/ui -> @bnto/core -> Go Engine
```

- **Go Engine** (`engine/`): Workflow execution, validation, all node types. The source of truth.
- **`@bnto/core`**: Transport-agnostic API layer. Same React hooks work for cloud (Convex), desktop (Wails), and REST. Components never know which backend they are talking to.
- **`@bnto/ui`**: Shared design system built on shadcn/ui + Tailwind CSS.
- **`@bnto/editor`**: Workflow editor -- JSON editor now, visual drag-and-drop later.

The desktop app (Wails v2) renders the same React frontend in a system webview. `@bnto/core` detects the runtime and swaps transport adapters internally, so there is no separate frontend for desktop.

For a deep dive into architecture decisions, see [`.claude/strategy/`](.claude/strategy/) and the full roadmap in [`.claude/PLAN.md`](.claude/PLAN.md).

## Contributing

Contributions are welcome. To get started:

1. Fork and clone the repository
2. Install prerequisites (Go 1.25+, Node.js 18+, pnpm, Task)
3. Run `pnpm install` and `task build` to verify your setup
4. Create a branch for your changes
5. Run `task check` before submitting a pull request

Please follow the existing code patterns and the [Bento Box Principle](.claude/rules/code-standards.md) -- small, focused files and functions with clear boundaries.

## License

[MIT](LICENSE) -- Copyright 2024-2026 [Develonaut](https://github.com/Develonaut)
