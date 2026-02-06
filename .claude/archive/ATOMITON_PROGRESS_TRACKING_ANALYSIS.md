# Atomiton Progress Tracking Analysis

**Date:** 2025-10-21
**Purpose:** Learn from Atomiton's conductor package to improve Bento's itamae loop progress tracking

## Executive Summary

Atomiton uses a **weight-based progress system** that counts static graph structure, not dynamic loop iterations. This is the same approach Bento currently uses, which causes the progress inaccuracies you've observed.

**Key Finding:** Atomiton does NOT solve the loop iteration counting problem. They accept that progress tracks the graph structure, not runtime iterations.

## 1. Progress Tracking Architecture

### Atomiton's Approach

**File:** `/packages/@atomiton/conductor/src/execution/ProgressCalculator.ts`

```typescript
export function calculateProgress(state: ExecutionGraphState): number {
  if (state.nodes.size === 0 || state.totalWeight === 0) return 0;

  let completedWeight = 0;

  for (const node of state.nodes.values()) {
    if (node.state === "completed" || node.state === "skipped") {
      completedWeight += node.weight;
    } else if (node.state === "executing") {
      // Include executing nodes' partial progress
      completedWeight += node.weight * (node.progress / 100);
    }
    // Error nodes contribute 0 to overall progress (execution failed)
  }

  return Math.round((completedWeight / state.totalWeight) * 100);
}
```

**Key Points:**
- Progress is **weighted** by node type (fast nodes = low weight, slow nodes = high weight)
- Nodes can report **partial progress** (0-100%) during execution
- Progress is **cached** and only recalculated on state mutations
- Total weight is calculated once at graph initialization

### Bento's Current Approach

**File:** `/pkg/itamae/executor.go` (lines 122-127)

```go
// Increment completed count and calculate progress percentage
i.completedNodes++
progressPct := 0
if i.totalNodes > 0 {
    progressPct = (i.completedNodes * 100) / i.totalNodes
}
```

**Key Points:**
- Simple count-based progress (no weights)
- No partial progress during execution
- Counts static graph structure via `countExecutableNodes()`
- Loops are counted as 1 node, regardless of iterations

## 2. Loop Handling

### Atomiton's Loop Implementation

**Files:**
- `/packages/@atomiton/nodes/src/executables/loop/operations.ts`
- `/packages/@atomiton/nodes/src/executables/loop/baseExecutor.ts`

**Loop Types:**
```typescript
// forEach: iterates over array
export async function executeForEach(
  items: unknown[],
  config: LoopParameters,
  context: NodeExecutionContext,
): Promise<LoopResult>

// times: repeats N times
export async function executeTimesLoop(
  times: number,
  config: LoopParameters,
  context: NodeExecutionContext,
): Promise<LoopResult>

// while/until: condition-based loops
export async function executeConditionLoop(...)
```

**Critical Discovery:**
```typescript
// From loop/index.ts
export const loopExecutable = createExecutable<LoopParameters>(
  "loop",
  async ({ getInput, config, context, getDuration }) => {
    // Loop executes internally, returns results array
    const loopResult = await executeForEach(array, configWithDefaults, context);

    return {
      results: loopResult.results,
      iterationCount: loopResult.iterationCount,
      errors: loopResult.errors,
      success: errors.length === 0,
      duration,
    };
  },
);
```

**Loops in Atomiton are LEAF NODES** - they:
1. Execute their iterations internally
2. Count as ONE node in the graph
3. Report progress as a single unit (0% → 100%)
4. Do NOT expose child nodes to the graph analyzer

### Bento's Loop Implementation

**File:** `/pkg/itamae/loop_foreach.go`

```go
func (i *Itamae) executeForEach(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Get the array to iterate over
    arrayValue := execCtx.resolveValue(def.Parameters["array"])
    array, ok := arrayValue.([]interface{})

    // Loop counter for context
    loopCtx := execCtx.createChild()

    // Iterate over array
    for idx, item := range array {
        loopCtx.set("item", item)
        loopCtx.set("index", idx)

        // Execute child nodes for this iteration
        for _, child := range def.Nodes {
            if err := i.executeNode(ctx, &child, loopCtx, result); err != nil {
                return err
            }
        }
    }
}
```

