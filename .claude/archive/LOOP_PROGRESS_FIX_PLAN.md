# Loop Progress Tracking Fix - Implementation Plan

**Date:** 2025-10-21
**Status:** Ready for implementation
**Estimated Effort:** 4-6 hours

## Problem Statement

Current Bento implementation:
- Counts loop child nodes in static graph structure
- Each loop iteration increments `completedNodes`
- Progress jumps incorrectly (e.g., 66% → 100% → 100% as iterations repeat)
- Formula: `progressPct = (completedNodes * 100) / totalNodes`

Example:
```
Bento with 3 nodes: [node1, loop(2 iterations, 1 child), node3]
Static count: 4 nodes (node1 + child + child + node3)

Execution:
- node1 completes: 25% ✓
- loop iter 1, child completes: 50% ✓
- loop iter 2, child completes: 75% ✗ (should still be in loop)
- node3 completes: 100% ✓

But child was counted twice, causing incorrect 75% reading.
```

## Solution: Leaf Node Loops (Atomiton Pattern)

Convert loops from **container nodes** to **leaf nodes**:
- Loop counts as 1 node regardless of iteration count
- Loop reports partial progress (0% → 100%) internally
- Child nodes execute inside loop, hidden from graph
- Simple, predictable, maintainable

## Implementation Steps

### Step 1: Add Partial Progress Support

**File:** `pkg/itamae/itamae.go`

```go
type Itamae struct {
    pantry         *pantry.Pantry
    logger         *shoyu.Logger
    messenger      ProgressMessenger
    onProgress     ProgressCallback
    slowMoDelay    time.Duration

    // New fields for partial progress
    nodeProgress   map[string]int           // nodeID -> progress (0-100)
    nodeMessages   map[string]string        // nodeID -> status message
    totalNodes     int
    completedNodes int
    mu             sync.RWMutex             // Protect progress state
}

// New method: Set partial progress for a node
func (i *Itamae) setNodeProgress(nodeID string, progress int, message string) {
    i.mu.Lock()
    defer i.mu.Unlock()

    // Clamp progress to 0-100
    if progress < 0 {
        progress = 0
    }
    if progress > 100 {
        progress = 100
    }

    i.nodeProgress[nodeID] = progress
    i.nodeMessages[nodeID] = message

    // Trigger progress callback
    i.notifyProgress(nodeID, message)
}

// New method: Calculate overall progress including partial progress
func (i *Itamae) calculateProgress() int {
    i.mu.RLock()
    defer i.mu.RUnlock()

    if i.totalNodes == 0 {
        return 0
    }

    // Count completed nodes as 100%, executing nodes by their partial progress
    totalProgress := i.completedNodes * 100

    // Add partial progress for executing nodes
    for _, progress := range i.nodeProgress {
        if progress < 100 {
            totalProgress += progress
        }
    }

    return totalProgress / i.totalNodes
}
```

### Step 2: Update Node Counting

**File:** `pkg/itamae/itamae.go`

```go
// countExecutableNodes counts nodes that contribute to progress.
// Loops are leaf nodes (count as 1), groups/parallel are containers (transparent).
func countExecutableNodes(def *neta.Definition) int {
    switch def.Type {
    case "loop":
        // Loop is a leaf node - count as 1 regardless of iterations
        return 1

    case "group", "parallel":
        // Container nodes - count children recursively
        count := 0
        for _, child := range def.Nodes {
            count += countExecutableNodes(&child)
        }
        return count

    default:
        // Regular leaf node - count as 1
        return 1
    }
}
```

### Step 3: Convert Loop to Leaf Node Execution

**File:** `pkg/itamae/loop.go`

