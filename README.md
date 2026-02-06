# Bento

**High-performance workflow automation — CLI, desktop, and cloud.**

Bento lets you define automated workflows as `.bento.json` files that orchestrate tasks like image processing, file operations, data transformation, and HTTP requests. Workflows are built from composable nodes connected together to automate complex multi-step processes.

## Why Bento?

- **Fast:** 15-30x faster startup than Node.js, 6x less memory
- **Portable:** Single 20MB binary, cross-platform (macOS, Windows, Linux)
- **Powerful:** True parallelism with goroutines, 10+ built-in node types
- **Simple:** JSON-based workflow definitions, no code required
- **Open Source:** Everything runs locally for free — cloud is optional convenience

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

## Example

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
bento run resize-images.bento.json
```

## CLI Usage

```bash
bento run workflow.bento.json       # Execute a workflow
bento validate workflow.bento.json  # Validate a workflow file
bento list                          # List available bentos
bento new my-workflow               # Create a new workflow
bento secrets set KEY VALUE         # Manage secrets
```

## Repository Structure

This is a monorepo with a Go execution engine and TypeScript frontend packages:

```
bento/
├── engine/              # Go CLI + execution engine
│   ├── cmd/bento/       # CLI binary
│   ├── pkg/             # Go packages (engine, registry, node types, etc.)
│   └── tests/           # Integration tests + fixtures
├── apps/
│   ├── web/             # Next.js cloud app (Phase 1)
│   └── desktop/         # Wails desktop app (Phase 3)
├── packages/
│   └── @bento/
│       ├── core/        # Transport-agnostic API layer
│       ├── ui/          # Design system (shadcn)
│       └── editor/      # Workflow editor components
├── Taskfile.yml         # Go build orchestration
├── turbo.json           # Turborepo config
└── .claude/             # Architecture docs + plan
```

## Development

**Prerequisites:** Go 1.21+, Node.js 18+, pnpm, [Task](https://taskfile.dev)

```bash
# Install dependencies
pnpm install

# Build
task build               # Go engine
task ui:build            # Frontend packages
task build:all           # Everything

# Test
task test                # Go tests (with race detector)
task ui:test             # Frontend tests
task test:all            # Everything

# Run
task vet                 # Go vet
task ui:dev              # Frontend dev server
```

## Architecture

Bento follows the **Bento Box Principle** — every file, function, and package does one thing well.

- **Go Engine** (`engine/`): Workflow execution, validation, all node types
- **@bento/core**: Transport-agnostic API — same interface for cloud (Convex), desktop (Wails), and REST
- **@bento/ui**: Shared design system across web and desktop
- **@bento/editor**: Workflow editor (JSON for now, visual drag-and-drop later)

See [.claude/PLAN.md](.claude/PLAN.md) for the full roadmap and [.claude/strategy/](.claude/strategy/) for architecture decisions.

## License

MIT
