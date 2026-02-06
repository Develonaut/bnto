# Phase 2: Core Features
**Bento Desktop - Wails Implementation**

**Duration:** 3-4 weeks
**Goal:** Build production-ready workflow execution and monitoring UI

**Prerequisites:** Phase 1 POC completed and approved

---

## Objectives

1. Workflow file browser with favorites
2. Workflow execution with real-time progress
3. Log viewer with filtering and search
4. Node library browser with documentation
5. Basic error handling and recovery

---

## Task Breakdown

### Task 1: Workflow File Browser

**Objective:** Browse, search, and manage workflow files

**Steps:**

1. Create WorkflowBrowser component
```typescript
// frontend/src/components/WorkflowBrowser/WorkflowBrowser.tsx
import { useState, useEffect } from 'react'
import { ListWorkflows, GetWorkflowInfo } from '../../../wailsjs/go/main/App'

interface WorkflowEntry {
    path: string
    name: string
    description: string
    lastModified: string
}

export function WorkflowBrowser({ onSelect }: { onSelect: (path: string) => void }) {
    const [workflows, setWorkflows] = useState<WorkflowEntry[]>([])
    const [loading, setLoading] = useState(false)
    const [searchQuery, setSearchQuery] = useState('')

    const loadWorkflows = async () => {
        setLoading(true)
        try {
            const entries = await ListWorkflows()
            setWorkflows(entries)
        } catch (err) {
            console.error('Failed to load workflows:', err)
        } finally {
            setLoading(false)
        }
    }

    useEffect(() => {
        loadWorkflows()
    }, [])

    const filteredWorkflows = workflows.filter(w =>
        w.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        w.description.toLowerCase().includes(searchQuery.toLowerCase())
    )

    return (
        <div className="workflow-browser">
            <div className="browser-header">
                <input
                    type="text"
                    placeholder="Search workflows..."
                    value={searchQuery}
                    onChange={e => setSearchQuery(e.target.value)}
                />
                <button onClick={loadWorkflows}>Refresh</button>
            </div>

            {loading ? (
                <div>Loading workflows...</div>
            ) : (
                <div className="workflow-list">
                    {filteredWorkflows.map(workflow => (
                        <WorkflowCard
                            key={workflow.path}
                            workflow={workflow}
                            onSelect={() => onSelect(workflow.path)}
                        />
                    ))}
                </div>
            )}
        </div>
    )
}

function WorkflowCard({ workflow, onSelect }: {
    workflow: WorkflowEntry
    onSelect: () => void
}) {
    return (
        <div className="workflow-card" onClick={onSelect}>
            <h3>{workflow.name}</h3>
            <p>{workflow.description}</p>
            <small>Modified: {workflow.lastModified}</small>
        </div>
    )
}
```

2. Add Go methods for workflow listing
```go
// cmd/bento-desktop/app.go

type WorkflowEntry struct {
    Path         string `json:"path"`
    Name         string `json:"name"`
    Description  string `json:"description"`
    LastModified string `json:"lastModified"`
}

func (a *App) ListWorkflows() ([]WorkflowEntry, error) {
    // Get workflows from configured directories
    dirs := a.getWorkflowDirectories()

    var entries []WorkflowEntry
    for _, dir := range dirs {
        files, err := filepath.Glob(filepath.Join(dir, "*.bento.json"))
        if err != nil {
            continue
        }

        for _, file := range files {
            def, err := hangiri.Load(file)
            if err != nil {
                continue
            }

            info, _ := os.Stat(file)
            entries = append(entries, WorkflowEntry{
                Path:         file,
                Name:         def.Name,
                Description:  def.Description,
                LastModified: info.ModTime().Format(time.RFC3339),
            })
        }
    }

    return entries, nil
}

func (a *App) getWorkflowDirectories() []string {
    // TODO: Get from settings in Phase 4
    return []string{
        "/Users/Ryan/Code/bento/examples",
        filepath.Join(os.Getenv("HOME"), "Bento", "Workflows"),
    }
}

func (a *App) GetWorkflowInfo(path string) (*neta.Definition, error) {
    return hangiri.Load(path)
}
```

