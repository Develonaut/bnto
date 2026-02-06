# Atomiton Hierarchical Progress Tracking - Deep Dive

**Date:** 2025-10-21
**Purpose:** Detailed investigation of Atomiton's hierarchical progress tracking to answer specific implementation questions

## Executive Summary

Atomiton **DOES** track hierarchical progress, but NOT in the way initially assumed. The hierarchy exists at:
1. **Graph level** - Overall execution progress (0-100%)
2. **Per-node level** - Each node tracks its own progress (0-100%)
3. **Weighted calculation** - Progress bubbles up via weighted summation, NOT parent/child relationships

**Critical Finding:** Atomiton does NOT track progress hierarchically by container nodes (groups/loops). Instead, ALL nodes are flattened into a single graph, and progress is calculated using a weighted sum across all nodes.

---

## 1. Node-Level Progress Tracking

### Node Progress Fields

**File:** `/packages/@atomiton/conductor/src/execution/types.ts` (Lines 18-28)

```typescript
export type ExecutionGraphNode = GraphNode & {
  state: NodeExecutionState;      // "pending" | "executing" | "completed" | "error" | "skipped"
  progress: number;                 // 0-100 (per-node progress)
  message?: string;                 // Status message
  startTime?: number;
  endTime?: number;
  error?: string;
  nodes?: ExecutionGraphNode[];     // Recursive - child nodes with their own progress
};
```

**Key Insight:** The `nodes?` field exists on the TYPE but is NOT used by the conductor for progress tracking. It's a vestigial field from the original node definition structure.

### Progress Tracking in Store

**File:** `/packages/@atomiton/conductor/src/execution/executionGraphStore.ts` (Lines 208-260)

```typescript
function setNodeProgress(nodeId: string, progress: number, message?: string) {
  const state = store.getState();
  const node = state.nodes.get(nodeId);
  if (!node) {
    getLogger().warn("Attempted to update progress for non-existent node", {
      nodeId,
      progress,
    });
    return;
  }

  const clampedProgress = Math.min(100, Math.max(0, progress));

  getLogger().debug("Node progress updated", {
    nodeId,
    nodeName: node.name,
    previousProgress: node.progress,
    newProgress: clampedProgress,
    message,
  });

  // Record in trace
  const allNodes = (
    Array.from(state.nodes.values()) as ExecutionGraphNode[]
  ).map((n) => ({
    id: n.id,
    name: n.name,
    state: n.state,
    progress: n.progress,
  }));
  traceManager.recordNodeProgress(
    nodeId,
    clampedProgress,
    message,
    state.cachedProgress,  // IMPORTANT: Overall graph progress
    allNodes,
  );

  store.setState((draft: ExecutionGraphState) => {
    const node = draft.nodes.get(nodeId);
    if (!node) return;

    node.progress = clampedProgress;  // Per-node progress
    if (message !== undefined) {
      node.message = message;
    }

    // Update cached progress
    draft.cachedProgress = calculateProgress(draft);  // Recalculate OVERALL progress
  });
}
```

**Answer to Question 1:**
✅ **Each node has a `progress` field (0-100)**
✅ **Progress is stored per-node in the execution graph store**
❌ **Container nodes (groups/loops) do NOT track child progress separately**

---

## 2. Progress Calculation Hierarchy

### How Graph Progress is Calculated

**File:** `/packages/@atomiton/conductor/src/execution/ProgressCalculator.ts` (Lines 11-27)

```typescript
export function calculateProgress(state: ExecutionGraphState): number {
  if (state.nodes.size === 0 || state.totalWeight === 0) return 0;

  let completedWeight = 0;

  // FLAT iteration - ALL nodes are at the same level
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

**Key Findings:**
1. **Flat structure** - All nodes are iterated at the same level
2. **Weight-based** - Each node has a weight based on its type
3. **Partial progress** - Executing nodes contribute fractionally
4. **Single-pass calculation** - O(n) over all nodes

### Graph Initialization (Where the "Flattening" Happens)

**File:** `/packages/@atomiton/conductor/src/execution/executionGraphStore.ts` (Lines 78-120)

```typescript
function initializeGraph(graph: ExecutionGraph) {
  getLogger().info("Initializing execution graph", {
    nodeCount: graph.nodes.size,
    totalWeight: graph.totalWeight,
    criticalPathLength: graph.criticalPath.length,
    maxParallelism: graph.maxParallelism,
    executionLayers: graph.executionOrder.length,
  });

  // Initialize trace
  traceManager.initializeTrace(graph);

  store.setState((draft: ExecutionGraphState) => {
    // Convert graph nodes to execution graph nodes
    const executionNodes = new Map<string, ExecutionGraphNode>();
    graph.nodes.forEach((node, id) => {
      executionNodes.set(id, {
        ...node,
        state: "pending",
        progress: 0,  // All nodes start at 0%
      });
    });

    // Convert edges to simple format
    const edges: Array<{ from: string; to: string }> = [];
    graph.nodes.forEach((node) => {
      node.dependencies.forEach((depId) => {
        edges.push({ from: depId, to: node.id });
      });
    });

    draft.nodes = executionNodes;
    draft.edges = edges;
    draft.executionOrder = graph.executionOrder;
    draft.criticalPath = graph.criticalPath;
    draft.totalWeight = graph.totalWeight;
    draft.maxParallelism = graph.maxParallelism;
    draft.isExecuting = true;
    draft.startTime = Date.now();
    draft.endTime = null;
    draft.cachedProgress = 0; // All nodes start as pending
  });
}
```

**Answer to Question 2:**
❌ **Groups do NOT calculate progress from children hierarchically**
❌ **Loops do NOT calculate progress from iterations hierarchically**
✅ **Progress is calculated as a weighted sum across ALL nodes in a FLAT structure**

**Critical Discovery:** The graph analyzer (`analyzeExecutionGraph`) creates a flat Map of ALL nodes, including children of groups and loops. There is NO hierarchical calculation.

---

## 3. Event Structure

### What's Included in Progress Events

**File:** `/packages/@atomiton/conductor/src/events/ConductorEventEmitter.ts` (Lines 55-60, 99-101)

```typescript
type ConductorEvents = {
  progress: (state: ExecutionGraphState) => void;
  started: (data: { executionId: string; nodeId: string }) => void;
  completed: (data: { executionId: string; result: ExecutionResult }) => void;
  error: (data: { executionId: string; error: Error }) => void;
};

