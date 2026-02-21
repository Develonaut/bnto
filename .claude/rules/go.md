# Go Code Standards

## Package Design

- **One responsibility per package** — engine orchestrates, registry registers, storage persists
- **Accept interfaces, return structs** — keep interfaces small (1-3 methods)
- **No circular dependencies** — packages only depend downward
- **`pkg/` not `internal/`** — engine packages are importable by `apps/api/` via `go.work`

## Error Handling

- **Always wrap errors with context:** `fmt.Errorf("loading workflow %s: %w", path, err)`
- **Never bare `return err`** — every error needs context for debugging
- **Never swallow errors** — if you ignore a returned error, add a comment explaining why

## Context Propagation

- **All long-running operations accept `context.Context`** as first parameter
- **Check cancellation in loops** — `select { case <-ctx.Done(): ... }`
- **Pass context through the entire chain** — never create new contexts mid-chain

## File and Function Size

- **Files: < 250 lines** (hard max 500)
- **Functions: < 20 lines** (hard max 30)
- **If it's getting long, split it** — new file, new function, new package

## Node Type Implementation

Each node type in `engine/pkg/node/library/` follows the same pattern:

```go
// node.go — implements NodeType interface
type MyNode struct{}
func (n *MyNode) Execute(ctx context.Context, input *node.Input) (*node.Output, error) { ... }

// factory.go — creates node instances
func Factory() node.Factory { return func() node.NodeType { return &MyNode{} } }
```

- Register in `engine/pkg/api/service.go` via `DefaultRegistry()`
- Unit tests in the same package
- Integration test with fixture `.bnto.json` in `engine/tests/integration/`

## Testing

- **`go test -race`** — always run with race detector
- **Table-driven tests** — use subtests for multiple cases
- **`httptest`** — for HTTP handler tests (no real servers)
- **Test fixtures** — `.bnto.json` files in `engine/tests/fixtures/`

## Naming

- **Packages:** lowercase, single word (`engine`, `registry`, `storage`)
- **Files:** lowercase with underscores (`edit_fields.go`, `shell_command.go`)
- **No `utils.go` or `helpers.go`** — name files by what they contain
- **Exported types:** PascalCase, unexported: camelCase