**Bento loops are CONTAINER NODES** - they:
1. Execute child nodes multiple times
2. Child nodes get counted in `countExecutableNodes()`
3. Progress tracking sees the child nodes complete multiple times
4. This causes progress to jump (e.g., 66% → 100% → 100%)

## 3. Event System

### Atomiton's Event Broadcasting

**File:** `/packages/@atomiton/conductor/src/events/ConductorEventEmitter.ts`

```typescript
export class ConductorEventEmitter extends EventEmitter<ConductorEvents> {
  /**
   * Throttled progress emitter to prevent IPC flooding
   * Default: Max 10 progress updates per second (100ms throttle)
   */
  private throttledProgressEmit: ((state: ExecutionGraphState) => void) & {
    cancel: () => void;
    flush: () => void;
  };

  constructor(progressThrottleMs: number = 100) {
    super();

    // Create throttled version of progress emission
    this.throttledProgressEmit = throttle(
      (state: ExecutionGraphState) => {
        this.emit("progress", state);
      },
      progressThrottleMs,
      { leading: true, trailing: true },
    );
  }
}
```

**Key Patterns:**
- **Throttling** prevents IPC flooding (100ms = max 10 updates/sec)
- **Leading + Trailing** ensures first and last update always sent
- **Flush** method for immediate critical updates
- **Zustand store subscription** triggers events automatically

**Integration:**
```typescript
// From conductor.ts
const executionGraphStore = createExecutionGraphStore();
const events = createConductorEventEmitter();

// Wire up store changes to emit events
executionGraphStore.subscribe((state: ExecutionGraphState) => {
  events.emitProgress(state);
});
```

### Bento's Event System

**File:** `/pkg/itamae/messages.go`

```go
type ProgressMessenger interface {
    SendNodeStarted(path, name, nodeType string)
    SendNodeCompleted(path string, duration time.Duration, err error)
}
```

**Current State:**
- Simple callback interface
- No throttling
- No state broadcasting
- Only node-level events (start/complete)

## 4. Go Performance Patterns

### Pattern 1: Cached Progress Calculation

**TypeScript (Atomiton):**
```typescript
export type ExecutionGraphState = {
  nodes: Map<string, ExecutionGraphNode>;
  totalWeight: number;
  cachedProgress: number; // Cached weighted progress (0-100)
  // ... other fields
};

// Only recalculate on state mutations
function setNodeState(nodeId: string, state: NodeExecutionState, error?: string) {
  store.setState((draft: ExecutionGraphState) => {
    // ... update node state

    // Update cached progress
    draft.cachedProgress = calculateProgress(draft);
  });
}
```

**Go Translation:**
```go
type executionState struct {
    nodes          map[string]*nodeState
    totalWeight    int
    cachedProgress int // Cache progress to avoid recalculation
    mu             sync.RWMutex
}

func (s *executionState) setNodeState(nodeID string, state nodeState) {
    s.mu.Lock()
    defer s.mu.Unlock()

    s.nodes[nodeID] = &state
    s.cachedProgress = s.calculateProgress() // O(n) but only on mutations
}

func (s *executionState) getProgress() int {
    s.mu.RLock()
    defer s.mu.RUnlock()
    return s.cachedProgress // O(1) read
}
```

### Pattern 2: Event Throttling with Channels

**Go Implementation:**
```go
type progressThrottler struct {
    events    chan ExecutionState
    interval  time.Duration
    broadcast func(ExecutionState)
}

func newProgressThrottler(interval time.Duration, broadcast func(ExecutionState)) *progressThrottler {
    t := &progressThrottler{
        events:    make(chan ExecutionState, 100), // Buffered
        interval:  interval,
        broadcast: broadcast,
    }
    go t.run()
    return t
}

func (t *progressThrottler) run() {
    ticker := time.NewTicker(t.interval)
    defer ticker.Stop()

    var latest ExecutionState
    hasUpdate := false

    for {
        select {
        case state := <-t.events:
            latest = state
            hasUpdate = true
        case <-ticker.C:
            if hasUpdate {
                t.broadcast(latest)
                hasUpdate = false
            }
        }
    }
}

func (t *progressThrottler) emit(state ExecutionState) {
    select {
    case t.events <- state:
        // Queued successfully
    default:
        // Channel full, drop this update
    }
}
```