```go
// executeLoop executes a loop as a leaf node with partial progress.
func (i *Itamae) executeLoop(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    i.notifyProgress(def.ID, "starting")

    // Send messenger event: node started
    if i.messenger != nil {
        i.messenger.SendNodeStarted(def.ID, def.Name, def.Type)
    }

    // Track execution time for messenger
    start := time.Now()

    // Get loop parameters
    mode, ok := def.Parameters["mode"].(string)
    if !ok {
        err := newNodeError(def.ID, "loop", "validate",
            fmt.Errorf("missing or invalid 'mode' parameter"))
        if i.messenger != nil {
            i.messenger.SendNodeCompleted(def.ID, time.Since(start), err)
        }
        return err
    }

    // Execute loop based on mode
    var err error
    switch mode {
    case "forEach":
        err = i.executeForEachLeaf(ctx, def, execCtx, result)
    case "times":
        err = i.executeTimesLeaf(ctx, def, execCtx, result)
    case "while":
        err = i.executeWhileLeaf(ctx, def, execCtx, result)
    default:
        err = newNodeError(def.ID, "loop", "validate",
            fmt.Errorf("unknown loop mode: %s", mode))
    }

    duration := time.Since(start)

    if err == nil {
        // Mark loop as completed
        i.mu.Lock()
        i.completedNodes++
        delete(i.nodeProgress, def.ID) // Clear partial progress
        i.mu.Unlock()

        i.notifyProgress(def.ID, "completed")
    }

    // Send messenger event: node completed
    if i.messenger != nil {
        i.messenger.SendNodeCompleted(def.ID, duration, err)
    }

    return err
}
```

### Step 4: Update forEach Implementation

**File:** `pkg/itamae/loop_foreach.go`

```go
// executeForEachLeaf executes a forEach loop as a leaf node.
// Reports partial progress as iterations complete.
func (i *Itamae) executeForEachLeaf(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Get the array to iterate over
    arrayValue := execCtx.resolveValue(def.Parameters["array"])
    array, ok := arrayValue.([]interface{})
    if !ok || array == nil {
        return newNodeError(def.ID, "loop", "validate",
            fmt.Errorf("forEach requires an array parameter"))
    }

    if len(array) == 0 {
        // Empty array - mark as 100% complete immediately
        i.setNodeProgress(def.ID, 100, "Empty array")
        return nil
    }

    if i.logger != nil {
        msg := msgLoopStarted(execCtx.depth, "forEach", def.Name, len(array))
        i.logger.Info(msg.format())
    }

    // Initialize progress
    i.setNodeProgress(def.ID, 0, fmt.Sprintf("Starting (0/%d)", len(array)))

    // Create loop context
    loopCtx := execCtx.createChild()

    // Execute iterations
    loopResults := make([]map[string]interface{}, 0, len(array))

    for idx, item := range array {
        // Check for cancellation
        select {
        case <-ctx.Done():
            return ctx.Err()
        default:
        }

        // Set loop variables
        loopCtx.set("item", item)
        loopCtx.set("index", idx)

        if i.logger != nil {
            msg := msgLoopIteration(execCtx.depth, idx+1, len(array))
            i.logger.Info(msg.format())
        }

        // Execute child nodes for this iteration
        iterationOutputs := make(map[string]interface{})
        for _, child := range def.Nodes {
            output, err := i.executeNodeInternal(ctx, &child, loopCtx, result)
            if err != nil {
                return err
            }
            iterationOutputs[child.ID] = output
        }

        loopResults = append(loopResults, iterationOutputs)

        // Update progress
        progress := ((idx + 1) * 100) / len(array)
        message := fmt.Sprintf("Iteration %d/%d", idx+1, len(array))
        i.setNodeProgress(def.ID, progress, message)
    }

    // Store loop results
    result.NodeOutputs[def.ID] = loopResults

    if i.logger != nil {
        msg := msgLoopCompleted(execCtx.depth, "forEach", len(array))
        i.logger.Info(msg.format())
    }

    return nil
}

// executeNodeInternal executes a node without incrementing global progress.
// Used for nodes inside loops (not counted in graph structure).
func (i *Itamae) executeNodeInternal(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) (interface{}, error) {
    // Log child node execution with proper indentation
    if i.logger != nil {
        msg := msgChildNodeStarted(execCtx.depth, def.Type, def.Name)
        i.logger.Info(msg.format())
    }

    // Get neta implementation from pantry
    netaImpl, err := i.pantry.GetNew(def.Type)
    if err != nil {
        return nil, newNodeError(def.ID, def.Type, "get neta", err)
    }

    // Prepare parameters with execution context
    params := make(map[string]interface{})
    for k, v := range def.Parameters {
        params[k] = execCtx.resolveValue(v)
    }
    params["_context"] = execCtx.toMap()
    params["_onOutput"] = func(line string) {
        if i.logger != nil {
            i.logger.Stream(line)
        }
    }

    // Track execution time
    start := time.Now()

    // Execute neta
    output, err := netaImpl.Execute(ctx, params)

    duration := time.Since(start)

    if err != nil {
        return nil, newNodeError(def.ID, def.Type, "execute", err)
    }

    // Store output in context for template resolution
    execCtx.set(def.ID, output)

    // Log completion
    if i.logger != nil {
        durationStr := formatDuration(duration)
        msg := msgChildNodeCompleted(execCtx.depth, def.Type, def.Name, durationStr, 0)
        i.logger.Info(msg.format())
    }

    return output, nil
}
```