// Progress emission
emitProgress(state: ExecutionGraphState): void {
  this.throttledProgressEmit(state);
}
```

### ExecutionGraphState Structure

**File:** `/packages/@atomiton/conductor/src/execution/types.ts` (Lines 33-44)

```typescript
export type ExecutionGraphState = {
  nodes: Map<string, ExecutionGraphNode>;  // ALL nodes (flat)
  edges: Array<{ from: string; to: string }>;
  executionOrder: string[][];              // Topologically sorted layers
  criticalPath: string[];
  totalWeight: number;                     // Sum of all node weights
  maxParallelism: number;
  isExecuting: boolean;
  startTime: number | null;
  endTime: number | null;
  cachedProgress: number;                  // OVERALL graph progress (0-100)
};
```

**Answer to Question 3:**
✅ **Progress events include BOTH node-level and graph-level progress**
- `state.cachedProgress` = overall graph progress (0-100%)
- `state.nodes.get(nodeId).progress` = individual node progress (0-100%)

**Event Data Structure:**
```typescript
{
  cachedProgress: 45,  // Overall: 45% complete
  nodes: Map {
    "node-1" => { progress: 100, state: "completed" },
    "node-2" => { progress: 60, state: "executing" },  // Currently at 60%
    "node-3" => { progress: 0, state: "pending" },
  },
  totalWeight: 500,
  // ... other metadata
}
```

---

## 4. How Groups Handle Progress

### Group Execution

**File:** `/packages/@atomiton/conductor/src/execution/executeGraph.ts` (Lines 44-76)

```typescript
// Handle single nodes (no children)
if (!node.nodes || node.nodes.length === 0) {
  // Apply visual delay BEFORE node execution for single nodes too
  const slowMoDelay =
    config.debugController?.getSlowMoDelay() ?? context.slowMo ?? 0;
  if (slowMoDelay > 0) {
    const { delay: wait } = await import("@atomiton/utils");
    await wait(slowMoDelay);
  }

  // Desktop conductor ALWAYS uses local execution
  const result = await executeGraphNode(
    node,
    context,
    startTime,
    config,
    executionGraphStore,
  );

  completeExecution(executionGraphStore);

  // Include trace in result
  return {
    ...result,
    trace: executionGraphStore?.getTrace(),
  };
}

// Use topological sort from graph analyzer and flatten levels to sequential execution
const sortedLevels = topologicalSort(node.nodes, node.edges || []);
const sortedIds = sortedLevels.flat();
const sorted = sortedIds
  .map((id) => node.nodes!.find((n) => n.id === id)!)
  .filter(Boolean);
