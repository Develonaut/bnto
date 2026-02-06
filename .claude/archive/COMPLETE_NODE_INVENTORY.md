# Complete Atomiton Node Inventory for Bento

**Date:** 2025-10-18
**Purpose:** Complete 1:1 catalog of ALL nodes in @atomiton/nodes for Go migration
**Requirement:** EVERY node must be ported - zero exceptions

---

## Node Count

**Total Nodes:** 10
**Status:** All nodes cataloged and ready for Go migration

---

## Complete Node Registry

From `@atomiton/nodes/src/executables/registry.ts`:

```typescript
nodeExecutableRegistry.set("parallel", parallelExecutable);
nodeExecutableRegistry.set("group", groupExecutable);
nodeExecutableRegistry.set("edit-fields", editFieldsExecutable);
nodeExecutableRegistry.set("image", imageExecutable);
nodeExecutableRegistry.set("http-request", httpRequestExecutable);
nodeExecutableRegistry.set("file-system", fileSystemExecutable);
nodeExecutableRegistry.set("transform", transformExecutable);
nodeExecutableRegistry.set("shell-command", shellCommandExecutable);
nodeExecutableRegistry.set("spreadsheet", spreadsheetExecutable);
nodeExecutableRegistry.set("loop", loopExecutable);
```

---

## Node-by-Node Analysis

### 1. **edit-fields** - Field Editor

**Type:** Data Transformation
**Complexity:** ⭐⭐ LOW
**Dependencies:** Template engine (Squirrelly → Go: `text/template`)

**Purpose:** Edit/transform object fields using templates

**Parameters:**
- `values` (object | JSON string) - Fields and values to set/edit
- `keepOnlySet` (boolean, default: false) - Only output defined fields

**TypeScript Example:**
```typescript
{
  type: "edit-fields",
  parameters: {
    values: {
      fullName: "{{firstName}} {{lastName}}",
      status: "active",
      timestamp: "{{$now}}"
    },
    keepOnlySet: false
  }
}
```

**Go Implementation:**
```go
pkg/nodes/library/editfields/
├── editfields.go      # Main executable
├── template.go        # Template parsing
└── variables.go       # Variable extraction

// Using text/template
tmpl := template.New("field")
tmpl.Parse(value)
result := tmpl.Execute(data)
```

**Go Libraries:**
- `text/template` (stdlib) - Template engine
- Go's string interpolation

**Migration Effort:** 2-3 days

---

### 2. **http-request** - HTTP Client

**Type:** I/O
**Complexity:** ⭐⭐ LOW
**Dependencies:** HTTP client (fetch → Go: `net/http`)

**Purpose:** Make HTTP requests

**Parameters:**
- `method` (enum: GET, POST, PUT, DELETE, default: GET)
- `url` (string, required) - Request URL
- `headers` (object | JSON string, default: {}) - Headers
- `body` (string | JSON string, optional) - Request body

**TypeScript Example:**
```typescript
{
  type: "http-request",
  parameters: {
    method: "POST",
    url: "https://api.example.com/users",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name: "Alice" })
  }
}
```

**Go Implementation:**
```go
pkg/nodes/library/http/
├── http.go           # Main executable
├── client.go         # HTTP client configuration
├── auth.go           # Authentication
└── response.go       # Response parsing

// Using net/http
client := &http.Client{Timeout: 30 * time.Second}
req, _ := http.NewRequest(method, url, body)
resp, _ := client.Do(req)
```

**Go Libraries:**
- `net/http` (stdlib) - HTTP client
- `encoding/json` (stdlib) - JSON handling

**Migration Effort:** 2-3 days

---

### 3. **image** - Image Processing

**Type:** Media Processing
**Complexity:** ⭐⭐⭐⭐ HIGH
**Dependencies:** Image library (Sharp → Go: govips or imaging)

**Purpose:** Image composition, overlay, resizing, effects