### Step 5: Update times Implementation

**File:** `pkg/itamae/loop_times.go`

```go
// executeTimesLeaf executes a times loop as a leaf node.
func (i *Itamae) executeTimesLeaf(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Get count parameter
    countValue := execCtx.resolveValue(def.Parameters["count"])
    count, ok := countValue.(int)
    if !ok {
        return newNodeError(def.ID, "loop", "validate",
            fmt.Errorf("times loop requires integer 'count' parameter"))
    }

    if count <= 0 {
        // Zero iterations - mark as complete
        i.setNodeProgress(def.ID, 100, "No iterations")
        return nil
    }

    if i.logger != nil {
        msg := msgLoopStarted(execCtx.depth, "times", def.Name, count)
        i.logger.Info(msg.format())
    }

    // Initialize progress
    i.setNodeProgress(def.ID, 0, fmt.Sprintf("Starting (0/%d)", count))

    // Create loop context
    loopCtx := execCtx.createChild()

    // Execute iterations
    loopResults := make([]map[string]interface{}, 0, count)

    for iteration := 0; iteration < count; iteration++ {
        // Check for cancellation
        select {
        case <-ctx.Done():
            return ctx.Err()
        default:
        }

        // Set loop variables
        loopCtx.set("iteration", iteration)
        loopCtx.set("index", iteration) // Alias for consistency with forEach

        if i.logger != nil {
            msg := msgLoopIteration(execCtx.depth, iteration+1, count)
            i.logger.Info(msg.format())
        }

        // Execute child nodes for this iteration
        iterationOutputs := make(map[string]interface{})
        for _, child := range def.Nodes {
            output, err := i.executeNodeInternal(ctx, &child, loopCtx, result)
            if err != nil {
                return err
            }
            iterationOutputs[child.ID] = output
        }

        loopResults = append(loopResults, iterationOutputs)

        // Update progress
        progress := ((iteration + 1) * 100) / count
        message := fmt.Sprintf("Iteration %d/%d", iteration+1, count)
        i.setNodeProgress(def.ID, progress, message)
    }

    // Store loop results
    result.NodeOutputs[def.ID] = loopResults

    if i.logger != nil {
        msg := msgLoopCompleted(execCtx.depth, "times", count)
        i.logger.Info(msg.format())
    }

    return nil
}
```

### Step 6: Add New Log Messages

**File:** `pkg/itamae/messages.go`

```go
func msgLoopStarted(depth int, mode, name string, iterations int) logMessage {
    indent := strings.Repeat("  ", depth)
    return logMessage{
        text: fmt.Sprintf("%s├─ Loop (%s): %s - %d iterations", indent, mode, name, iterations),
    }
}

func msgLoopIteration(depth int, current, total int) logMessage {
    indent := strings.Repeat("  ", depth+1)
    return logMessage{
        text: fmt.Sprintf("%s├─ Iteration %d/%d", indent, current, total),
    }
}

func msgLoopCompleted(depth int, mode string, iterations int) logMessage {
    indent := strings.Repeat("  ", depth)
    return logMessage{
        text: fmt.Sprintf("%s└─ Loop (%s) completed - %d iterations", indent, mode, iterations),
    }
}
```