### Pattern 3: Avoiding Race Conditions

**Atomiton's Store Pattern:**
```typescript
// Uses Zustand + Immer for immutable updates
store.setState((draft: ExecutionGraphState) => {
  // Immer makes draft mutable-looking but creates new state
  const node = draft.nodes.get(nodeId);
  if (!node) return;

  node.state = state;
  draft.cachedProgress = calculateProgress(draft);
});
```

**Go Translation:**
```go
type executionStore struct {
    state *executionState
    mu    sync.RWMutex
    subs  []func(ExecutionState)
}

func (s *executionStore) updateState(fn func(*executionState)) {
    s.mu.Lock()

    // Create copy for mutation
    newState := s.state.copy()
    fn(newState)

    // Atomic swap
    s.state = newState
    s.mu.Unlock()

    // Notify subscribers (outside lock)
    s.notifySubscribers()
}

func (s *executionStore) notifySubscribers() {
    s.mu.RLock()
    state := s.state.snapshot() // Read-only snapshot
    subs := s.subs
    s.mu.RUnlock()

    for _, sub := range subs {
        sub(state) // Run callbacks without holding lock
    }
}
```

### Pattern 4: Partial Progress Tracking

**Atomiton:**
```typescript
function setNodeProgress(nodeId: string, progress: number, message?: string) {
  const clampedProgress = Math.min(100, Math.max(0, progress));

  store.setState((draft: ExecutionGraphState) => {
    const node = draft.nodes.get(nodeId);
    if (!node) return;

    node.progress = clampedProgress;
    if (message !== undefined) {
      node.message = message;
    }

    // Update overall progress based on partial progress
    draft.cachedProgress = calculateProgress(draft);
  });
}
```

**Go Translation:**
```go
type nodeProgress struct {
    progress int    // 0-100
    message  string
}

func (i *Itamae) setNodeProgress(nodeID string, progress int, message string) {
    // Clamp to 0-100
    if progress < 0 {
        progress = 0
    }
    if progress > 100 {
        progress = 100
    }

    i.state.updateState(func(s *executionState) {
        node, ok := s.nodes[nodeID]
        if !ok {
            return
        }

        node.progress = progress
        node.message = message

        // Recalculate overall progress
        s.cachedProgress = s.calculateProgress()
    })
}
```

## 5. Recommendations for Bento

### Option A: Keep Container Loops, Fix Progress Tracking

**Pros:**
- Maintains visibility into loop execution
- Progress shows actual work being done
- Useful for debugging long loops

**Implementation:**
```go
// Count expected iterations at graph initialization
func (i *Itamae) analyzeGraph(def *neta.Definition) *graphAnalysis {
    analysis := &graphAnalysis{
        totalNodes: 0,
        nodeWeights: make(map[string]int),
    }

    i.analyzeNode(def, analysis, 1) // multiplier = 1 initially
    return analysis
}

func (i *Itamae) analyzeNode(def *neta.Definition, analysis *graphAnalysis, multiplier int) {
    switch def.Type {
    case "loop":
        // Estimate iterations
        iterations := i.estimateLoopIterations(def)

        // Count child nodes multiplied by iterations
        for _, child := range def.Nodes {
            i.analyzeNode(&child, analysis, multiplier * iterations)
        }

    case "group", "parallel":
        // Count children with current multiplier
        for _, child := range def.Nodes {
            i.analyzeNode(&child, analysis, multiplier)
        }

    default:
        // Leaf node: count it (multiplied)
        analysis.totalNodes += multiplier
    }
}

func (i *Itamae) estimateLoopIterations(def *neta.Definition) int {
    mode := def.Parameters["mode"].(string)

    switch mode {
    case "times":
        // Known iteration count
        return def.Parameters["count"].(int)
    case "forEach":
        // Known array length
        array := def.Parameters["array"].([]interface{})
        return len(array)
    case "while":
        // Unknown - use default estimate (e.g., 10)
        return 10
    default:
        return 1
    }
}
```

**Cons:**
- Complex estimation logic
- While loops can't be predicted
- Template-based loop arrays require resolution
- Multiplier can explode with nested loops

### Option B: Convert Loops to Leaf Nodes (Atomiton Style)

**Pros:**
- Simple, predictable progress
- Loops count as 1 node regardless of iterations
- No estimation needed
- Matches user mental model ("loop" = 1 thing)