**Parameters:**
- `operation` (enum: overlay, merge, composite, blend, default: overlay)
- `images` (array of strings) - Image paths/URLs
- `output` (string) - Output path
- `width` (number, optional) - Output width
- `height` (number, optional) - Output height
- `format` (enum: png, jpeg, webp, default: png)
- `quality` (number 1-100, default: 90)
- `fit` (enum: cover, contain, fill, inside, outside, default: cover)
- `position` (enum: center, top, right, bottom, left, etc., default: center)
- `blendMode` (enum: 21 blend modes, default: over)
- `opacity` (number 0-1, default: 1)
- `blur` (number 0-100, optional)
- `sharpen` (boolean, optional)
- `grayscale` (boolean, optional)

**TypeScript Example:**
```typescript
{
  type: "image",
  parameters: {
    operation: "overlay",
    images: ["base.png", "overlay.png"],
    output: "result.png",
    width: 1920,
    height: 1080,
    format: "png",
    blendMode: "multiply",
    opacity: 0.8
  }
}
```

**Go Implementation:**
```go
pkg/nodes/library/image/
├── image.go          # Main executable
├── operations.go     # Operation types
├── composite.go      # Image composition
├── resize.go         # Resizing operations
└── effects.go        # Blur, sharpen, grayscale

// Using govips (recommended) or imaging
import "github.com/davidbyttow/govips/v2/vips"

img, _ := vips.NewImageFromFile("input.png")
img.Resize(width, height)
img.WriteToFile("output.png")
```

**Go Libraries (Choose one):**
- ✅ **govips** (`github.com/davidbyttow/govips/v2`) - Fast, libvips wrapper (recommended)
- **imaging** (`github.com/disintegration/imaging`) - Pure Go, slower
- **bimg** (`github.com/h2non/bimg`) - libvips wrapper

**Migration Effort:** 1-2 weeks (complex API, many operations)

---

### 4. **transform** - Data Transformation

**Type:** Data Processing
**Complexity:** ⭐⭐⭐⭐⭐ VERY HIGH (JavaScript execution)
**Dependencies:** JS runtime (Function constructor → Go: expr or v8go)

**Purpose:** Transform arrays using map, filter, reduce, sort, etc.

**Parameters:**
- `operation` (enum: map, filter, reduce, sort, group, flatten, unique, reverse, limit, skip, slice)
- `transformFunction` (string, default: "item => item") - JS function
- `sortKey` (string, optional) - Property to sort by
- `sortDirection` (enum: asc, desc, default: asc)
- `reduceInitial` (string, optional) - Initial value for reduce
- `groupKey` (string, optional) - Property to group by
- `flattenDepth` (number 1-10, default: 1)
- `limitCount` (number, optional)
- `skipCount` (number, optional)
- `sliceStart` (number, optional)
- `sliceEnd` (number, optional)
- `data` (array, optional) - Input data for testing

**TypeScript Example:**
```typescript
{
  type: "transform",
  parameters: {
    operation: "map",
    transformFunction: "item => ({ ...item, total: item.price * item.quantity })"
  }
}
```

**Go Implementation (Two Options):**

**Option A: Expression Language (Recommended MVP)**
```go
pkg/nodes/library/transform/
├── transform.go         # Main executable
├── operations.go        # Operation dispatcher
├── expr.go             # Expression evaluation
├── map.go              # Map operation
├── filter.go           # Filter operation
├── reduce.go           # Reduce operation
└── utils.go            # Helpers

// Using expr
import "github.com/antonmedv/expr"

program, _ := expr.Compile("item.price * 1.1")
output, _ := expr.Run(program, env)
```

**Option B: V8 JavaScript (Full Compatibility)**
```go
pkg/nodes/library/transform/
├── transform.go         # Main executable
├── operations.go        # Operation dispatcher
├── v8.go               # V8 runtime
└── utils.go            # Helpers

// Using v8go
import "rogchap.com/v8go"

iso := v8go.NewIsolate()
ctx := v8go.NewContext(iso)
val, _ := ctx.RunScript("item => item * 2", "transform.js")
```