```

**Critical Finding:** When executing a group:
1. The group's **child nodes** are executed sequentially
2. Each child node reports its OWN progress to the store
3. The GROUP itself does NOT have a progress value
4. The overall graph progress reflects the sum of ALL child progress

### Graph Analysis (Where Children are Flattened)

**File:** `/packages/@atomiton/nodes/src/graph/graphAnalyzer.ts` (Lines 36-100)

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

  const nodes = flow.nodes;
  const edges = flow.edges || [];

  // Build adjacency maps
  const dependencyMap = new Map<string, string[]>();
  const dependentMap = new Map<string, string[]>();

  nodes.forEach((node) => {
    dependencyMap.set(node.id, []);
    dependentMap.set(node.id, []);
  });

  edges.forEach((edge) => {
    const deps = dependencyMap.get(edge.target) || [];
    deps.push(edge.source);
    dependencyMap.set(edge.target, deps);

    const dependents = dependentMap.get(edge.source) || [];
    dependents.push(edge.target);
    dependentMap.set(edge.source, dependents);
  });

  // Topological sort for execution order
  const executionOrder = topologicalSort(nodes, edges);

  // Find critical path
  const { path: criticalPath, weight: criticalPathWeight } = findCriticalPath(
    nodes,
    edges,
  );

  // Calculate max parallelism
  const maxParallelism = findParallelBranches(executionOrder);

  // Build graph nodes with metadata
  const graphNodes = new Map<string, GraphNode>();
  let totalWeight = 0;

  // Assign levels based on execution order
  executionOrder.forEach((level, levelIndex) => {
    level.forEach((nodeId) => {
      const node = nodes.find((n) => n.id === nodeId)!;
      const weight =
        DEFAULT_NODE_WEIGHTS[node.type] || DEFAULT_NODE_WEIGHTS.default;

      graphNodes.set(nodeId, {
        id: node.id,
        name: node.name || node.type,
        type: node.type,
        weight,
        dependencies: dependencyMap.get(nodeId) || [],
        dependents: dependentMap.get(nodeId) || [],
        level: levelIndex,
      });

      totalWeight += weight;
    });
  });

  return {
    nodes: graphNodes,
    totalWeight,
    criticalPath,
    criticalPathWeight,
    maxParallelism,
    executionOrder,
  };
}
```

**Key Discovery:** The graph analyzer:
1. Takes a flow with child nodes
2. Flattens them into a single Map<string, GraphNode>
3. Calculates total weight as the sum of all child weights
4. Does NOT include the parent group/loop in the node map

**Answer:**
❌ **Groups do NOT calculate their own progress**
✅ **Children are flattened and tracked individually**
✅ **Overall progress = weighted sum of ALL children**

---

## 5. How Loops Handle Progress

### Loop as Leaf Node

**File:** `/packages/@atomiton/nodes/src/executables/loop/index.ts`

Loops are implemented as **leaf node executables**, meaning:
1. They are registered as executable node types
2. They execute their iterations internally
3. They do NOT expose child nodes to the graph analyzer
4. They count as ONE node in the execution graph

**Loop Execution Flow:**
```
1. Graph analyzer sees "loop" node
2. Loop is added to graph with type="loop", weight=100 (default)
3. During execution, loop runs iterations internally
4. Loop can call setNodeProgress() to report partial progress
5. Loop completes and reports 100%
```

**Critical Difference from Groups:**
```typescript
// GROUP: Children are in node.nodes[] and get analyzed
{
  type: "group",
  nodes: [
    { id: "child1", type: "file-write" },  // Analyzed and added to graph
    { id: "child2", type: "template" },    // Analyzed and added to graph
  ]
}

// LOOP: Children are in node.nodes[] but NOT analyzed
{
  type: "loop",
  nodes: [
    { id: "child1", type: "file-write" },  // NOT analyzed (executed internally)
    { id: "child2", type: "template" },    // NOT analyzed (executed internally)
  ]
}
```

The difference is in the **executor**:
- Groups call `executeGraph()` recursively, which analyzes children
- Loops call their internal executor, which does NOT analyze children

**Answer:**
❌ **Loops do NOT calculate progress from iterations**
✅ **Loops report progress as a SINGLE unit (0% → 100%)**
✅ **Loop iterations are hidden from the graph analyzer**

---

## 6. Implementation Pattern for Bento

### Recommended Approach: Flat Graph with Weighted Progress

```go
// 1. Graph Analysis (one-time, at initialization)
type graphNode struct {
    ID           string
    Type         string
    Weight       int
    Dependencies []string
    Level        int
}

type executionGraph struct {
    Nodes       map[string]*graphNode
    TotalWeight int
    mu          sync.RWMutex
}

func analyzeGraph(def *neta.Definition) *executionGraph {
    graph := &executionGraph{
        Nodes: make(map[string]*graphNode),
    }

    // Flatten all nodes into the graph
    analyzeNode(def, graph, 0)

    return graph
}

func analyzeNode(def *neta.Definition, graph *executionGraph, level int) {
    switch def.Type {
    case "group", "parallel":
        // Groups: analyze children, don't add group itself
        for _, child := range def.Nodes {
            analyzeNode(&child, graph, level)
        }

    case "loop":
        // Loops: add loop as single node, don't analyze children
        weight := getNodeWeight(def.Type)
        graph.Nodes[def.ID] = &graphNode{
            ID:     def.ID,
            Type:   def.Type,
            Weight: weight,
            Level:  level,
        }
        graph.TotalWeight += weight

    default:
        // Leaf nodes: add to graph
        weight := getNodeWeight(def.Type)
        graph.Nodes[def.ID] = &graphNode{
            ID:     def.ID,
            Type:   def.Type,
            Weight: weight,
            Level:  level,
        }
        graph.TotalWeight += weight
    }
}

func getNodeWeight(nodeType string) int {
    weights := map[string]int{
        "shell-command": 300,
        "file-write":    50,
        "template":      100,
        "loop":          100,
    }
    if w, ok := weights[nodeType]; ok {
        return w
    }
    return 100
}
```

