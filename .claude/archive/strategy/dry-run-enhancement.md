# Dry-Run Enhancement Strategy

**Document Version:** 1.0
**Created:** 2025-10-20
**Purpose:** Define TDD-driven strategy for enhancing bento's dry-run capabilities to provide comprehensive execution preview and side effect analysis

---

## Executive Summary

### Problem Statement

The current `--dry-run` flag in bento provides minimal insight into what a workflow will actually do. It only shows:
- Bento name
- Total node count
- Node list with IDs/types (if --verbose)
- Basic validation status

**Critical Gaps:**
- No execution simulation (can't see what files will be created/modified/deleted)
- No loop iteration preview (can't see CSV rows that will be processed)
- No template resolution preview (can't see actual paths with variables substituted)
- No side effect analysis (file operations, HTTP requests, shell commands)
- No resource conflict detection (multiple nodes writing to same file)

### Business Impact

For workflows like product automation (CSV ingestion → folder creation → overlay copying → rendering):
- Users cannot validate workflow correctness without executing
- Risk of unintended file system modifications
- Difficult to debug template resolution issues
- No way to estimate resource usage (disk space, API calls)

### Goal

Implement production-quality dry-run capabilities that provide comprehensive execution preview, enabling users to understand exactly what a bento will do before running it.

---

## Current State Analysis

### Implementation Location

**File:** `cmd/bento/run.go` (lines 238-254)

```go
func showDryRun(def *neta.Definition) error {
    printInfo("DRY RUN MODE - No execution will occur")
    fmt.Printf("\nWould execute bento: %s\n", def.Name)
    fmt.Printf("Total nodes to execute: %d\n\n", len(def.Nodes))

    if verboseFlag {
        printInfo("Nodes that would be executed:")
        for i, node := range def.Nodes {
            fmt.Printf("  %d. [%s] %s (type: %s)\n", i+1, node.ID, node.Name, node.Type)
        }
    }

    fmt.Println("\nValidation: ✓ Passed")
    printSuccess("Dry run complete. Use 'bento run' without --dry-run to execute.")
    return nil
}
```

### Architecture Overview

**Bento Execution Flow:**
1. Load bento definition (`neta.Definition`)
2. Create Itamae orchestrator
3. Execute each node via `neta.Executable` interface
4. Pass context between nodes via `map[string]interface{}`

**Key Interfaces:**

```go
// pkg/neta/executable.go
type Executable interface {
    Execute(ctx context.Context, params map[string]interface{}) (interface{}, error)
}
```

### Neta Types Requiring Dry-Run Support

1. **file-system** - Create/delete/copy/move files and directories
2. **http-request** - Make API calls
3. **shell-command** - Execute shell commands
4. **loop** - Iterate over collections (forEach, forEachAsync)
5. **spreadsheet** - Read CSV/Excel files
6. **image** - Process images (resize, convert, optimize)
7. **parallel** - Execute multiple neta concurrently
8. **group** - Sequential execution of child neta
9. **transform** - Data transformation
10. **edit-fields** - Modify context fields

### Gap Analysis

| Capability | Current State | Required State |
|-----------|---------------|----------------|
| Node listing | ✓ Basic | ✓ Enhanced with parameters |
| Template resolution | ✗ None | ✓ Show resolved paths |
| Loop iteration preview | ✗ None | ✓ Show first N items |
| File operation preview | ✗ None | ✓ Show all file operations |
| HTTP request preview | ✗ None | ✓ Show URLs and methods |
| Shell command preview | ✗ None | ✓ Show commands (sanitized) |
| Resource conflict detection | ✗ None | ✓ Detect conflicts |
| Execution flow visualization | ✗ None | ✓ Show execution order |

---

## Proposed Architecture

### Core Concept: DryRun Interface

Add a new optional interface that neta can implement:

```go
// pkg/neta/dryrun.go

// DryRunnable is an optional interface that neta can implement
// to provide detailed dry-run information.
type DryRunnable interface {
    // DryRun simulates execution and returns a preview of what would happen.
    // It should NOT modify any state or make external calls.
    //
    // Parameters:
    //   - ctx: Context for cancellation
    //   - params: Input parameters (same as Execute)
    //
    // Returns:
    //   - DryRunResult: Detailed preview of execution
    //   - error: If preview cannot be generated
    DryRun(ctx context.Context, params map[string]interface{}) (*DryRunResult, error)
}

// DryRunResult contains detailed information about what would be executed.
type DryRunResult struct {
    // NodeID is the ID of the node being previewed
    NodeID string

    // NodeName is the display name of the node
    NodeName string

    // NodeType is the type of neta (file-system, http-request, etc.)
    NodeType string

    // Operations lists all operations that would be performed
    Operations []Operation

    // OutputPreview shows a preview of what would be returned
    // (useful for loops to show first N items)
    OutputPreview interface{}

    // Warnings lists potential issues detected during dry-run
    Warnings []string

    // EstimatedDuration is an optional estimate of execution time
    EstimatedDuration *time.Duration
}

// Operation represents a single operation that would be performed.
type Operation struct {
    // Type is the kind of operation (create_file, http_get, shell_exec, etc.)
    Type OperationType

    // Description is a human-readable description
    Description string

    // Target is the resource being operated on (file path, URL, etc.)
    Target string

    // Details contains operation-specific information
    Details map[string]interface{}
}

// OperationType represents the kind of operation.
type OperationType string

const (
    OpCreateFile      OperationType = "create_file"
    OpDeleteFile      OperationType = "delete_file"
    OpModifyFile      OperationType = "modify_file"
    OpCopyFile        OperationType = "copy_file"
    OpMoveFile        OperationType = "move_file"
    OpCreateDir       OperationType = "create_dir"
    OpDeleteDir       OperationType = "delete_dir"
    OpHTTPGet         OperationType = "http_get"
    OpHTTPPost        OperationType = "http_post"
    OpHTTPPut         OperationType = "http_put"
    OpHTTPDelete      OperationType = "http_delete"
    OpShellExec       OperationType = "shell_exec"
    OpImageProcess    OperationType = "image_process"
    OpDataTransform   OperationType = "data_transform"
)
```

### Enhanced showDryRun Function

Replace the current simple implementation with a comprehensive simulator:

```go
// cmd/bento/run.go

func showDryRun(def *neta.Definition, itamae *itamae.Itamae) error {
    printInfo("DRY RUN MODE - No execution will occur")
    fmt.Printf("\nWould execute bento: %s\n", def.Name)
    fmt.Printf("Total nodes: %d\n\n", len(def.Nodes))

    // Create dry-run simulator
    simulator := NewDryRunSimulator(itamae)

    // Simulate execution
    results, err := simulator.Simulate(context.Background(), def)
    if err != nil {
        return fmt.Errorf("dry-run simulation failed: %w", err)
    }

    // Display results
    displayDryRunResults(results)

    return nil
}
```

### DryRunSimulator

New component to orchestrate dry-run simulation:

```go
// cmd/bento/dryrun.go

type DryRunSimulator struct {
    itamae *itamae.Itamae
    results []neta.DryRunResult
    operations []neta.Operation
}

func NewDryRunSimulator(itamae *itamae.Itamae) *DryRunSimulator {
    return &DryRunSimulator{
        itamae: itamae,
    }
}

// Simulate walks through the bento definition and simulates each node.
func (s *DryRunSimulator) Simulate(ctx context.Context, def *neta.Definition) ([]neta.DryRunResult, error) {
    // Implementation will recursively simulate each node
    // following the same execution order as real execution
}

// displayDryRunResults formats and prints the simulation results.
func displayDryRunResults(results []neta.DryRunResult) {
    // Format and display results in a readable format
}
```

---

## TDD Implementation Plan

### Phase 1: Core Infrastructure (RED → GREEN → REFACTOR)

**Goal:** Establish DryRunnable interface and basic file-system neta implementation

#### RED: Write Failing Tests First

**Test File:** `pkg/neta/dryrun_test.go`

```go
func TestDryRunnable_Interface(t *testing.T) {
    // Verify interface exists and can be implemented
}

func TestFileSystemNeta_DryRun_CreateDirectory(t *testing.T) {
    // Given: file-system neta configured to create directory
    params := map[string]interface{}{
        "operation": "create_directory",
        "path":      "/tmp/test-dir",
    }

    neta := createFileSystemNeta(t)

    // When: DryRun is called
    result, err := neta.DryRun(context.Background(), params)

    // Then: Should return operation preview WITHOUT creating directory
    require.NoError(t, err)
    assert.Equal(t, 1, len(result.Operations))
    assert.Equal(t, neta.OpCreateDir, result.Operations[0].Type)
    assert.Equal(t, "/tmp/test-dir", result.Operations[0].Target)

    // Verify directory was NOT actually created
    _, err = os.Stat("/tmp/test-dir")
    assert.True(t, os.IsNotExist(err), "Directory should NOT be created during dry-run")
}

func TestFileSystemNeta_DryRun_CopyFile(t *testing.T) {
    // Given: file-system neta configured to copy file
    params := map[string]interface{}{
        "operation": "copy",
        "source":    "/tmp/source.txt",
        "dest":      "/tmp/dest.txt",
    }

    neta := createFileSystemNeta(t)

    // When: DryRun is called
    result, err := neta.DryRun(context.Background(), params)

    // Then: Should return operation preview
    require.NoError(t, err)
    assert.Equal(t, 1, len(result.Operations))
    assert.Equal(t, neta.OpCopyFile, result.Operations[0].Type)
    assert.Equal(t, "/tmp/source.txt", result.Operations[0].Details["source"])
    assert.Equal(t, "/tmp/dest.txt", result.Operations[0].Target)
}
```

**Test File:** `cmd/bento/dryrun_test.go`

```go
func TestDryRunSimulator_FileSystemOperations(t *testing.T) {
    // Given: bento with file-system operations
    bento := createTestBento(t, `{
        "id": "test",
        "type": "bento",
        "name": "Test Bento",
        "nodes": [{
            "id": "create-dir",
            "type": "file-system",
            "name": "Create Directory",
            "parameters": {
                "operation": "create_directory",
                "path": "products/test"
            }
        }]
    }`)

    simulator := NewDryRunSimulator(createItamae(t))

    // When: Simulate is called
    results, err := simulator.Simulate(context.Background(), bento)

    // Then: Should return preview of operations
    require.NoError(t, err)
    assert.Equal(t, 1, len(results))
    assert.Equal(t, "create-dir", results[0].NodeID)
    assert.Equal(t, 1, len(results[0].Operations))
    assert.Equal(t, neta.OpCreateDir, results[0].Operations[0].Type)
}
```

#### GREEN: Implement Minimal Code to Pass Tests

1. Create `pkg/neta/dryrun.go` with interfaces
2. Implement `DryRun()` method on file-system neta
3. Create `cmd/bento/dryrun.go` with simulator
4. Update `cmd/bento/run.go` to use simulator

#### REFACTOR: Clean Up Implementation

1. Extract common dry-run logic
2. Add helper functions for operation creation
3. Improve error messages
4. Add inline documentation

---

### Phase 2: Template Resolution Preview (RED → GREEN → REFACTOR)

**Goal:** Show resolved paths/values after template substitution

#### RED: Write Failing Tests

```go
func TestDryRunSimulator_TemplateResolution(t *testing.T) {
    // Given: bento with templated paths
    bento := createTestBento(t, `{
        "id": "test",
        "type": "bento",
        "name": "Test Bento",
        "nodes": [{
            "id": "create-dir",
            "type": "file-system",
            "name": "Create Product Directory",
            "parameters": {
                "operation": "create_directory",
                "path": "products/{{.item.name}}"
            }
        }]
    }`)

    // Context with template variables
    params := map[string]interface{}{
        "item": map[string]interface{}{
            "name": "Combat Dog (Supplies)",
        },
    }

    simulator := NewDryRunSimulator(createItamae(t))

    // When: Simulate is called
    results, err := simulator.Simulate(context.Background(), bento, params)

    // Then: Should show resolved path
    require.NoError(t, err)
    assert.Equal(t, 1, len(results))
    assert.Equal(t, "products/Combat Dog (Supplies)", results[0].Operations[0].Target)
}
```

#### GREEN: Implement Template Resolution in DryRun

1. Add template engine integration to dry-run
2. Resolve templates before creating operations
3. Show both original and resolved values

#### REFACTOR: Optimize Template Engine Usage

---

### Phase 3: Loop Iteration Preview (RED → GREEN → REFACTOR)

**Goal:** Show preview of loop iterations without full execution

#### RED: Write Failing Tests

```go
func TestLoopNeta_DryRun_ForEach(t *testing.T) {
    // Given: loop neta with CSV data source
    params := map[string]interface{}{
        "mode": "forEach",
        "over": "{{.csvData.data}}",
        "childNodes": []interface{}{
            map[string]interface{}{
                "id":   "child",
                "type": "file-system",
                "parameters": map[string]interface{}{
                    "operation": "create_directory",
                    "path":      "products/{{.item.name}}",
                },
            },
        },
        "csvData": map[string]interface{}{
            "data": []interface{}{
                map[string]interface{}{"name": "Combat Dog (Supplies)"},
                map[string]interface{}{"name": "Combat Dog (Gas Mask)"},
                map[string]interface{}{"name": "Combat Dog (Attack)"},
            },
        },
    }

    neta := createLoopNeta(t)

    // When: DryRun is called
    result, err := neta.DryRun(context.Background(), params)

    // Then: Should preview loop iterations
    require.NoError(t, err)
    assert.Contains(t, result.Description, "Would iterate 3 times")

    // Should show preview of first 2 iterations
    assert.Equal(t, 2, len(result.Operations))
    assert.Contains(t, result.Operations[0].Description, "Iteration 1/3")
    assert.Contains(t, result.Operations[1].Description, "Iteration 2/3")

    // Output preview should include sample data
    preview := result.OutputPreview.(map[string]interface{})
    assert.Equal(t, 3, preview["totalIterations"])
    assert.Equal(t, 2, preview["previewCount"])
}
```

#### GREEN: Implement Loop Preview

1. Detect loop neta in dry-run
2. Show total iteration count
3. Preview first N iterations (configurable, default 2)
4. Recursively dry-run child nodes for previewed iterations

#### REFACTOR: Add Configuration for Preview Count

---

### Phase 4: Integration Tests (RED → GREEN → REFACTOR)

**Goal:** End-to-end testing with real bento files

#### RED: Write Failing Tests

**Test File:** `tests/integration/dryrun_test.go`

```go
func TestDryRun_ProductAutomation(t *testing.T) {
    // Given: product automation bento
    bentoPath := "examples/product-automation.bento.json"
    envVars := map[string]string{
        "INPUT_CSV": "tests/fixtures/products-test.csv",
    }

    // When: Run with --dry-run flag
    output, err := RunBentoDryRun(t, bentoPath, envVars)

    // Then: Should show comprehensive preview
    require.NoError(t, err)

    // Verify dry-run mode message
    assert.Contains(t, output, "DRY RUN MODE - No execution will occur")

    // Verify loop iteration preview
    assert.Contains(t, output, "Would iterate 3 times")

    // Verify file operations preview
    assert.Contains(t, output, "Would create directory: products/Combat Dog (Supplies)")
    assert.Contains(t, output, "Would create directory: products/Combat Dog (Gas Mask)")

    // Verify HTTP operations preview
    assert.Contains(t, output, "Would download: https://example.com/overlay.png")

    // Verify shell command preview
    assert.Contains(t, output, "Would execute: blender")

    // Verify resource summary
    assert.Contains(t, output, "Summary:")
    assert.Contains(t, output, "Directories to create: 3")
    assert.Contains(t, output, "Files to download: 3")
    assert.Contains(t, output, "Images to process: 24")

    // Verify NO actual execution occurred
    assert.False(t, fileExists("products/Combat Dog (Supplies)"),
        "Directories should NOT be created during dry-run")
}

func TestDryRun_FolderSetup(t *testing.T) {
    // Given: folder setup bento
    bentoPath := "examples/folder-setup.bento.json"

    // When: Run with --dry-run flag
    output, err := RunBentoDryRun(t, bentoPath, nil)

    // Then: Should show folder creation preview
    require.NoError(t, err)
    assert.Contains(t, output, "Would create directory: products/Combat Dog (Supplies)")
    assert.Contains(t, output, "Would create directory: products/Combat Dog (Gas Mask)")
    assert.Contains(t, output, "Would create directory: products/Combat Dog (Attack)")

    // Verify template resolution shown
    assert.Contains(t, output, "Template: products/{{.item.name}}")
    assert.Contains(t, output, "Resolved: products/Combat Dog (Supplies)")
}

// Helper function to run bento with dry-run flag
func RunBentoDryRun(t *testing.T, bentoPath string, envVars map[string]string) (string, error) {
    t.Helper()
    cmd := exec.Command("bento", "run", bentoPath, "--dry-run", "--verbose")

    // Set environment variables
    if envVars != nil {
        for k, v := range envVars {
            cmd.Env = append(cmd.Env, fmt.Sprintf("%s=%s", k, v))
        }
    }

    output, err := cmd.CombinedOutput()
    return string(output), err
}
```

#### GREEN: Implement End-to-End Dry-Run

1. Wire up DryRunSimulator to run command
2. Implement display formatting
3. Add verbose output option
4. Ensure no side effects occur

#### REFACTOR: Improve Output Formatting

---

### Phase 5: Advanced Features (RED → GREEN → REFACTOR)

**Goal:** Resource conflict detection and optimization suggestions

#### RED: Write Failing Tests

```go
func TestDryRunSimulator_DetectConflicts(t *testing.T) {
    // Given: bento with conflicting file operations
    bento := createTestBento(t, `{
        "id": "test",
        "type": "bento",
        "name": "Conflicting Operations",
        "nodes": [
            {
                "id": "write1",
                "type": "file-system",
                "parameters": {
                    "operation": "write_file",
                    "path": "output/result.txt",
                    "content": "Version 1"
                }
            },
            {
                "id": "write2",
                "type": "file-system",
                "parameters": {
                    "operation": "write_file",
                    "path": "output/result.txt",
                    "content": "Version 2"
                }
            }
        ]
    }`)

    simulator := NewDryRunSimulator(createItamae(t))

    // When: Simulate is called
    results, err := simulator.Simulate(context.Background(), bento)

    // Then: Should detect conflict
    require.NoError(t, err)

    // Should have warnings
    var warnings []string
    for _, result := range results {
        warnings = append(warnings, result.Warnings...)
    }

    assert.Contains(t, warnings, "Multiple nodes write to: output/result.txt")
}
```

#### GREEN: Implement Conflict Detection

1. Track all file operations across nodes
2. Detect multiple writes to same file
3. Detect read-after-delete scenarios
4. Add warnings to results

#### REFACTOR: Add Configurable Conflict Rules

---

## Test Specifications

### Unit Test Coverage Requirements

| Component | Minimum Coverage | Critical Paths |
|-----------|-----------------|----------------|
| DryRunnable interface | 100% | All implementations |
| file-system neta DryRun | 100% | All operations |
| loop neta DryRun | 100% | forEach, forEachAsync |
| DryRunSimulator | 95% | Simulation logic |
| Template resolution | 100% | Variable substitution |

### Integration Test Requirements

| Test Case | Description | Success Criteria |
|-----------|-------------|------------------|
| Product Automation | Full workflow preview | Shows all 41 nodes, loop iterations, file ops |
| Folder Setup | Simple loop with folders | Shows 3 directories to create |
| CSV Reader | Data source preview | Shows CSV columns and row count |
| Template Resolution | Variable substitution | Shows resolved paths |
| Conflict Detection | Multiple writes to same file | Shows warnings |

### Manual Test Scenarios

1. **Large CSV Files**: Test with 100+ row CSV, verify preview limit works
2. **Complex Templates**: Test nested templates and edge cases
3. **Parallel Operations**: Test parallel neta dry-run
4. **Error Scenarios**: Test dry-run with invalid templates

---

## Success Criteria

### Functional Requirements

- [ ] All neta types implement DryRunnable interface
- [ ] Dry-run shows comprehensive operation preview
- [ ] Loop iterations are previewed (configurable limit)
- [ ] Template resolution is shown in output
- [ ] No side effects occur during dry-run
- [ ] Resource conflicts are detected and warned

### Performance Requirements

- [ ] Dry-run completes in <5% of normal execution time
- [ ] Memory usage is reasonable for large bentos (<100MB)
- [ ] Output is readable and not overwhelming

### User Experience Requirements

- [ ] Output is clear and actionable
- [ ] Warnings are visible and helpful
- [ ] Verbose mode provides additional detail
- [ ] Summary shows totals (files created, API calls, etc.)

### Code Quality Requirements

- [ ] All new code follows Go Proverbs
- [ ] Test coverage meets minimums
- [ ] Documentation is complete
- [ ] No breaking changes to existing API

---

## Implementation Phases Timeline

| Phase | Duration | Deliverables |
|-------|----------|--------------|
| Phase 1: Core Infrastructure | 2-3 days | DryRunnable interface, file-system impl |
| Phase 2: Template Resolution | 1-2 days | Template preview in dry-run |
| Phase 3: Loop Preview | 2-3 days | Loop iteration preview |
| Phase 4: Integration Tests | 1-2 days | End-to-end tests |
| Phase 5: Advanced Features | 2-3 days | Conflict detection |
| **Total** | **8-13 days** | Production-ready dry-run |

---

## Colossus Implementation Prompt

```
CONTEXT: Bento is a workflow automation CLI tool written in Go. Users execute workflows called "bentos" which are JSON files defining a series of operations (called "neta"). The current --dry-run flag is very basic and only shows a list of nodes without any execution preview or side effect analysis.

PROBLEM: Users need comprehensive dry-run capabilities to preview what a bento will do before executing it, especially for workflows that create files, make API calls, or process large datasets.

TASK: Implement production-quality dry-run capabilities for bento following Test-Driven Development (TDD) methodology.

REQUIREMENTS:

1. INTERFACE DESIGN (pkg/neta/dryrun.go)
   - Create DryRunnable interface with DryRun() method
   - Define DryRunResult struct to hold operation previews
   - Define Operation struct for individual operations
   - Define OperationType constants (OpCreateFile, OpHTTPGet, etc.)
   - Follow Go interface design best practices (small, focused)

2. FILE-SYSTEM NETA IMPLEMENTATION (pkg/neta/library/filesystem/filesystem.go)
   - Implement DryRun() method for file-system neta
   - Preview all operations: create_directory, copy, move, delete, write_file
   - Resolve templates in paths before showing preview
   - Return DryRunResult with operation list
   - DO NOT perform actual file operations during dry-run
   - Add comprehensive unit tests FIRST (TDD: RED → GREEN → REFACTOR)

3. LOOP NETA IMPLEMENTATION (pkg/neta/library/loop/loop.go)
   - Implement DryRun() method for loop neta
   - Show total iteration count
   - Preview first 2 iterations (configurable via maxPreviewIterations)
   - Recursively call DryRun() on child nodes for previewed iterations
   - Add unit tests FIRST

4. DRY-RUN SIMULATOR (cmd/bento/dryrun.go)
   - Create DryRunSimulator struct
   - Implement Simulate() method to walk bento definition
   - Collect results from all nodes
   - Detect resource conflicts (multiple writes to same file)
   - Format and display results in readable format
   - Add unit tests FIRST

5. INTEGRATION WITH RUN COMMAND (cmd/bento/run.go)
   - Update showDryRun() to use DryRunSimulator
   - Pass Itamae instance to simulator
   - Display results with clear formatting
   - Show summary statistics (files to create, API calls, etc.)
   - Ensure --verbose flag provides additional detail

6. INTEGRATION TESTS (tests/integration/dryrun_test.go)
   - Create integration tests for product-automation.bento.json
   - Create integration tests for folder-setup.bento.json
   - Verify no actual execution occurs (no files created)
   - Verify output contains expected preview information
   - Use existing test helpers (RunBento, CleanupTestDir, etc.)

7. ADDITIONAL NETA IMPLEMENTATIONS
   - http-request: Preview URLs, methods, headers
   - shell-command: Preview commands (sanitize sensitive data)
   - spreadsheet: Preview row count, column names
   - image: Preview operations (resize, convert, etc.)
   - parallel: Preview parallel execution plan
   - group: Preview sequential execution

CONSTRAINTS:

1. FOLLOW GO PROVERBS
   - Don't just check errors, handle them gracefully
   - Make the zero value useful
   - Interface pollution is real - keep interfaces small
   - The bigger the interface, the weaker the abstraction
   - Accept interfaces, return structs
   - Clear is better than clever

2. TDD METHODOLOGY
   - Write tests FIRST (RED)
   - Implement minimal code to pass (GREEN)
   - Refactor for clarity and maintainability (REFACTOR)
   - Commit after each GREEN phase
   - All tests must pass before moving to next phase

3. NO SIDE EFFECTS
   - Dry-run MUST NOT create/modify/delete any files
   - Dry-run MUST NOT make HTTP requests
   - Dry-run MUST NOT execute shell commands
   - Dry-run MUST NOT modify any external state

4. BACKWARDS COMPATIBILITY
   - Do not break existing neta implementations
   - DryRunnable is optional interface
   - If neta doesn't implement DryRunnable, show basic preview

5. CODE ORGANIZATION
   - Keep packages focused and cohesive
   - Follow existing bento code structure
   - Use existing utilities (template engine, context passing)
   - Add clear godoc comments for all public interfaces

IMPLEMENTATION PHASES:

Phase 1: Core Infrastructure (DO THIS FIRST)
- Write tests for DryRunnable interface
- Create pkg/neta/dryrun.go with interfaces
- Implement DryRun() for file-system neta
- Write cmd/bento/dryrun.go with simulator
- Update cmd/bento/run.go to use simulator
- Verify all tests pass

Phase 2: Template Resolution
- Write tests for template resolution in dry-run
- Implement template resolution in file-system DryRun()
- Show both original and resolved values in output
- Verify tests pass

Phase 3: Loop Preview
- Write tests for loop neta DryRun()
- Implement iteration preview (show first N)
- Recursively dry-run child nodes
- Verify tests pass

Phase 4: Integration Tests
- Write end-to-end tests with real bento files
- Test product-automation.bento.json dry-run
- Test folder-setup.bento.json dry-run
- Verify no actual execution occurs
- Verify tests pass

Phase 5: Additional Neta
- Implement DryRun() for http-request neta
- Implement DryRun() for shell-command neta
- Implement DryRun() for other neta types
- Write tests for each implementation

Phase 6: Conflict Detection
- Write tests for resource conflict detection
- Implement conflict detection in DryRunSimulator
- Add warnings to DryRunResult
- Verify tests pass

SUCCESS CRITERIA:
- All tests pass (unit + integration)
- No side effects during dry-run
- Output is clear and actionable
- Template resolution works correctly
- Loop preview shows first N iterations
- Resource conflicts are detected
- Code follows Go best practices
- Documentation is complete

REFERENCE FILES:
- cmd/bento/run.go (current dry-run implementation, lines 238-254)
- pkg/neta/executable.go (interface to extend, lines 47-66)
- pkg/neta/definition.go (bento structure, lines 49-63)
- tests/integration/product_automation_test.go (example integration test)
- examples/product-automation.bento.json (real-world example)

START WITH: Phase 1, write tests FIRST (TDD), commit after GREEN phase.
```

---

## References

- Current dry-run implementation: `cmd/bento/run.go:238-254`
- Neta interface: `pkg/neta/executable.go:47-66`
- Integration test examples: `tests/integration/product_automation_test.go`
- Go Proverbs: https://go-proverbs.github.io/

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-20 | Claude Code | Initial strategy document |