**Go Libraries (Choose one):**
- ✅ **expr** (`github.com/antonmedv/expr`) - Expression language, covers 90% (recommended MVP)
- **v8go** (`rogchap.com/v8go`) - Full JavaScript via V8 (requires CGO)
- **goja** (`github.com/dop251/goja`) - Pure Go JavaScript interpreter (slower)

**Migration Effort:** 1-2 weeks (depends on JS compatibility requirements)

---

### 5. **file-system** - File Operations

**Type:** I/O
**Complexity:** ⭐⭐ LOW
**Dependencies:** File system (fs → Go: `os`, `io/fs`)

**Purpose:** Read, write, list, copy, move, delete files

**Parameters:**
- `operation` (enum: read, write, list, exists, create, delete, copy, move, default: read)
- `path` (string, required) - File/directory path
- `content` (string, optional) - Content to write
- `recursive` (boolean, default: false) - Recursive list

**TypeScript Example:**
```typescript
{
  type: "file-system",
  parameters: {
    operation: "write",
    path: "/tmp/output.txt",
    content: "Hello, world!"
  }
}
```

**Go Implementation:**
```go
pkg/nodes/library/filesystem/
├── filesystem.go     # Main executable
├── read.go          # Read operations
├── write.go         # Write operations
├── list.go          # List operations
└── utils.go         # Path utilities

// Using os package
content, _ := os.ReadFile(path)
os.WriteFile(path, data, 0644)
entries, _ := os.ReadDir(path)
```

**Go Libraries:**
- `os` (stdlib) - File operations
- `io/fs` (stdlib) - File system interface
- `path/filepath` (stdlib) - Path manipulation

**Migration Effort:** 2-3 days

---

### 6. **spreadsheet** - Excel/CSV Reader

**Type:** Data I/O
**Complexity:** ⭐⭐⭐ MEDIUM
**Dependencies:** Spreadsheet library (xlsx → Go: excelize)

**Purpose:** Read Excel and CSV files

**Parameters:**
- `path` (string, optional) - File path
- `data` (string, optional) - Raw content
- `format` (enum: csv, xlsx, xls, xlsb, ods, fods, optional) - Auto-detected
- `sheetName` (string, optional) - Sheet name
- `sheetIndex` (number, optional) - Sheet index (0-based)
- `hasHeaders` (boolean, default: true) - First row is headers
- `range` (string, optional) - Cell range (e.g., "A1:D10")
- `delimiter` (string, default: ",") - CSV delimiter
- `skipEmptyLines` (boolean, default: true)

**TypeScript Example:**
```typescript
{
  type: "spreadsheet",
  parameters: {
    path: "data.xlsx",
    sheetName: "Sheet1",
    hasHeaders: true,
    range: "A1:Z100"
  }
}
```

**Go Implementation:**
```go
pkg/nodes/library/spreadsheet/
├── spreadsheet.go    # Main executable
├── excel.go         # Excel operations (excelize)
├── csv.go           # CSV operations (encoding/csv)
└── format.go        # Format detection

// Using excelize
import "github.com/xuri/excelize/v2"

f, _ := excelize.OpenFile("data.xlsx")
rows, _ := f.GetRows("Sheet1")
```

**Go Libraries:**
- `github.com/xuri/excelize/v2` - Excel operations
- `encoding/csv` (stdlib) - CSV operations

**Migration Effort:** 3-5 days

---

### 7. **shell-command** - Shell Execution

**Type:** System
**Complexity:** ⭐⭐ LOW
**Dependencies:** Shell (child_process → Go: `os/exec`)

**Purpose:** Execute shell commands

**Parameters:**
- `program` (string, required) - Program to execute
- `args` (array of strings | JSON string, default: []) - Arguments
- `stdin` (string, optional) - Stdin data
- `captureOutput` (boolean, default: true) - Capture output

**TypeScript Example:**
```typescript
{
  type: "shell-command",
  parameters: {
    program: "git",
    args: ["status", "--short"],
    captureOutput: true
  }
}
```