**Implementation:**
```go
func (i *Itamae) executeLoop(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Loop is now a leaf node - handle internally
    mode := def.Parameters["mode"].(string)

    switch mode {
    case "forEach":
        return i.executeForEachLeaf(ctx, def, execCtx, result)
    case "times":
        return i.executeTimesLeaf(ctx, def, execCtx, result)
    }
}

func (i *Itamae) executeForEachLeaf(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Report loop as 1 node with partial progress
    i.setNodeProgress(def.ID, 0, "Starting loop")

    array := execCtx.resolveValue(def.Parameters["array"]).([]interface{})
    loopResults := make([]interface{}, 0, len(array))

    for idx, item := range array {
        // Update partial progress
        progress := (idx * 100) / len(array)
        i.setNodeProgress(def.ID, progress, fmt.Sprintf("Iteration %d/%d", idx+1, len(array)))

        // Create child context
        loopCtx := execCtx.createChild()
        loopCtx.set("item", item)
        loopCtx.set("index", idx)

        // Execute child nodes internally
        iterationResult := make(map[string]interface{})
        for _, child := range def.Nodes {
            output, err := i.executeNodeInternal(ctx, &child, loopCtx)
            if err != nil {
                return err
            }
            iterationResult[child.ID] = output
        }

        loopResults = append(loopResults, iterationResult)
    }

    // Complete loop
    i.setNodeProgress(def.ID, 100, "Completed")
    i.completedNodes++

    // Store results
    result.NodeOutputs[def.ID] = loopResults

    return nil
}
```

**Cons:**
- Hides loop internals from progress tracking
- Child nodes not visible in progress updates
- Different execution model than groups

### Option C: Hybrid Approach with Weights

**Pros:**
- Best of both worlds
- Container loops with weighted progress
- More accurate than simple counting

**Implementation:**
```go
type nodeWeight struct {
    baseWeight     int // Static weight per node type
    multiplier     int // For loops: iteration count
    totalWeight    int // baseWeight * multiplier
}

func (i *Itamae) calculateWeights(def *neta.Definition) map[string]nodeWeight {
    weights := make(map[string]nodeWeight)
    i.analyzeNodeWeights(def, weights, 1)
    return weights
}

func (i *Itamae) analyzeNodeWeights(def *neta.Definition, weights map[string]nodeWeight, multiplier int) {
    baseWeight := getBaseWeight(def.Type)

    switch def.Type {
    case "loop":
        iterations := i.estimateLoopIterations(def)

        // Loop container has weight
        weights[def.ID] = nodeWeight{
            baseWeight:  baseWeight,
            multiplier:  1,
            totalWeight: baseWeight,
        }

        // Children inherit loop multiplier
        for _, child := range def.Nodes {
            i.analyzeNodeWeights(&child, weights, multiplier * iterations)
        }

    case "group", "parallel":
        weights[def.ID] = nodeWeight{
            baseWeight:  baseWeight,
            multiplier:  1,
            totalWeight: baseWeight,
        }

        for _, child := range def.Nodes {
            i.analyzeNodeWeights(&child, weights, multiplier)
        }

    default:
        // Leaf node
        weights[def.ID] = nodeWeight{
            baseWeight:  baseWeight,
            multiplier:  multiplier,
            totalWeight: baseWeight * multiplier,
        }
    }
}

func getBaseWeight(nodeType string) int {
    weights := map[string]int{
        "shell-command": 300,  // Slow
        "file-write":    50,   // Fast
        "template":      100,  // Medium
        "group":         0,    // No intrinsic weight
        "loop":          0,    // No intrinsic weight
        "parallel":      0,    // No intrinsic weight
    }

    if w, ok := weights[nodeType]; ok {
        return w
    }
    return 100 // Default
}

func (i *Itamae) calculateProgress() int {
    totalWeight := 0
    completedWeight := 0

    for nodeID, weight := range i.weights {
        totalWeight += weight.totalWeight

        node := i.state.nodes[nodeID]
        if node.state == "completed" {
            completedWeight += weight.totalWeight
        } else if node.state == "executing" {
            // Partial progress
            completedWeight += (weight.totalWeight * node.progress) / 100
        }
    }

    if totalWeight == 0 {
        return 0
    }

    return (completedWeight * 100) / totalWeight
}
```