3. Add favorites functionality
```go
// cmd/bento-desktop/app.go

func (a *App) AddFavorite(path string) error {
    // TODO: Persist to settings
    return nil
}

func (a *App) RemoveFavorite(path string) error {
    return nil
}

func (a *App) ListFavorites() ([]string, error) {
    return []string{}, nil
}
```

**Acceptance Criteria:**
- [ ] Can list workflows from configured directories
- [ ] Search filters workflows by name/description
- [ ] Can click workflow to select it
- [ ] Can refresh workflow list
- [ ] Can add/remove favorites (placeholder for Phase 4)
- [ ] Shows workflow metadata (name, description, last modified)

**Files Created:**
- `frontend/src/components/WorkflowBrowser/WorkflowBrowser.tsx`
- `frontend/src/components/WorkflowBrowser/WorkflowCard.tsx`
- `frontend/src/components/WorkflowBrowser/WorkflowBrowser.css`

**Files Modified:**
- `cmd/bento-desktop/app.go`

---

### Task 2: Workflow Execution Engine Integration

**Objective:** Execute workflows and capture results/progress

**Steps:**

1. Create execution manager
```go
// internal/desktop/execution/manager.go
package execution

import (
    "context"
    "sync"
    "github.com/yourusername/bento/pkg/itamae"
    "github.com/yourusername/bento/pkg/neta"
)

type Manager struct {
    mu         sync.RWMutex
    executions map[string]*Execution
}

type Execution struct {
    ID       string
    Status   string  // "running", "completed", "failed"
    Progress float64 // 0.0 to 1.0
    Error    string
    Result   interface{}
}

func NewManager() *Manager {
    return &Manager{
        executions: make(map[string]*Execution),
    }
}

func (m *Manager) Start(ctx context.Context, def *neta.Definition) (string, error) {
    executionID := generateID()

    exec := &Execution{
        ID:       executionID,
        Status:   "running",
        Progress: 0.0,
    }

    m.mu.Lock()
    m.executions[executionID] = exec
    m.mu.Unlock()

    // Run in goroutine
    go m.execute(ctx, executionID, def)

    return executionID, nil
}

func (m *Manager) execute(ctx context.Context, id string, def *neta.Definition) {
    result, err := itamae.Execute(ctx, def)

    m.mu.Lock()
    defer m.mu.Unlock()

    exec := m.executions[id]
    if err != nil {
        exec.Status = "failed"
        exec.Error = err.Error()
    } else {
        exec.Status = "completed"
        exec.Result = result
    }
    exec.Progress = 1.0
}

func (m *Manager) GetStatus(id string) (*Execution, error) {
    m.mu.RLock()
    defer m.mu.RUnlock()

    exec, ok := m.executions[id]
    if !ok {
        return nil, fmt.Errorf("execution not found: %s", id)
    }

    return exec, nil
}

func (m *Manager) Cancel(id string) error {
    // TODO: Implement cancellation
    return nil
}

func generateID() string {
    return uuid.New().String()
}
```

2. Add execution methods to App
```go
// cmd/bento-desktop/app.go

import (
    "github.com/yourusername/bento/cmd/bento-desktop/internal/desktop/execution"
)

type App struct {
    ctx     context.Context
    execMgr *execution.Manager
}

func NewApp() *App {
    return &App{
        execMgr: execution.NewManager(),
    }
}

func (a *App) RunWorkflow(path string) (string, error) {
    def, err := hangiri.Load(path)
    if err != nil {
        return "", err
    }

    executionID, err := a.execMgr.Start(a.ctx, def)
    if err != nil {
        return "", err
    }

    // Start progress monitoring
    go a.monitorExecution(executionID)

    return executionID, nil
}

func (a *App) GetExecutionStatus(id string) (*execution.Execution, error) {
    return a.execMgr.GetStatus(id)
}

func (a *App) CancelExecution(id string) error {
    return a.execMgr.Cancel(id)
}

func (a *App) monitorExecution(id string) {
    ticker := time.NewTicker(100 * time.Millisecond)
    defer ticker.Stop()

    for range ticker.C {
        exec, err := a.execMgr.GetStatus(id)
        if err != nil {
            break
        }

        // Emit progress event
        runtime.EventsEmit(a.ctx, "execution:progress", map[string]interface{}{
            "id":       exec.ID,
            "status":   exec.Status,
            "progress": exec.Progress,
        })

        if exec.Status != "running" {
            break
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Can start workflow execution
- [ ] Returns execution ID immediately
- [ ] Execution runs in background
- [ ] Can query execution status
- [ ] Can cancel execution (stub for now)
- [ ] Emits progress events

**Files Created:**
- `internal/desktop/execution/manager.go`
- `internal/desktop/execution/manager_test.go`

**Files Modified:**
- `cmd/bento-desktop/app.go`

---

### Task 3: Execution Viewer UI

**Objective:** Display real-time execution progress

**Steps:**

1. Create ExecutionViewer component
```typescript
// frontend/src/components/ExecutionViewer/ExecutionViewer.tsx
import { useState, useEffect } from 'react'
import { RunWorkflow, GetExecutionStatus } from '../../../wailsjs/go/main/App'
import { EventsOn } from '../../../wailsjs/runtime/runtime'