**Go Implementation:**
```go
pkg/nodes/library/shellcommand/
├── shellcommand.go   # Main executable
├── executor.go      # Command execution
└── output.go        # Output handling

// Using os/exec
cmd := exec.Command(program, args...)
cmd.Stdin = strings.NewReader(stdin)
output, _ := cmd.CombinedOutput()
```

**Go Libraries:**
- `os/exec` (stdlib) - Command execution

**Migration Effort:** 1-2 days

---

### 8. **loop** - Loop Execution

**Type:** Control Flow
**Complexity:** ⭐⭐⭐ MEDIUM
**Dependencies:** None (pure logic)

**Purpose:** Iterate and execute child nodes

**Parameters:**
- `loopType` (enum: forEach, times, while, default: forEach)
- `array` (array, optional) - Array for forEach
- `count` (number 1-10000, default: 10) - Iteration count for times
- `condition` (string, optional) - JS condition for while
- `collectResults` (boolean, default: true) - Collect iteration results

**TypeScript Example:**
```typescript
{
  type: "loop",
  parameters: {
    loopType: "forEach",
    array: [1, 2, 3, 4, 5],
    collectResults: true
  },
  nodes: [ /* child nodes to execute per iteration */ ]
}
```

**Go Implementation:**
```go
pkg/nodes/library/loop/
├── loop.go          # Main executable
├── foreach.go       # ForEach implementation
├── times.go         # Times implementation
├── while.go         # While implementation (needs expr)
└── collector.go     # Result collection

// forEach implementation
for i, item := range array {
    ctx := createIterationContext(item, i)
    result, _ := conductor.Run(ctx, childNode)
    results = append(results, result)
}
```

**Go Libraries:**
- None (pure Go logic)
- Optional: `expr` for while condition evaluation

**Migration Effort:** 3-4 days

---

### 9. **group** - Sequential/Parallel Group

**Type:** Control Flow
**Complexity:** ⭐⭐⭐ MEDIUM
**Dependencies:** None (pure logic)

**Purpose:** Group and execute child nodes sequentially or in parallel

**Parameters:**
- `timeout` (number 1000-300000ms, default: 30000) - Max execution time
- `retries` (number 0-10, default: 1) - Retry attempts on failure
- `parallel` (boolean, default: false) - Execute in parallel

**TypeScript Example:**
```typescript
{
  type: "group",
  parameters: {
    timeout: 60000,
    retries: 2,
    parallel: false
  },
  nodes: [ /* child nodes */ ],
  edges: [ /* connections between nodes */ ]
}
```

**Go Implementation:**
```go
pkg/nodes/library/group/
├── group.go         # Main executable
├── sequential.go    # Sequential execution
├── parallel.go      # Parallel execution (goroutines)
├── retry.go         # Retry logic
└── timeout.go       # Timeout handling

// Parallel execution with goroutines
var wg sync.WaitGroup
results := make(chan *ExecutionResult, len(nodes))

for _, node := range nodes {
    wg.Add(1)
    go func(n *NodeDefinition) {
        defer wg.Done()
        result, _ := conductor.Run(ctx, n)
        results <- result
    }(node)
}

wg.Wait()
close(results)
```

**Go Libraries:**
- None (use goroutines and channels)
- `context` (stdlib) - Timeout/cancellation

**Migration Effort:** 3-4 days

---

### 10. **parallel** - Advanced Parallel Execution

**Type:** Control Flow
**Complexity:** ⭐⭐⭐⭐ HIGH
**Dependencies:** None (concurrency primitives)

**Purpose:** Execute child nodes with advanced parallelism control

**Parameters:**
- `concurrency` (number 1-50, default: 5) - Max concurrent operations
- `strategy` (enum: all, race, allSettled, default: allSettled)
- `operationTimeout` (number 1000-300000ms, default: 30000) - Per-operation timeout
- `globalTimeout` (number 5000-600000ms, default: 120000) - Global timeout
- `failFast` (boolean, default: false) - Stop on first error
- `maintainOrder` (boolean, default: true) - Preserve input order