## 6. Recommended Approach for Bento

**Use Option B: Leaf Node Loops** (Atomiton's approach)

**Rationale:**
1. **Simplicity:** No complex estimation or weight calculations
2. **Predictability:** Loops always count as 1 node
3. **User Mental Model:** "Loop" is conceptually 1 operation
4. **Maintainability:** Less code, fewer edge cases
5. **Standard Library First:** Avoid premature complexity

**Implementation Plan:**

1. **Modify Loop Execution** (1 file)
   - Convert loop executor to leaf node style
   - Add partial progress reporting
   - Execute child nodes internally

2. **Update Progress Tracking** (1 file)
   - Add `setNodeProgress(id, progress, message)` method
   - Add partial progress to calculation
   - Cache progress to avoid recalculation

3. **Add Event Throttling** (1 file, optional)
   - Create simple throttler with Go channels
   - Default 100ms interval (10 updates/sec)
   - Use for messenger updates

4. **Update Count Function** (1 file)
   - Loops count as 1 node
   - Remove recursive counting for loop children
   - Keep recursive counting for groups/parallel

**Files to Change:**
```
pkg/itamae/loop.go           # Convert to leaf node execution
pkg/itamae/loop_foreach.go   # Update forEach implementation
pkg/itamae/loop_times.go     # Update times implementation
pkg/itamae/executor.go       # Add setNodeProgress method
pkg/itamae/itamae.go        # Update countExecutableNodes
pkg/itamae/messages.go       # Add progress event (optional)
```

**Estimated Effort:** 4-6 hours

## 7. Anti-Patterns to Avoid

### ❌ Don't: Use reflection for progress tracking
```go
// BAD - Violates Go idioms
func (i *Itamae) getProgress() int {
    val := reflect.ValueOf(i.state)
    // ...
}
```

### ❌ Don't: Create a "utils" package for progress
```go
// BAD - Bento Box violation
pkg/utils/progress.go  // Grab bag of helpers
```

### ❌ Don't: Over-engineer with generics
```go
// BAD - YAGNI
func CalculateProgress[T NodeState](nodes []T) int {
    // Unnecessary complexity
}
```

### ✅ Do: Keep it in the itamae package
```go
// GOOD - Single responsibility
pkg/itamae/progress.go  // Progress tracking for itamae
```

### ✅ Do: Use simple types
```go
// GOOD - Clear and simple
type nodeProgress struct {
    state    string
    progress int
    message  string
}
```

### ✅ Do: Use standard library patterns
```go
// GOOD - Standard sync primitives
type progressState struct {
    mu       sync.RWMutex
    progress int
}
```

## 8. Code Examples from Atomiton

### Example 1: Graph Analysis
**File:** `/packages/@atomiton/nodes/src/graph/graphAnalyzer.ts:36-59`

```typescript
export function analyzeExecutionGraph(flow: NodeDefinition): ExecutionGraph {
  // Handle single nodes by creating a 1-node graph
  if (!flow.nodes || flow.nodes.length === 0) {
    const weight =
      DEFAULT_NODE_WEIGHTS[flow.type] || DEFAULT_NODE_WEIGHTS.default;
    const graphNodes = new Map<string, GraphNode>();
    graphNodes.set(flow.id, {
      id: flow.id,
      name: flow.name || flow.type,
      type: flow.type,
      weight,
      dependencies: [],
      dependents: [],
      level: 0,
    });

    return {
      nodes: graphNodes,
      totalWeight: weight,
      criticalPath: [flow.id],
      criticalPathWeight: weight,
      maxParallelism: 1,
      executionOrder: [[flow.id]],
    };
  }
  // ... continue with multi-node graphs
}
```

### Example 2: Progress Controller
**File:** `/packages/@atomiton/conductor/src/execution/progressController.ts:62-105`

```typescript
export function createProgressController(
  node: NodeDefinition,
  executionGraphStore: ExecutionGraphStore,
  slowMo: number,
): {
  start: () => Promise<void>;
  markComplete: () => Promise<void>;
  cancel: () => void;
} {
  const nodeState = executionGraphStore.getState().nodes.get(node.id);
  const weight = nodeState?.weight ?? 100;

  const { steps, messages } = generateProgressSteps(weight);
  let markedComplete = false;
  let cancelled = false;

  return {
    start: async () => {
      for (let i = 0; i < steps.length; i++) {
        if (cancelled) break;

        executionGraphStore.setNodeProgress(node.id, steps[i], messages[i]);

        if (i < steps.length - 1) {
          await delay(slowMo);
        }
      }
    },
    markComplete: async () => {
      if (!markedComplete) {
        executionGraphStore.setNodeProgress(node.id, 100, "Complete");
        await delay(slowMo / 2);
        executionGraphStore.setNodeState(node.id, "completed");
        markedComplete = true;
      }
    },
    cancel: () => {
      cancelled = true;
    },
  };
}
```

### Example 3: Store Subscription Pattern
**File:** `/packages/@atomiton/conductor/src/conductor.ts:111-113`

```typescript
// Wire up store changes to emit events
executionGraphStore.subscribe((state: ExecutionGraphState) => {
  events.emitProgress(state);
});
```

## 9. Performance Considerations

### Memory Usage

**Atomiton:** Uses Map for nodes (O(1) lookups)
```typescript
nodes: Map<string, ExecutionGraphNode>
```

**Bento:** Should use map[string]*nodeState
```go
type executionState struct {
    nodes map[string]*nodeState  // O(1) lookups
}
```

### Concurrency

**Atomiton:** JavaScript is single-threaded, no locks needed
**Bento:** Go is concurrent, requires careful locking

```go
// Read-heavy workload: prefer RWMutex
type progressState struct {
    mu       sync.RWMutex
    progress int
}

// Many readers, few writers
func (s *progressState) getProgress() int {
    s.mu.RLock()
    defer s.mu.RUnlock()
    return s.progress
}

// Infrequent writes
func (s *progressState) setProgress(p int) {
    s.mu.Lock()
    defer s.mu.Unlock()
    s.progress = p
}
```

### Progress Calculation Frequency

**Atomiton:** O(n) on mutations, O(1) on reads (cached)
**Bento:** Should match this pattern

```go
// GOOD - Cache and only recalculate on mutations
func (s *executionState) updateNode(id string, state string) {
    s.mu.Lock()
    s.nodes[id].state = state
    s.cachedProgress = s.calculateProgress() // O(n) but infrequent
    s.mu.Unlock()
}

func (s *executionState) getProgress() int {
    s.mu.RLock()
    defer s.mu.RUnlock()
    return s.cachedProgress // O(1)
}
```

## 10. Testing Insights

**File:** `/packages/@atomiton/conductor/src/execution/asyncProgress.test.ts`

Key test scenarios:
1. Random delays (10-100ms) completing at different rates
2. Fast nodes (<5ms) vs slow nodes (>200ms)
3. Monotonically increasing progress
4. Final progress reaches 100%
5. Out-of-order completion
6. Concurrent progress updates

**Go Test Structure:**
```go
func TestProgressTracking(t *testing.T) {
    tests := []struct {
        name     string
        nodes    []NodeDef
        expected []int // Expected progress snapshots
    }{
        {
            name: "simple linear progress",
            nodes: []NodeDef{
                {ID: "1", Type: "file-write"},
                {ID: "2", Type: "file-write"},
                {ID: "3", Type: "file-write"},
            },
            expected: []int{0, 33, 66, 100},
        },
        {
            name: "loop with 3 iterations",
            nodes: []NodeDef{
                {ID: "loop1", Type: "loop", Mode: "times", Count: 3},
            },
            expected: []int{0, 100}, // Loop counts as 1 node
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            // Test implementation
        })
    }
}
```

## Conclusion

Atomiton provides excellent patterns for:
- ✅ Event-driven architecture with throttling
- ✅ Weighted progress calculations
- ✅ Cached progress for performance
- ✅ Partial progress during execution
- ✅ Clean separation of concerns

However, Atomiton **does NOT solve the loop iteration counting problem**. They treat loops as leaf nodes that report 0% → 100%, which is the recommended approach for Bento.

**Next Steps:**
1. Implement Option B (Leaf Node Loops)
2. Add partial progress tracking
3. Add event throttling (optional, for TUI)
4. Update tests to verify monotonic progress
5. Document the new progress model in `.claude/`