```go
// 2. Execution State (mutable, tracks progress)
type nodeState struct {
    State    string  // "pending" | "executing" | "completed" | "error"
    Progress int     // 0-100
    Message  string
}

type executionState struct {
    Graph          *executionGraph
    Nodes          map[string]*nodeState
    CachedProgress int
    mu             sync.RWMutex
}

func (s *executionState) setNodeProgress(nodeID string, progress int, message string) {
    s.mu.Lock()
    defer s.mu.Unlock()

    node, ok := s.Nodes[nodeID]
    if !ok {
        return
    }

    // Clamp progress
    if progress < 0 {
        progress = 0
    }
    if progress > 100 {
        progress = 100
    }

    node.Progress = progress
    node.Message = message

    // Recalculate overall progress
    s.CachedProgress = s.calculateProgress()
}

func (s *executionState) calculateProgress() int {
    if s.Graph.TotalWeight == 0 {
        return 0
    }

    completedWeight := 0

    for nodeID, graphNode := range s.Graph.Nodes {
        state := s.Nodes[nodeID]

        switch state.State {
        case "completed", "skipped":
            completedWeight += graphNode.Weight

        case "executing":
            // Partial progress
            completedWeight += (graphNode.Weight * state.Progress) / 100
        }
    }

    return (completedWeight * 100) / s.Graph.TotalWeight
}

func (s *executionState) getProgress() int {
    s.mu.RLock()
    defer s.mu.RUnlock()
    return s.CachedProgress
}
```

```go
// 3. Loop Execution (reports partial progress)
func (i *Itamae) executeForEachLoop(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Set loop to executing
    i.state.setNodeState(def.ID, "executing")
    i.state.setNodeProgress(def.ID, 0, "Starting loop")

    array := execCtx.resolveValue(def.Parameters["array"]).([]interface{})
    loopResults := make([]interface{}, 0, len(array))

    for idx, item := range array {
        // Update partial progress
        progress := (idx * 100) / len(array)
        i.state.setNodeProgress(
            def.ID,
            progress,
            fmt.Sprintf("Iteration %d/%d", idx+1, len(array)),
        )

        // Execute child nodes internally (not tracked in graph)
        loopCtx := execCtx.createChild()
        loopCtx.set("item", item)
        loopCtx.set("index", idx)

        iterationResult := make(map[string]interface{})
        for _, child := range def.Nodes {
            // Execute WITHOUT tracking (internal to loop)
            output, err := i.executeNodeInternal(ctx, &child, loopCtx)
            if err != nil {
                i.state.setNodeState(def.ID, "error")
                return err
            }
            iterationResult[child.ID] = output
        }

        loopResults = append(loopResults, iterationResult)
    }

    // Complete loop
    i.state.setNodeProgress(def.ID, 100, "Completed")
    i.state.setNodeState(def.ID, "completed")

    result.NodeOutputs[def.ID] = loopResults
    return nil
}
```

### Key Differences from Current Bento Implementation

| Aspect | Current Bento | Recommended (Atomiton Style) |
|--------|---------------|------------------------------|
| **Loop children** | Counted in graph | NOT counted in graph |
| **Progress calculation** | Simple count | Weighted sum |
| **Partial progress** | No | Yes (via setNodeProgress) |
| **Loop reporting** | Children report individually | Loop reports as single unit |
| **Graph structure** | Hierarchical (recursive) | Flat (all nodes at same level) |

---

## 7. Answering the Original Questions

### Q1: How does each node (especially groups/loops) track its own progress?