**TypeScript Example:**
```typescript
{
  type: "parallel",
  parameters: {
    concurrency: 10,
    strategy: "allSettled",
    operationTimeout: 30000,
    globalTimeout: 120000,
    failFast: false,
    maintainOrder: true
  },
  nodes: [ /* child nodes to execute in parallel */ ]
}
```

**Go Implementation:**
```go
pkg/nodes/library/parallel/
├── parallel.go       # Main executable
├── pool.go          # Worker pool
├── strategies.go    # Execution strategies
├── timeout.go       # Timeout management
└── collector.go     # Result collection

// Worker pool with semaphore
sem := make(chan struct{}, concurrency)
results := make([]*ExecutionResult, len(nodes))

for i, node := range nodes {
    sem <- struct{}{}
    go func(idx int, n *NodeDefinition) {
        defer func() { <-sem }()
        ctx, cancel := context.WithTimeout(context.Background(), opTimeout)
        defer cancel()
        results[idx], _ = conductor.Run(ctx, n)
    }(i, node)
}
```

**Go Libraries:**
- `context` (stdlib) - Timeouts
- `sync` (stdlib) - WaitGroup, Mutex
- Goroutines and channels for concurrency

**Migration Effort:** 4-5 days

---

## Node Complexity Summary

| Node | Complexity | Effort | Dependencies |
|------|------------|--------|--------------|
| edit-fields | ⭐⭐ Low | 2-3 days | text/template |
| http-request | ⭐⭐ Low | 2-3 days | net/http |
| file-system | ⭐⭐ Low | 2-3 days | os, io/fs |
| shell-command | ⭐⭐ Low | 1-2 days | os/exec |
| group | ⭐⭐⭐ Medium | 3-4 days | goroutines |
| loop | ⭐⭐⭐ Medium | 3-4 days | goroutines |
| spreadsheet | ⭐⭐⭐ Medium | 3-5 days | excelize |
| parallel | ⭐⭐⭐⭐ High | 4-5 days | concurrency primitives |
| image | ⭐⭐⭐⭐ High | 1-2 weeks | govips |
| transform | ⭐⭐⭐⭐⭐ Very High | 1-2 weeks | expr or v8go |

**Total Migration Effort:** 5-7 weeks for all 10 nodes

---

## Node Dependencies Graph

```
Pure Logic (No external deps):
├── group           # Uses goroutines
├── loop            # Uses goroutines (while needs expr)
└── parallel        # Uses goroutines, channels

Stdlib Only:
├── edit-fields     # text/template
├── http-request    # net/http
├── file-system     # os, io/fs
└── shell-command   # os/exec

External Libraries:
├── spreadsheet     # excelize
├── image           # govips (or imaging)
└── transform       # expr (or v8go)
```

---

## Testing Strategy for Each Node

### Unit Tests (Required for ALL nodes)

```go
func TestEditFieldsExecute(t *testing.T) {
    node := &editfields.Node{}
    params := map[string]interface{}{
        "values": map[string]interface{}{
            "name": "{{firstName}} {{lastName}}",
        },
    }
    input := map[string]interface{}{
        "firstName": "Alice",
        "lastName": "Smith",
    }

    result, err := node.Execute(context.Background(), params, input)
    assert.NoError(t, err)
    assert.Equal(t, "Alice Smith", result["name"])
}
```

### Integration Tests (Required for ALL nodes)

```go
func TestHTTPRequestIntegration(t *testing.T) {
    // Spin up test HTTP server
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(200)
        json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
    }))
    defer ts.Close()

    // Execute HTTP request node
    node := &http.Node{}
    result, err := node.Execute(ctx, map[string]interface{}{
        "method": "GET",
        "url": ts.URL,
    })

    assert.NoError(t, err)
    assert.Equal(t, 200, result["status"])
}
```

---

## Migration Checklist

### Per Node Requirements