## Testing Plan

### Test 1: Simple Loop Progress
```go
func TestLoopProgress(t *testing.T) {
    // Bento: forEach loop with 3 iterations
    def := &neta.Definition{
        ID:   "root",
        Type: "group",
        Nodes: []neta.Definition{
            {
                ID:   "loop1",
                Type: "loop",
                Parameters: map[string]interface{}{
                    "mode":  "forEach",
                    "array": []interface{}{"a", "b", "c"},
                },
                Nodes: []neta.Definition{
                    {ID: "child1", Type: "file-write"},
                },
            },
        },
    }

    // Expected: 2 nodes total (loop1 = 1 node)
    // Progress: 0% → 33% → 66% → 100% (as loop progresses internally)
}
```

### Test 2: Nested Loops
```go
func TestNestedLoopProgress(t *testing.T) {
    // Outer loop: 2 iterations
    // Inner loop: 3 iterations
    // Total: 2 nodes (outer = 1, inner = 1)
    // Progress should be smooth 0% → 100%
}
```

### Test 3: Mixed Nodes and Loops
```go
func TestMixedProgress(t *testing.T) {
    // [node1, loop(3 iterations), node2]
    // Total: 3 nodes
    // Progress: 0% → 33% (node1) → 66% (loop completes) → 100% (node2)
}
```

## Migration Checklist

- [ ] Add `nodeProgress` and `nodeMessages` maps to `Itamae` struct
- [ ] Add `sync.RWMutex` for concurrent safety
- [ ] Implement `setNodeProgress()` method
- [ ] Implement `calculateProgress()` method
- [ ] Update `countExecutableNodes()` to treat loops as leaf nodes
- [ ] Update `executeLoop()` to use leaf node pattern
- [ ] Implement `executeForEachLeaf()` in `loop_foreach.go`
- [ ] Implement `executeTimesLeaf()` in `loop_times.go`
- [ ] Implement `executeNodeInternal()` for loop children
- [ ] Add loop log messages to `messages.go`
- [ ] Update unit tests
- [ ] Add integration tests for progress tracking
- [ ] Update documentation in `.claude/`

## Verification

After implementation, verify:
1. ✅ Progress is monotonically increasing (never goes backward)
2. ✅ Progress reaches exactly 100% at completion
3. ✅ Loop iterations don't inflate node count
4. ✅ Nested loops work correctly
5. ✅ Empty loops (0 iterations) complete immediately
6. ✅ Cancelled loops show partial progress

## Performance Notes

**Memory Impact:** Minimal
- 2 new maps: `nodeProgress` and `nodeMessages`
- Size: O(n) where n = number of nodes
- Typical bento: 10-50 nodes = ~1KB overhead

**Concurrency:** Safe
- `sync.RWMutex` protects progress state
- Read-heavy workload (progress queries)
- Write-light workload (progress updates)

**Calculation Cost:** O(n)
- Calculated on each node completion
- Cached in `calculateProgress()` result
- Typical bento: <1ms calculation time

## Go Idioms Applied

✅ **Simple is better than complex**
- No complex estimation algorithms
- Straightforward leaf node execution

✅ **Clear is better than clever**
- Explicit progress tracking
- Clear separation: loops vs groups

✅ **Accept interfaces, return structs**
- `ProgressMessenger` interface
- `Result` struct return type

✅ **A little copying is better than a little dependency**
- No external progress tracking library
- Self-contained in itamae package

✅ **Make the zero value useful**
- Empty `nodeProgress` map = 0% progress
- Empty array loop = immediate completion

## References

- Full analysis: `.claude/ATOMITON_PROGRESS_TRACKING_ANALYSIS.md`
- Atomiton conductor: `/Users/Ryan/Code/atomiton/packages/@atomiton/conductor`
- Bento Box Principle: `.claude/BENTO_BOX_PRINCIPLE.md`