**Answer:**
- ✅ Each node has a `progress` field (0-100)
- ✅ Groups do NOT have progress (they don't exist in the graph)
- ✅ Loops have progress (they count as 1 node)
- ✅ Leaf nodes have progress

### Q2: Is there a progress field on each node instance?

**Answer:**
- ✅ YES - `node.progress` exists on `ExecutionGraphNode`
- ✅ It's stored in the execution graph store
- ✅ It's updated via `setNodeProgress(nodeId, progress, message)`

### Q3: How is progress calculated for container nodes vs leaf nodes?

**Answer:**
- ❌ Container nodes (groups) do NOT have progress
- ✅ Leaf nodes have progress (0-100)
- ✅ Loops are treated as leaf nodes (count as 1 node)
- ✅ Overall progress = weighted sum of ALL nodes

### Q4: How does a group calculate its progress from children?

**Answer:**
- ❌ Groups do NOT calculate progress
- ✅ Children are flattened into the graph
- ✅ Overall graph progress includes all children
- ❌ The group itself doesn't exist in the execution graph

### Q5: How do loops calculate progress from iterations?

**Answer:**
- ❌ Loops do NOT calculate progress from iterations
- ✅ Loops report progress as a single unit (0% → 100%)
- ✅ Loop children are executed internally (not tracked)
- ✅ Loops can call `setNodeProgress()` to report partial progress

### Q6: Do onNodeProgress events include both node-level and graph-level progress?

**Answer:**
- ✅ YES - Events include the full `ExecutionGraphState`
- ✅ `state.cachedProgress` = overall graph progress
- ✅ `state.nodes.get(nodeId).progress` = individual node progress
- ✅ Subscribers receive the entire state on every update

### Q7: What data is sent in the progress event?

**Answer:**
```typescript
{
  nodes: Map<string, ExecutionGraphNode>,  // All nodes with individual progress
  cachedProgress: number,                  // Overall graph progress (0-100)
  totalWeight: number,
  isExecuting: boolean,
  startTime: number,
  endTime: number,
  // ... other metadata
}
```

---

## 8. Key Takeaways for Bento

1. **Flatten the graph** - Don't track hierarchy, flatten all leaf nodes into a single map
2. **Loops as leaf nodes** - Don't expose loop children to the graph analyzer
3. **Weight-based progress** - Use node type weights for more accurate progress
4. **Partial progress** - Allow nodes to report progress during execution
5. **Cache progress** - Only recalculate on state mutations (O(1) reads)
6. **Event throttling** - Prevent flooding with high-frequency updates
7. **Single source of truth** - The execution graph store is the only state

**The "hierarchy" is an illusion** - it's actually a flat structure with weighted summation.

---

## Files Referenced

```
/packages/@atomiton/conductor/src/execution/types.ts
/packages/@atomiton/conductor/src/execution/executionGraphStore.ts
/packages/@atomiton/conductor/src/execution/ProgressCalculator.ts
/packages/@atomiton/conductor/src/execution/executeGraph.ts
/packages/@atomiton/conductor/src/events/ConductorEventEmitter.ts
/packages/@atomiton/nodes/src/graph/graphAnalyzer.ts
/packages/@atomiton/nodes/src/executables/loop/index.ts
```

---

## IMPLEMENTATION PROMPT FOR COLOSSUS

**Context:** You are Colossus, the Go standards guardian who prevents reinventing wheels, ensures idiomatic Go, and maintains clean module boundaries. You have been given this analysis document that shows how Atomiton (our TypeScript predecessor) implements accurate progress tracking for graph execution.

**Goal:** Implement accurate progress tracking in Bento's itamae executor that will provide precise progress reporting in both logs and the TUI progress bar. The progress tracking must handle loops correctly and follow Go idioms.

### Current Problem

**File:** `pkg/itamae/itamae.go:117-119, 123-127`

```go
// Current implementation (INCORRECT)
i.totalNodes = countExecutableNodes(def)  // Counts leaf nodes only
i.completedNodes = 0

// Later...
i.completedNodes++
progressPct := (i.completedNodes * 100) / i.totalNodes
```

**Issues:**
1. **Loop counting bug**: Loops execute children N times, but `countExecutableNodes()` only counts definition structure
   - Example: Loop with 2 child nodes × 3 iterations = 6 executions, but counted as 2
   - Result: Progress shows 50%, 100%, 150%, 200%, 250%, 300% (capped at 100%)
2. **No partial progress**: Nodes can't report progress during execution (e.g., "Iteration 3/10")
3. **No weights**: All nodes count equally (fast file-write = slow shell-command)
4. **Hierarchical structure**: We track execution hierarchically instead of flattening the graph

### Solution from Atomiton Analysis

Atomiton solves this with:

1. **Flat graph structure**: All executable nodes (including loops as single units) in one map
2. **Loops as leaf nodes**: Loop children are NOT tracked in the graph; loops report internal progress
3. **Weight-based calculation**: `progress = (completedWeight / totalWeight) * 100`
4. **Partial progress support**: Nodes report 0-100% during execution via `setNodeProgress()`

### Implementation Requirements

#### 1. Graph Analysis (One-Time, at Initialization)

Create a new file `pkg/itamae/graph.go`:

```go
package itamae

import "sync"

// graphNode represents a node in the flattened execution graph
type graphNode struct {
    ID     string
    Type   string
    Weight int
    Level  int
}

// executionGraph is the analyzed graph structure (immutable after creation)
type executionGraph struct {
    Nodes       map[string]*graphNode
    TotalWeight int
}

// analyzeGraph flattens the bento definition into an execution graph
// Groups are transparent (children are flattened)
// Loops are opaque (count as single node, children hidden)
func analyzeGraph(def *neta.Definition) *executionGraph {
    graph := &executionGraph{
        Nodes: make(map[string]*graphNode),
    }
    analyzeNode(def, graph, 0)
    return graph
}

func analyzeNode(def *neta.Definition, graph *executionGraph, level int) {
    switch def.Type {
    case "group", "parallel":
        // Groups: flatten children, don't add group itself
        for i := range def.Nodes {
            analyzeNode(&def.Nodes[i], graph, level)
        }

    case "loop":
        // Loops: add as single node (children execute internally)
        weight := getNodeWeight(def.Type)
        graph.Nodes[def.ID] = &graphNode{
            ID:     def.ID,
            Type:   def.Type,
            Weight: weight,
            Level:  level,
        }
        graph.TotalWeight += weight

    default:
        // Leaf nodes: add to graph
        weight := getNodeWeight(def.Type)
        graph.Nodes[def.ID] = &graphNode{
            ID:     def.ID,
            Type:   def.Type,
            Weight: weight,
            Level:  level,
        }
        graph.TotalWeight += weight
    }
}

func getNodeWeight(nodeType string) int {
    // Based on Atomiton's DEFAULT_NODE_WEIGHTS
    weights := map[string]int{
        "shell-command": 300, // Slow operations
        "http-request":  200,
        "image":         150,
        "spreadsheet":   100,
        "loop":          100, // Loops count as medium weight
        "file-system":   50,  // Fast operations
        "edit-fields":   50,
        "transform":     50,
    }
    if w, ok := weights[nodeType]; ok {
        return w
    }
    return 100 // Default weight
}
```

#### 2. Execution State Tracking

Modify `pkg/itamae/itamae.go`:

```go
// nodeState tracks runtime state for a single node
type nodeState struct {
    State    string // "pending" | "executing" | "completed" | "error"
    Progress int    // 0-100
    Message  string
}

// executionState tracks runtime progress (separate from graph analysis)
type executionState struct {
    graph          *executionGraph
    nodes          map[string]*nodeState
    cachedProgress int
    mu             sync.RWMutex
}

// Itamae orchestrates bento execution
type Itamae struct {
    pantry      *pantry.Pantry
    logger      *shoyu.Logger
    messenger   ProgressMessenger
    onProgress  ProgressCallback
    slowMoDelay time.Duration
    state       *executionState // NEW: replaces totalNodes/completedNodes
}

// setNodeProgress updates a node's progress and recalculates overall progress
func (s *executionState) setNodeProgress(nodeID string, progress int, message string) {
    s.mu.Lock()
    defer s.mu.Unlock()

    node, ok := s.nodes[nodeID]
    if !ok {
        return
    }

    // Clamp progress 0-100
    if progress < 0 {
        progress = 0
    }
    if progress > 100 {
        progress = 100
    }

    node.Progress = progress
    node.Message = message

    // Recalculate overall progress
    s.cachedProgress = s.calculateProgress()
}

// calculateProgress computes overall graph progress using weighted sum
func (s *executionState) calculateProgress() int {
    if s.graph.TotalWeight == 0 {
        return 0
    }

    completedWeight := 0

    for nodeID, graphNode := range s.graph.Nodes {
        state := s.nodes[nodeID]

        switch state.State {
        case "completed":
            completedWeight += graphNode.Weight

        case "executing":
            // Include partial progress from executing nodes
            completedWeight += (graphNode.Weight * state.Progress) / 100
        }
        // pending and error nodes contribute 0
    }

    return (completedWeight * 100) / s.graph.TotalWeight
}

// getProgress returns the cached overall progress (thread-safe)
func (s *executionState) getProgress() int {
    s.mu.RLock()
    defer s.mu.RUnlock()
    return s.cachedProgress
}

// setNodeState updates a node's execution state
func (s *executionState) setNodeState(nodeID string, state string) {
    s.mu.Lock()
    defer s.mu.Unlock()

    node, ok := s.nodes[nodeID]
    if !ok {
        return
    }

    node.State = state

    // Recalculate overall progress
    s.cachedProgress = s.calculateProgress()
}
```

#### 3. Initialize State in Serve()

Modify `pkg/itamae/itamae.go` in the `Serve()` method:

```go
func (i *Itamae) Serve(ctx context.Context, def *neta.Definition) (*Result, error) {
    start := time.Now()

    // Analyze graph structure (one-time)
    graph := analyzeGraph(def)

    // Initialize execution state
    i.state = &executionState{
        graph: graph,
        nodes: make(map[string]*nodeState),
    }

    // Initialize all nodes to pending
    for nodeID := range graph.Nodes {
        i.state.nodes[nodeID] = &nodeState{
            State:    "pending",
            Progress: 0,
        }
    }

    if i.logger != nil {
        msg := msgBentoStarted(def.Name)
        i.logger.Info(msg.format())
    }

    // ... rest of Serve()
}
```

#### 4. Refactor Loop Execution (CRITICAL)

**Current:** Loops execute children and each child reports completion → inflates progress

**New:** Loops execute as leaf nodes and report internal iteration progress

Modify `pkg/itamae/loop_foreach.go`:

```go
// executeForEach executes a forEach loop AS A LEAF NODE
func (i *Itamae) executeForEach(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    items, err := i.extractLoopItems(def, execCtx)
    if err != nil {
        return err
    }

    // Mark loop as executing
    i.state.setNodeState(def.ID, "executing")
    i.state.setNodeProgress(def.ID, 0, "Starting loop")

    if i.logger != nil {
        msg := msgLoopStarted(execCtx.depth, def.Name)
        i.logger.Info(msg.format())
    }

    start := time.Now()
    loopResults := make([]interface{}, 0, len(items))

    for idx, item := range items {
        // Report partial progress for this iteration
        progress := (idx * 100) / len(items)
        message := fmt.Sprintf("Iteration %d/%d", idx+1, len(items))
        i.state.setNodeProgress(def.ID, progress, message)

        // Execute iteration INTERNALLY (children not tracked in graph)
        iterResult, err := i.executeLoopIteration(ctx, def, item, idx, execCtx)
        if err != nil {
            i.state.setNodeState(def.ID, "error")
            return err
        }

        loopResults = append(loopResults, iterResult)
    }

    duration := time.Since(start)

    // Mark loop complete
    i.state.setNodeProgress(def.ID, 100, "Completed")
    i.state.setNodeState(def.ID, "completed")

    // Store result
    result.NodeOutputs[def.ID] = loopResults
    result.NodesExecuted++

    if i.logger != nil {
        durationStr := formatDuration(duration)
        // Get final progress
        progressPct := i.state.getProgress()
        msg := msgLoopCompleted(execCtx.depth, def.Name, durationStr, progressPct)
        i.logger.Info(msg.format())
    }

    return nil
}

// executeLoopIteration executes one iteration (INTERNAL - not tracked)
func (i *Itamae) executeLoopIteration(
    ctx context.Context,
    def *neta.Definition,
    item interface{},
    idx int,
    execCtx *executionContext,
) (map[string]interface{}, error) {
    iterCtx := execCtx.withDepth(1)
    iterCtx.set("item", item)
    iterCtx.set("index", idx)

    iterResult := make(map[string]interface{})

    for j := range def.Nodes {
        childDef := &def.Nodes[j]

        // Execute child WITHOUT state tracking (internal to loop)
        output, err := i.executeNodeInternal(ctx, childDef, iterCtx)
        if err != nil {
            return nil, fmt.Errorf("iteration %d failed: %w", idx, err)
        }

        iterResult[childDef.ID] = output
    }

    return iterResult, nil
}

// executeNodeInternal executes a node without state tracking (for loop children)
func (i *Itamae) executeNodeInternal(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
) (interface{}, error) {
    // Get neta implementation
    netaImpl, err := i.pantry.GetNew(def.Type)
    if err != nil {
        return nil, err
    }

    // Prepare parameters
    params := make(map[string]interface{})
    for k, v := range def.Parameters {
        params[k] = execCtx.resolveValue(v)
    }
    params["_context"] = execCtx.toMap()

    // Execute
    output, err := netaImpl.Execute(ctx, params)
    if err != nil {
        return nil, err
    }

    // Store in context
    execCtx.set(def.ID, output)

    return output, nil
}
```

Do the same for `pkg/itamae/loop_times.go`.

#### 5. Update Regular Node Execution

Modify `pkg/itamae/executor.go`:

```go
func (i *Itamae) executeSingle(
    ctx context.Context,
    def *neta.Definition,
    execCtx *executionContext,
    result *Result,
) error {
    // Mark as executing
    i.state.setNodeState(def.ID, "executing")
    i.state.setNodeProgress(def.ID, 0, "Starting")

    // ... existing execution logic ...

    duration := time.Since(start)

    if err != nil {
        i.state.setNodeState(def.ID, "error")
        return newNodeError(def.ID, def.Type, "execute", err)
    }

    // Mark as completed
    i.state.setNodeProgress(def.ID, 100, "Completed")
    i.state.setNodeState(def.ID, "completed")

    // Store output
    execCtx.set(def.ID, output)
    result.NodeOutputs[def.ID] = output
    result.NodesExecuted++

    // Log with overall progress
    if i.logger != nil {
        if execCtx.depth > 0 {
            durationStr := formatDuration(duration)
            progressPct := i.state.getProgress()
            msg := msgChildNodeCompleted(execCtx.depth, def.Type, def.Name, durationStr, progressPct)
            i.logger.Info(msg.format())
        }
    }

    return nil
}
```

#### 6. Update Log Messages

Modify `pkg/itamae/messages.go` to add progress to loop messages:

```go
// msgLoopCompleted creates a message for loop execution completion.
// Format: " 75% │  │  └─ Perfected NETA:loop name … (2ms)"
func msgLoopCompleted(depth int, name, duration string, progressPct int) logMessage {
    indent := getIndent(depth)
    statusWord := getStatusWord(name, false)
    pctPrefix := formatProgressPrefix(progressPct)
    return logMessage{
        emoji: "",
        text:  pctPrefix + indent + "  └─ " + statusWord + " NETA:loop " + name + " … (" + duration + ")",
    }
}
```

#### 7. Update TUI Progress Bar

Modify `pkg/miso/executor.go` to use the new progress calculation:

```go
// updateProgress gets progress from itamae state
func (e *Executor) updateProgress() {
    // This is now calculated by itamae's executionState
    // The messenger will receive updates via SendNodeCompleted
    // which can include the overall progress percentage
}
```

Modify `pkg/itamae/executor.go` to send progress in messenger events:

```go
// Send messenger event with overall progress
if i.messenger != nil {
    overallProgress := i.state.getProgress()
    i.messenger.SendNodeCompleted(def.ID, duration, err, overallProgress)
}
```

### Testing Requirements

Create `pkg/itamae/graph_test.go`:

```go
func TestAnalyzeGraph_FlattenGroups(t *testing.T) {
    def := &neta.Definition{
        Type: "group",
        Nodes: []neta.Definition{
            {ID: "node1", Type: "file-system"},
            {ID: "node2", Type: "shell-command"},
        },
    }

    graph := analyzeGraph(def)

    // Groups should be transparent
    assert.Len(t, graph.Nodes, 2)
    assert.Contains(t, graph.Nodes, "node1")
    assert.Contains(t, graph.Nodes, "node2")
    assert.Equal(t, 350, graph.TotalWeight) // 50 + 300
}

func TestAnalyzeGraph_LoopAsLeaf(t *testing.T) {
    def := &neta.Definition{
        Type: "group",
        Nodes: []neta.Definition{
            {ID: "node1", Type: "file-system"},
            {
                ID:   "loop1",
                Type: "loop",
                Nodes: []neta.Definition{
                    {ID: "child1", Type: "file-system"},
                    {ID: "child2", Type: "file-system"},
                },
            },
        },
    }

    graph := analyzeGraph(def)

    // Loop children should NOT be in graph
    assert.Len(t, graph.Nodes, 2) // node1 + loop1 only
    assert.Contains(t, graph.Nodes, "node1")
    assert.Contains(t, graph.Nodes, "loop1")
    assert.NotContains(t, graph.Nodes, "child1")
    assert.NotContains(t, graph.Nodes, "child2")
    assert.Equal(t, 150, graph.TotalWeight) // 50 + 100
}

func TestProgressCalculation_PartialProgress(t *testing.T) {
    graph := &executionGraph{
        Nodes: map[string]*graphNode{
            "node1": {ID: "node1", Weight: 100},
            "node2": {ID: "node2", Weight: 200},
            "node3": {ID: "node3", Weight: 100},
        },
        TotalWeight: 400,
    }

    state := &executionState{
        graph: graph,
        nodes: map[string]*nodeState{
            "node1": {State: "completed", Progress: 100},
            "node2": {State: "executing", Progress: 50},  // 50% of 200 = 100
            "node3": {State: "pending", Progress: 0},
        },
    }

    progress := state.calculateProgress()

    // Expected: (100 + 100 + 0) / 400 * 100 = 50%
    assert.Equal(t, 50, progress)
}
```

### Go Idiom Compliance Checklist

- [ ] Use `sync.RWMutex` for thread-safe state access
- [ ] Cache expensive calculations (`cachedProgress`)
- [ ] Separate concerns (graph analysis vs execution state)
- [ ] Immutable graph structure after initialization
- [ ] Clear ownership (state belongs to Itamae, passed by pointer)
- [ ] Standard library only (no external dependencies for core logic)
- [ ] Idiomatic error handling (wrap with context)
- [ ] Table-driven tests

### Success Criteria

1. **Accurate progress**: Loop with 2 children × 3 iterations shows smooth 0% → 100% (not 0% → 300%)
2. **Weighted progress**: Shell commands contribute more to progress than file operations
3. **Partial progress**: Loops can report "Iteration 3/10" in logs
4. **Thread-safe**: Progress can be queried from TUI goroutine while execution runs
5. **Performance**: O(1) progress reads (cached), O(n) progress calculations (only on mutations)
6. **Log alignment**: Progress percentages align correctly in logs

### Files to Modify

```
pkg/itamae/graph.go              # NEW - graph analysis
pkg/itamae/itamae.go             # Modify - add executionState
pkg/itamae/executor.go           # Modify - use state.setNodeProgress()
pkg/itamae/loop_foreach.go       # Refactor - execute as leaf
pkg/itamae/loop_times.go         # Refactor - execute as leaf
pkg/itamae/messages.go           # Modify - add progress to loop messages
pkg/itamae/graph_test.go         # NEW - test graph analysis
pkg/miso/executor.go             # Modify - receive progress from messenger
```

### Implementation Order

1. Create `graph.go` with analysis functions + tests
2. Add `executionState` to `itamae.go`
3. Update `executeSingle()` to use state tracking
4. Refactor loop execution to be leaf-based
5. Update log messages with progress
6. Update TUI to use new progress
7. Add comprehensive tests
8. Verify with example bentos

### Anti-Patterns to Avoid

❌ **Don't** add hierarchy to graph (keep it flat)
❌ **Don't** track loop children in the graph
❌ **Don't** use global state (keep state in Itamae)
❌ **Don't** calculate progress on every read (cache it)
❌ **Don't** forget mutex locks (state is shared)
❌ **Don't** break existing behavior (ensure backward compatibility)

---

**End of Implementation Prompt**