- [ ] **Core Executable**
  - [ ] Implement Executable interface
  - [ ] Parameter validation
  - [ ] Error handling
  - [ ] Logging integration

- [ ] **Operations**
  - [ ] All operations implemented
  - [ ] Edge cases handled
  - [ ] Performance optimized

- [ ] **Testing**
  - [ ] Unit tests (>80% coverage)
  - [ ] Integration tests
  - [ ] Error scenario tests
  - [ ] Performance benchmarks

- [ ] **Documentation**
  - [ ] Parameter documentation
  - [ ] Usage examples
  - [ ] Error codes documented

- [ ] **Registry**
  - [ ] Node registered in global registry
  - [ ] Type mapping correct
  - [ ] Factory function created

---

## Go Package Structure for Nodes

```
pkg/nodes/
├── definition.go         # NodeDefinition struct
├── executable.go         # Executable interface
├── registry.go           # Global node registry
├── factory.go            # Node creation
│
└── library/
    ├── editfields/
    │   ├── editfields.go
    │   ├── editfields_test.go
    │   ├── template.go
    │   └── variables.go
    │
    ├── http/
    │   ├── http.go
    │   ├── http_test.go
    │   ├── client.go
    │   ├── auth.go
    │   └── response.go
    │
    ├── image/
    │   ├── image.go
    │   ├── image_test.go
    │   ├── operations.go
    │   ├── composite.go
    │   ├── resize.go
    │   └── effects.go
    │
    ├── transform/
    │   ├── transform.go
    │   ├── transform_test.go
    │   ├── operations.go
    │   ├── expr.go           # Or v8.go
    │   ├── map.go
    │   ├── filter.go
    │   └── reduce.go
    │
    ├── filesystem/
    │   ├── filesystem.go
    │   ├── filesystem_test.go
    │   ├── read.go
    │   ├── write.go
    │   └── list.go
    │
    ├── spreadsheet/
    │   ├── spreadsheet.go
    │   ├── spreadsheet_test.go
    │   ├── excel.go
    │   ├── csv.go
    │   └── format.go
    │
    ├── shellcommand/
    │   ├── shellcommand.go
    │   ├── shellcommand_test.go
    │   ├── executor.go
    │   └── output.go
    │
    ├── loop/
    │   ├── loop.go
    │   ├── loop_test.go
    │   ├── foreach.go
    │   ├── times.go
    │   ├── while.go
    │   └── collector.go
    │
    ├── group/
    │   ├── group.go
    │   ├── group_test.go
    │   ├── sequential.go
    │   ├── parallel.go
    │   ├── retry.go
    │   └── timeout.go
    │
    └── parallel/
        ├── parallel.go
        ├── parallel_test.go
        ├── pool.go
        ├── strategies.go
        ├── timeout.go
        └── collector.go
```

---

## Success Criteria

### Node Library Complete When:

- ✅ All 10 nodes implemented
- ✅ All unit tests passing (>80% coverage per node)
- ✅ All integration tests passing
- ✅ Performance benchmarks meet or exceed TypeScript version
- ✅ All nodes registered in global registry
- ✅ Documentation complete for all nodes

### Validation:

```go
// Verify all nodes registered
func TestAllNodesRegistered(t *testing.T) {
    expectedNodes := []string{
        "edit-fields",
        "http-request",
        "image",
        "transform",
        "file-system",
        "spreadsheet",
        "shell-command",
        "loop",
        "group",
        "parallel",
    }

    for _, nodeType := range expectedNodes {
        assert.True(t, registry.Has(nodeType),
            "Node %s should be registered", nodeType)
    }
}
```

---

## Next Steps

1. **Finalize library choices** - User to specify preferred Go libraries
2. **Start with simple nodes** - edit-fields, http-request, file-system
3. **Implement complex nodes** - image, transform
4. **Integration testing** - Test all nodes together
5. **Performance benchmarking** - Compare with TypeScript version

---

**Status:** Complete inventory ready for migration
**Last Updated:** 2025-10-18