interface Execution {
    id: string
    status: 'running' | 'completed' | 'failed'
    progress: number
    error?: string
}

export function ExecutionViewer({ workflowPath }: { workflowPath: string }) {
    const [execution, setExecution] = useState<Execution | null>(null)
    const [loading, setLoading] = useState(false)

    const runWorkflow = async () => {
        setLoading(true)
        try {
            const executionID = await RunWorkflow(workflowPath)
            setExecution({
                id: executionID,
                status: 'running',
                progress: 0,
            })
        } catch (err) {
            console.error('Failed to run workflow:', err)
        } finally {
            setLoading(false)
        }
    }

    useEffect(() => {
        // Listen for progress events
        const unlisten = EventsOn('execution:progress', (data: any) => {
            if (execution && data.id === execution.id) {
                setExecution(prev => ({
                    ...prev!,
                    status: data.status,
                    progress: data.progress,
                }))
            }
        })

        return () => unlisten()
    }, [execution])

    return (
        <div className="execution-viewer">
            <div className="controls">
                <button onClick={runWorkflow} disabled={loading || execution?.status === 'running'}>
                    {execution?.status === 'running' ? 'Running...' : 'Run Workflow'}
                </button>
            </div>

            {execution && (
                <div className="execution-status">
                    <h3>Execution: {execution.id}</h3>
                    <div className="status-badge">{execution.status}</div>
                    <div className="progress-bar">
                        <div
                            className="progress-fill"
                            style={{ width: `${execution.progress * 100}%` }}
                        />
                    </div>
                    <div className="progress-text">
                        {Math.round(execution.progress * 100)}%
                    </div>

                    {execution.status === 'completed' && (
                        <div className="success">Workflow completed successfully!</div>
                    )}

                    {execution.status === 'failed' && (
                        <div className="error">Error: {execution.error}</div>
                    )}
                </div>
            )}
        </div>
    )
}
```

2. Add execution history
```typescript
// frontend/src/components/ExecutionViewer/ExecutionHistory.tsx
export function ExecutionHistory({ executions }: { executions: Execution[] }) {
    return (
        <div className="execution-history">
            <h3>Recent Executions</h3>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Status</th>
                        <th>Progress</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {executions.map(exec => (
                        <tr key={exec.id}>
                            <td>{exec.id.substring(0, 8)}...</td>
                            <td>{exec.status}</td>
                            <td>{Math.round(exec.progress * 100)}%</td>
                            <td>
                                <button>View Logs</button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Can trigger workflow execution
- [ ] Shows real-time progress updates
- [ ] Shows completion/failure status
- [ ] Progress bar animates smoothly
- [ ] Execution history displays recent runs
- [ ] Can view logs for past executions

**Files Created:**
- `frontend/src/components/ExecutionViewer/ExecutionViewer.tsx`
- `frontend/src/components/ExecutionViewer/ExecutionHistory.tsx`
- `frontend/src/components/ExecutionViewer/ExecutionViewer.css`

---

### Task 4: Log Viewer with Filtering

**Objective:** Display and filter execution logs

**Steps:**

1. Capture logs during execution
```go
// internal/desktop/execution/manager.go

type LogEntry struct {
    Timestamp time.Time `json:"timestamp"`
    Level     string    `json:"level"`
    Message   string    `json:"message"`
    NodeID    string    `json:"nodeId,omitempty"`
}

type Execution struct {
    // ... existing fields
    Logs []LogEntry `json:"logs"`
}

func (m *Manager) execute(ctx context.Context, id string, def *neta.Definition) {
    // Capture logs from shoyu
    logHandler := func(entry shoyu.Entry) {
        m.addLog(id, LogEntry{
            Timestamp: entry.Timestamp,
            Level:     entry.Level,
            Message:   entry.Message,
            NodeID:    entry.NodeID,
        })
    }

    // Execute with log handler
    result, err := itamae.ExecuteWithLogger(ctx, def, logHandler)

    // ... rest of execution
}

func (m *Manager) addLog(executionID string, entry LogEntry) {
    m.mu.Lock()
    defer m.mu.Unlock()

    exec := m.executions[executionID]
    exec.Logs = append(exec.Logs, entry)
}
```

2. Add log retrieval methods
```go
// cmd/bento-desktop/app.go

func (a *App) GetExecutionLogs(id string) ([]execution.LogEntry, error) {
    exec, err := a.execMgr.GetStatus(id)
    if err != nil {
        return nil, err
    }
    return exec.Logs, nil
}
```

3. Create LogViewer component
```typescript
// frontend/src/components/LogViewer/LogViewer.tsx
import { useState, useEffect } from 'react'
import { GetExecutionLogs } from '../../../wailsjs/go/main/App'

interface LogEntry {
    timestamp: string
    level: string
    message: string
    nodeId?: string
}

export function LogViewer({ executionId }: { executionId: string }) {
    const [logs, setLogs] = useState<LogEntry[]>([])
    const [filter, setFilter] = useState({
        level: 'all',
        search: '',
    })

    useEffect(() => {
        const loadLogs = async () => {
            const entries = await GetExecutionLogs(executionId)
            setLogs(entries)
        }

        loadLogs()

        // Poll for new logs while execution is running
        const interval = setInterval(loadLogs, 1000)
        return () => clearInterval(interval)
    }, [executionId])

    const filteredLogs = logs.filter(log => {
        if (filter.level !== 'all' && log.level !== filter.level) {
            return false
        }
        if (filter.search && !log.message.toLowerCase().includes(filter.search.toLowerCase())) {
            return false
        }
        return true
    })

    return (
        <div className="log-viewer">
            <div className="log-controls">
                <select value={filter.level} onChange={e => setFilter({ ...filter, level: e.target.value })}>
                    <option value="all">All Levels</option>
                    <option value="debug">Debug</option>
                    <option value="info">Info</option>
                    <option value="warn">Warning</option>
                    <option value="error">Error</option>
                </select>

                <input
                    type="text"
                    placeholder="Search logs..."
                    value={filter.search}
                    onChange={e => setFilter({ ...filter, search: e.target.value })}
                />
            </div>

            <div className="log-entries">
                {filteredLogs.map((log, i) => (
                    <LogEntry key={i} log={log} />
                ))}
            </div>
        </div>
    )
}

function LogEntry({ log }: { log: LogEntry }) {
    return (
        <div className={`log-entry log-${log.level}`}>
            <span className="log-timestamp">{new Date(log.timestamp).toLocaleTimeString()}</span>
            <span className="log-level">{log.level.toUpperCase()}</span>
            {log.nodeId && <span className="log-node">[{log.nodeId}]</span>}
            <span className="log-message">{log.message}</span>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Displays logs from execution
- [ ] Can filter by log level
- [ ] Can search log messages
- [ ] Auto-updates during execution
- [ ] Scrolls to bottom for new logs
- [ ] Color-codes log levels

**Files Created:**
- `frontend/src/components/LogViewer/LogViewer.tsx`
- `frontend/src/components/LogViewer/LogEntry.tsx`
- `frontend/src/components/LogViewer/LogViewer.css`

**Files Modified:**
- `internal/desktop/execution/manager.go`
- `cmd/bento-desktop/app.go`

---

### Task 5: Node Library Browser

**Objective:** Browse available node types with documentation

**Steps:**

1. Add node library methods
```go
// cmd/bento-desktop/app.go

type NodeTypeInfo struct {
    Type        string            `json:"type"`
    Name        string            `json:"name"`
    Description string            `json:"description"`
    Category    string            `json:"category"`
    Parameters  []ParameterInfo   `json:"parameters"`
}

type ParameterInfo struct {
    Name        string `json:"name"`
    Type        string `json:"type"`
    Required    bool   `json:"required"`
    Description string `json:"description"`
    Default     interface{} `json:"default,omitempty"`
}

func (a *App) ListNodeTypes() ([]NodeTypeInfo, error) {
    types := pantry.ListTypes()

    var infos []NodeTypeInfo
    for _, t := range types {
        schema := pantry.GetSchema(t)
        infos = append(infos, NodeTypeInfo{
            Type:        t,
            Name:        schema.Name,
            Description: schema.Description,
            Category:    schema.Category,
            Parameters:  convertParameters(schema.Parameters),
        })
    }

    return infos, nil
}

func (a *App) GetNodeTypeSchema(nodeType string) (*NodeTypeInfo, error) {
    schema := pantry.GetSchema(nodeType)
    if schema == nil {
        return nil, fmt.Errorf("node type not found: %s", nodeType)
    }

    return &NodeTypeInfo{
        Type:        nodeType,
        Name:        schema.Name,
        Description: schema.Description,
        Category:    schema.Category,
        Parameters:  convertParameters(schema.Parameters),
    }, nil
}
```

2. Create NodeLibrary component
```typescript
// frontend/src/components/NodeLibrary/NodeLibrary.tsx
import { useState, useEffect } from 'react'
import { ListNodeTypes, GetNodeTypeSchema } from '../../../wailsjs/go/main/App'

interface NodeType {
    type: string
    name: string
    description: string
    category: string
}

export function NodeLibrary() {
    const [nodeTypes, setNodeTypes] = useState<NodeType[]>([])
    const [selectedNode, setSelectedNode] = useState<NodeType | null>(null)
    const [searchQuery, setSearchQuery] = useState('')

    useEffect(() => {
        const loadNodeTypes = async () => {
            const types = await ListNodeTypes()
            setNodeTypes(types)
        }
        loadNodeTypes()
    }, [])

    const filteredNodes = nodeTypes.filter(node =>
        node.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        node.description.toLowerCase().includes(searchQuery.toLowerCase())
    )

    const groupedNodes = filteredNodes.reduce((acc, node) => {
        if (!acc[node.category]) {
            acc[node.category] = []
        }
        acc[node.category].push(node)
        return acc
    }, {} as Record<string, NodeType[]>)

    return (
        <div className="node-library">
            <div className="library-sidebar">
                <input
                    type="text"
                    placeholder="Search nodes..."
                    value={searchQuery}
                    onChange={e => setSearchQuery(e.target.value)}
                />

                {Object.entries(groupedNodes).map(([category, nodes]) => (
                    <div key={category} className="node-category">
                        <h3>{category}</h3>
                        {nodes.map(node => (
                            <div
                                key={node.type}
                                className="node-item"
                                onClick={() => setSelectedNode(node)}
                            >
                                {node.name}
                            </div>
                        ))}
                    </div>
                ))}
            </div>

            <div className="library-details">
                {selectedNode ? (
                    <NodeDetails nodeType={selectedNode.type} />
                ) : (
                    <div>Select a node to view details</div>
                )}
            </div>
        </div>
    )
}

function NodeDetails({ nodeType }: { nodeType: string }) {
    const [schema, setSchema] = useState<any>(null)

    useEffect(() => {
        const loadSchema = async () => {
            const s = await GetNodeTypeSchema(nodeType)
            setSchema(s)
        }
        loadSchema()
    }, [nodeType])

    if (!schema) return <div>Loading...</div>

    return (
        <div className="node-details">
            <h2>{schema.name}</h2>
            <p>{schema.description}</p>

            <h3>Parameters</h3>
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Type</th>
                        <th>Required</th>
                        <th>Description</th>
                    </tr>
                </thead>
                <tbody>
                    {schema.parameters.map((param: any) => (
                        <tr key={param.name}>
                            <td>{param.name}</td>
                            <td>{param.type}</td>
                            <td>{param.required ? 'Yes' : 'No'}</td>
                            <td>{param.description}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Lists all available node types
- [ ] Groups nodes by category
- [ ] Can search nodes by name/description
- [ ] Shows detailed documentation for selected node
- [ ] Shows parameter specifications
- [ ] Updates when new node types are registered

**Files Created:**
- `frontend/src/components/NodeLibrary/NodeLibrary.tsx`
- `frontend/src/components/NodeLibrary/NodeDetails.tsx`
- `frontend/src/components/NodeLibrary/NodeLibrary.css`

**Files Modified:**
- `cmd/bento-desktop/app.go`

---

### Task 6: Main Application Layout

**Objective:** Combine all components into cohesive UI

**Steps:**

1. Create main layout
```typescript
// frontend/src/App.tsx
import { useState } from 'react'
import { WorkflowBrowser } from './components/WorkflowBrowser/WorkflowBrowser'
import { ExecutionViewer } from './components/ExecutionViewer/ExecutionViewer'
import { LogViewer } from './components/LogViewer/LogViewer'
import { NodeLibrary } from './components/NodeLibrary/NodeLibrary'

type View = 'browser' | 'execute' | 'logs' | 'library'

export default function App() {
    const [activeView, setActiveView] = useState<View>('browser')
    const [selectedWorkflow, setSelectedWorkflow] = useState<string | null>(null)
    const [executionId, setExecutionId] = useState<string | null>(null)

    return (
        <div className="app">
            <nav className="sidebar">
                <h1>Bento Desktop</h1>
                <button onClick={() => setActiveView('browser')}>Workflows</button>
                <button onClick={() => setActiveView('execute')}>Execute</button>
                <button onClick={() => setActiveView('logs')}>Logs</button>
                <button onClick={() => setActiveView('library')}>Node Library</button>
            </nav>

            <main className="content">
                {activeView === 'browser' && (
                    <WorkflowBrowser
                        onSelect={path => {
                            setSelectedWorkflow(path)
                            setActiveView('execute')
                        }}
                    />
                )}

                {activeView === 'execute' && selectedWorkflow && (
                    <ExecutionViewer
                        workflowPath={selectedWorkflow}
                        onExecutionStart={id => setExecutionId(id)}
                    />
                )}

                {activeView === 'logs' && executionId && (
                    <LogViewer executionId={executionId} />
                )}

                {activeView === 'library' && (
                    <NodeLibrary />
                )}
            </main>
        </div>
    )
}
```

2. Add global styles
```css
/* frontend/src/App.css */
.app {
    display: flex;
    height: 100vh;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

.sidebar {
    width: 200px;
    background: #2a2a2a;
    color: white;
    padding: 20px;
}

.sidebar h1 {
    font-size: 18px;
    margin-bottom: 30px;
}

.sidebar button {
    display: block;
    width: 100%;
    padding: 10px;
    margin-bottom: 10px;
    background: transparent;
    border: 1px solid #555;
    color: white;
    cursor: pointer;
}

.sidebar button:hover {
    background: #3a3a3a;
}

.content {
    flex: 1;
    padding: 20px;
    overflow: auto;
}
```

**Acceptance Criteria:**
- [ ] Sidebar navigation works
- [ ] All views render correctly
- [ ] Can navigate between views
- [ ] State persists across view changes
- [ ] Responsive layout (basic)

**Files Modified:**
- `frontend/src/App.tsx`
- `frontend/src/App.css`

---

## Deliverables

### Code Deliverables

- [ ] `frontend/src/components/WorkflowBrowser/` - File browser
- [ ] `frontend/src/components/ExecutionViewer/` - Execution UI
- [ ] `frontend/src/components/LogViewer/` - Log display
- [ ] `frontend/src/components/NodeLibrary/` - Node documentation
- [ ] `internal/desktop/execution/` - Execution manager
- [ ] Updated `cmd/bento-desktop/app.go` - All new methods

### Documentation Deliverables

- [ ] Updated `cmd/bento-desktop/ARCHITECTURE.md` - Document new features
- [ ] `cmd/bento-desktop/USER_GUIDE.md` - Basic usage guide

---

## Success Criteria

### Functional Requirements
- [ ] Can browse workflows from configured directories
- [ ] Can search and filter workflows
- [ ] Can execute workflow and see real-time progress
- [ ] Can view logs with filtering
- [ ] Can browse node library with documentation
- [ ] Navigation between views works smoothly

### Non-Functional Requirements
- [ ] UI is responsive and performant
- [ ] Real-time updates don't lag
- [ ] Memory usage stays reasonable during execution
- [ ] All TypeScript builds without errors
- [ ] All Go code builds without errors

### Bento Box Principle Compliance
- [ ] Execution logic isolated in `internal/desktop/execution/`
- [ ] UI components are single-responsibility
- [ ] No modifications to `pkg/` packages
- [ ] Clear boundaries between layers
- [ ] No utility grab bags

---

## Testing Checklist

### Manual Testing

- [ ] Browse workflows in different directories
- [ ] Search workflows by name
- [ ] Execute a simple workflow
- [ ] Execute a complex workflow
- [ ] View real-time progress updates
- [ ] View logs during execution
- [ ] Filter logs by level
- [ ] Search log messages
- [ ] Browse node library
- [ ] View node documentation
- [ ] Navigate between all views

### Integration Testing

```go
// internal/desktop/execution/manager_test.go
func TestExecutionManager(t *testing.T) {
    mgr := NewManager()

    // Load test workflow
    def, _ := hangiri.Load("testdata/simple.bento.json")

    // Start execution
    execID, err := mgr.Start(context.Background(), def)
    assert.NoError(t, err)

    // Wait for completion
    time.Sleep(1 * time.Second)

    // Check status
    exec, err := mgr.GetStatus(execID)
    assert.NoError(t, err)
    assert.Equal(t, "completed", exec.Status)
}
```

---

## Known Limitations

1. **No persistence**: Execution history lost on app restart (addressed in Phase 4)
2. **Basic UI**: Minimal styling, no theming yet
3. **No cancellation**: Cancel button doesn't actually cancel yet
4. **Polling for updates**: Should use WebSockets/events (acceptable for now)
5. **No execution inputs**: Can't provide runtime inputs yet

---

## Performance Targets

- [ ] Load 100 workflows in < 1 second
- [ ] Start execution in < 100ms
- [ ] Progress updates at 10 Hz (100ms intervals)
- [ ] Log updates at 1 Hz (1 second intervals)
- [ ] Memory usage < 100MB with 1 running workflow
- [ ] Memory usage < 200MB with 10 past executions

---

## Next Steps

After completing Phase 2:

1. **User Testing**
   - Get feedback on core workflow execution UI
   - Identify pain points in navigation
   - Gather feature requests

2. **Decision Point**
   - ✅ Proceed to Phase 3 (Visual Editor)
   - ⚠️ Iterate on Phase 2 if issues found
   - 📋 Prioritize additional features vs editor

3. **If Proceeding**
   - Review [wails-phase3-workflow-editor.md](./wails-phase3-workflow-editor.md)
   - Allocate 5-6 weeks for Phase 3
   - Research React Flow library

---

## Colossus Review Prompt

```
I've completed Phase 2 (Core Features) for Bento Desktop.

Before marking this phase complete, please:

1. Review all new code against Bento Box Principle (.claude/BENTO_BOX_PRINCIPLE.md):
   - Check internal/desktop/execution/ for single responsibility
   - Verify no utility grab bags
   - Confirm clear boundaries between layers

2. Verify Go Standards (.claude/GO_STANDARDS_REVIEW.md):
   - Check internal/desktop/execution/manager.go
   - Verify proper error handling
   - Check concurrency safety (mutex usage)
   - Verify no data races

3. Review React components:
   - Check component sizes (< 250 lines ideal)
   - Verify proper separation of concerns
   - Check for code duplication

4. Verify all Task Acceptance Criteria are met (listed in wails-phase2-core-features.md)

5. Run the code-review command: /code-review cmd/bento-desktop/ internal/desktop/

Key areas to scrutinize:
- Is execution/manager.go properly isolated and tested?
- Are React components properly structured?
- Is the Go ↔ React communication clean?
- Are there any race conditions in concurrent execution?
- Is error handling comprehensive?

Testing requirements:
- Run: go test ./internal/desktop/...
- Verify: All tests pass
- Check: Test coverage > 70%

After review, provide:
- List of issues found (if any)
- Recommendations for improvements
- Approval to proceed to Phase 3 OR items to address first

Do not approve this phase until:
1. The code-review command has been run
2. All Go tests pass
3. All critical issues are resolved
4. Manual testing checklist is completed
```

---

**Phase 2 Status:** Ready to implement
**Previous Phase:** [Phase 1: POC Setup](./wails-phase1-poc.md)
**Next Phase:** [Phase 3: Workflow Editor](./wails-phase3-workflow-editor.md)
