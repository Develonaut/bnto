# Phase 3: Visual Workflow Editor
**Bento Desktop - Wails Implementation**

**Duration:** 5-6 weeks
**Goal:** Build visual drag-and-drop workflow editor with real-time validation

**Prerequisites:** Phase 2 Core Features completed and approved

---

## Objectives

1. Visual workflow canvas using React Flow
2. Node palette with drag-and-drop
3. Property editor for node configuration
4. Real-time workflow validation
5. Save and load workflows from editor
6. Workflow testing and debugging

---

## Technology Choice: React Flow

**Why React Flow?**
- Purpose-built for node-based editors
- Excellent TypeScript support
- Performant with large graphs
- Rich ecosystem of plugins
- Good documentation

**Installation:**
```bash
cd cmd/bento-desktop/frontend
npm install reactflow
```

---

## Task Breakdown

### Task 1: React Flow Integration

**Objective:** Set up React Flow and create basic canvas

**Steps:**

1. Install dependencies
```bash
cd frontend
npm install reactflow
npm install @types/reactflow --save-dev
```

2. Create WorkflowCanvas component
```typescript
// frontend/src/components/WorkflowEditor/WorkflowCanvas.tsx
import { useCallback } from 'react'
import ReactFlow, {
    Node,
    Edge,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
    addEdge,
    Connection,
} from 'reactflow'
import 'reactflow/dist/style.css'

interface WorkflowCanvasProps {
    initialNodes?: Node[]
    initialEdges?: Edge[]
    onNodesChange?: (nodes: Node[]) => void
    onEdgesChange?: (edges: Edge[]) => void
}

export function WorkflowCanvas({
    initialNodes = [],
    initialEdges = [],
    onNodesChange,
    onEdgesChange,
}: WorkflowCanvasProps) {
    const [nodes, setNodes, onNodesChangeInternal] = useNodesState(initialNodes)
    const [edges, setEdges, onEdgesChangeInternal] = useEdgesState(initialEdges)

    const onConnect = useCallback(
        (connection: Connection) => {
            setEdges((eds) => addEdge(connection, eds))
            onEdgesChange?.(edges)
        },
        [edges, onEdgesChange]
    )

    const handleNodesChange = useCallback(
        (changes: any) => {
            onNodesChangeInternal(changes)
            onNodesChange?.(nodes)
        },
        [nodes, onNodesChange, onNodesChangeInternal]
    )

    return (
        <div className="workflow-canvas" style={{ height: '100%', width: '100%' }}>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={handleNodesChange}
                onEdgesChange={onEdgesChangeInternal}
                onConnect={onConnect}
                fitView
            >
                <Controls />
                <Background />
            </ReactFlow>
        </div>
    )
}
```

3. Create WorkflowEditor container
```typescript
// frontend/src/components/WorkflowEditor/WorkflowEditor.tsx
import { useState } from 'react'
import { Node, Edge } from 'reactflow'
import { WorkflowCanvas } from './WorkflowCanvas'
import { NodePalette } from './NodePalette'
import { PropertyPanel } from './PropertyPanel'

export function WorkflowEditor() {
    const [nodes, setNodes] = useState<Node[]>([])
    const [edges, setEdges] = useState<Edge[]>([])
    const [selectedNode, setSelectedNode] = useState<Node | null>(null)

    return (
        <div className="workflow-editor">
            <div className="editor-layout">
                <NodePalette onAddNode={(nodeType) => {
                    // TODO: Add node to canvas
                }} />

                <WorkflowCanvas
                    initialNodes={nodes}
                    initialEdges={edges}
                    onNodesChange={setNodes}
                    onEdgesChange={setEdges}
                />

                <PropertyPanel
                    node={selectedNode}
                    onUpdate={(updates) => {
                        // TODO: Update node properties
                    }}
                />
            </div>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] React Flow renders without errors
- [ ] Can drag nodes around canvas
- [ ] Can connect nodes with edges
- [ ] Can zoom and pan canvas
- [ ] Controls (zoom, fit view) work

**Files Created:**
- `frontend/src/components/WorkflowEditor/WorkflowCanvas.tsx`
- `frontend/src/components/WorkflowEditor/WorkflowEditor.tsx`
- `frontend/src/components/WorkflowEditor/WorkflowEditor.css`

---

### Task 2: Custom Node Components

**Objective:** Create custom React Flow nodes for each Bento node type

**Steps:**

1. Create base BentoNode component
```typescript
// frontend/src/components/WorkflowEditor/nodes/BentoNode.tsx
import { memo } from 'react'
import { Handle, Position, NodeProps } from 'reactflow'

export interface BentoNodeData {
    id: string
    type: string
    name: string
    parameters: Record<string, any>
    status?: 'idle' | 'running' | 'completed' | 'failed'
}

export const BentoNode = memo(({ data, selected }: NodeProps<BentoNodeData>) => {
    return (
        <div className={`bento-node ${data.type} ${selected ? 'selected' : ''} ${data.status || ''}`}>
            <Handle type="target" position={Position.Top} />

            <div className="node-header">
                <div className="node-icon">{getNodeIcon(data.type)}</div>
                <div className="node-title">{data.name || data.type}</div>
            </div>

            <div className="node-body">
                {Object.keys(data.parameters).length > 0 && (
                    <div className="node-params">
                        {Object.entries(data.parameters).slice(0, 3).map(([key, value]) => (
                            <div key={key} className="param">
                                <span className="param-key">{key}:</span>
                                <span className="param-value">{String(value).substring(0, 20)}</span>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {data.status && (
                <div className={`node-status status-${data.status}`}>
                    {getStatusIcon(data.status)}
                </div>
            )}

            <Handle type="source" position={Position.Bottom} />
        </div>
    )
})

function getNodeIcon(type: string): string {
    const icons: Record<string, string> = {
        'http-request': '🌐',
        'file-system': '📁',
        'shell-command': '⚙️',
        'edit-fields': '✏️',
        'spreadsheet': '📊',
        'image': '🖼️',
        'transform': '🔄',
        'group': '📦',
        'loop': '🔁',
        'parallel': '⚡',
    }
    return icons[type] || '📌'
}

function getStatusIcon(status: string): string {
    return {
        'running': '⏳',
        'completed': '✅',
        'failed': '❌',
    }[status] || ''
}
```

2. Register custom node types
```typescript
// frontend/src/components/WorkflowEditor/nodeTypes.ts
import { NodeTypes } from 'reactflow'
import { BentoNode } from './nodes/BentoNode'

export const nodeTypes: NodeTypes = {
    'bento-node': BentoNode,
}
```

3. Update WorkflowCanvas to use custom nodes
```typescript
// frontend/src/components/WorkflowEditor/WorkflowCanvas.tsx
import { nodeTypes } from './nodeTypes'

export function WorkflowCanvas({ ... }: WorkflowCanvasProps) {
    // ...
    return (
        <ReactFlow
            nodes={nodes}
            edges={edges}
            nodeTypes={nodeTypes}  // Add this
            // ...
        >
```

**Acceptance Criteria:**
- [ ] Custom node component renders
- [ ] Shows node type icon
- [ ] Shows node name/type
- [ ] Shows truncated parameters
- [ ] Shows status indicator
- [ ] Handles work correctly
- [ ] Selection highlights node

**Files Created:**
- `frontend/src/components/WorkflowEditor/nodes/BentoNode.tsx`
- `frontend/src/components/WorkflowEditor/nodes/BentoNode.css`
- `frontend/src/components/WorkflowEditor/nodeTypes.ts`

---

### Task 3: Node Palette (Drag & Drop)

**Objective:** Create draggable node palette

**Steps:**

1. Create NodePalette component
```typescript
// frontend/src/components/WorkflowEditor/NodePalette.tsx
import { useState, useEffect } from 'react'
import { ListNodeTypes } from '../../../wailsjs/go/main/App'

interface NodeType {
    type: string
    name: string
    description: string
    category: string
}

export function NodePalette({ onAddNode }: { onAddNode: (nodeType: string) => void }) {
    const [nodeTypes, setNodeTypes] = useState<NodeType[]>([])
    const [searchQuery, setSearchQuery] = useState('')
    const [expandedCategories, setExpandedCategories] = useState<Set<string>>(new Set())

    useEffect(() => {
        const loadNodeTypes = async () => {
            const types = await ListNodeTypes()
            setNodeTypes(types)
            // Expand all categories by default
            setExpandedCategories(new Set(types.map(t => t.category)))
        }
        loadNodeTypes()
    }, [])

    const filteredNodes = nodeTypes.filter(node =>
        node.name.toLowerCase().includes(searchQuery.toLowerCase())
    )

    const groupedNodes = filteredNodes.reduce((acc, node) => {
        if (!acc[node.category]) {
            acc[node.category] = []
        }
        acc[node.category].push(node)
        return acc
    }, {} as Record<string, NodeType[]>)

    const onDragStart = (event: React.DragEvent, nodeType: string) => {
        event.dataTransfer.setData('application/reactflow', nodeType)
        event.dataTransfer.effectAllowed = 'move'
    }

    const toggleCategory = (category: string) => {
        setExpandedCategories(prev => {
            const next = new Set(prev)
            if (next.has(category)) {
                next.delete(category)
            } else {
                next.add(category)
            }
            return next
        })
    }

    return (
        <div className="node-palette">
            <div className="palette-header">
                <h3>Node Library</h3>
                <input
                    type="text"
                    placeholder="Search nodes..."
                    value={searchQuery}
                    onChange={e => setSearchQuery(e.target.value)}
                />
            </div>

            <div className="palette-categories">
                {Object.entries(groupedNodes).map(([category, nodes]) => (
                    <div key={category} className="category">
                        <div
                            className="category-header"
                            onClick={() => toggleCategory(category)}
                        >
                            <span>{expandedCategories.has(category) ? '▼' : '▶'}</span>
                            <span>{category}</span>
                            <span className="category-count">({nodes.length})</span>
                        </div>

                        {expandedCategories.has(category) && (
                            <div className="category-nodes">
                                {nodes.map(node => (
                                    <div
                                        key={node.type}
                                        className="palette-node"
                                        draggable
                                        onDragStart={(e) => onDragStart(e, node.type)}
                                        onClick={() => onAddNode(node.type)}
                                    >
                                        <span className="node-icon">{getNodeIcon(node.type)}</span>
                                        <span className="node-name">{node.name}</span>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>
                ))}
            </div>
        </div>
    )
}
```

2. Handle drop on canvas
```typescript
// frontend/src/components/WorkflowEditor/WorkflowCanvas.tsx
import { useCallback, useRef } from 'react'

export function WorkflowCanvas({ ... }: WorkflowCanvasProps) {
    const reactFlowWrapper = useRef<HTMLDivElement>(null)
    const [reactFlowInstance, setReactFlowInstance] = useState<any>(null)

    const onDrop = useCallback(
        (event: React.DragEvent) => {
            event.preventDefault()

            const reactFlowBounds = reactFlowWrapper.current!.getBoundingClientRect()
            const nodeType = event.dataTransfer.getData('application/reactflow')

            if (!nodeType || !reactFlowInstance) return

            const position = reactFlowInstance.project({
                x: event.clientX - reactFlowBounds.left,
                y: event.clientY - reactFlowBounds.top,
            })

            const newNode: Node = {
                id: `${nodeType}-${Date.now()}`,
                type: 'bento-node',
                position,
                data: {
                    type: nodeType,
                    name: nodeType,
                    parameters: {},
                },
            }

            setNodes((nds) => nds.concat(newNode))
        },
        [reactFlowInstance, setNodes]
    )

    const onDragOver = useCallback((event: React.DragEvent) => {
        event.preventDefault()
        event.dataTransfer.dropEffect = 'move'
    }, [])

    return (
        <div
            ref={reactFlowWrapper}
            className="workflow-canvas"
            onDrop={onDrop}
            onDragOver={onDragOver}
        >
            <ReactFlow
                onInit={setReactFlowInstance}
                // ...
            />
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Node palette displays all node types
- [ ] Can search/filter nodes
- [ ] Can collapse/expand categories
- [ ] Can drag node from palette to canvas
- [ ] Can click node to add to canvas center
- [ ] New nodes appear with default properties

**Files Created:**
- `frontend/src/components/WorkflowEditor/NodePalette.tsx`
- `frontend/src/components/WorkflowEditor/NodePalette.css`

**Files Modified:**
- `frontend/src/components/WorkflowEditor/WorkflowCanvas.tsx`

---

### Task 4: Property Panel

**Objective:** Edit node properties with validation

**Steps:**

1. Create PropertyPanel component
```typescript
// frontend/src/components/WorkflowEditor/PropertyPanel.tsx
import { useState, useEffect } from 'react'
import { Node } from 'reactflow'
import { GetNodeTypeSchema } from '../../../wailsjs/go/main/App'

interface PropertyPanelProps {
    node: Node | null
    onUpdate: (nodeId: string, updates: Record<string, any>) => void
    onDelete: (nodeId: string) => void
}

export function PropertyPanel({ node, onUpdate, onDelete }: PropertyPanelProps) {
    const [schema, setSchema] = useState<any>(null)
    const [values, setValues] = useState<Record<string, any>>({})
    const [errors, setErrors] = useState<Record<string, string>>({})

    useEffect(() => {
        if (node) {
            setValues(node.data.parameters || {})
            loadSchema(node.data.type)
        }
    }, [node])

    const loadSchema = async (nodeType: string) => {
        const s = await GetNodeTypeSchema(nodeType)
        setSchema(s)
    }

    const handleChange = (paramName: string, value: any) => {
        const newValues = { ...values, [paramName]: value }
        setValues(newValues)

        // Validate
        const error = validateParameter(paramName, value, schema)
        setErrors(prev => ({
            ...prev,
            [paramName]: error || '',
        }))

        // Update node
        if (node) {
            onUpdate(node.id, { parameters: newValues })
        }
    }

    const handleNameChange = (name: string) => {
        if (node) {
            onUpdate(node.id, { name })
        }
    }

    if (!node) {
        return (
            <div className="property-panel empty">
                <p>Select a node to edit its properties</p>
            </div>
        )
    }

    return (
        <div className="property-panel">
            <div className="panel-header">
                <h3>Properties</h3>
                <button onClick={() => onDelete(node.id)} className="delete-btn">
                    Delete
                </button>
            </div>

            <div className="panel-section">
                <label>Node Name</label>
                <input
                    type="text"
                    value={node.data.name}
                    onChange={e => handleNameChange(e.target.value)}
                />
            </div>

            <div className="panel-section">
                <label>Node Type</label>
                <div className="readonly">{node.data.type}</div>
            </div>

            {schema && (
                <div className="panel-section">
                    <h4>Parameters</h4>
                    {schema.parameters.map((param: any) => (
                        <div key={param.name} className="param-field">
                            <label>
                                {param.name}
                                {param.required && <span className="required">*</span>}
                            </label>
                            <ParamInput
                                param={param}
                                value={values[param.name]}
                                onChange={(value) => handleChange(param.name, value)}
                            />
                            {errors[param.name] && (
                                <div className="error">{errors[param.name]}</div>
                            )}
                            <div className="param-description">{param.description}</div>
                        </div>
                    ))}
                </div>
            )}
        </div>
    )
}

function ParamInput({ param, value, onChange }: {
    param: any
    value: any
    onChange: (value: any) => void
}) {
    switch (param.type) {
        case 'string':
            return (
                <input
                    type="text"
                    value={value || ''}
                    onChange={e => onChange(e.target.value)}
                    placeholder={param.default}
                />
            )

        case 'number':
            return (
                <input
                    type="number"
                    value={value || ''}
                    onChange={e => onChange(parseFloat(e.target.value))}
                    placeholder={param.default}
                />
            )

        case 'boolean':
            return (
                <input
                    type="checkbox"
                    checked={value || false}
                    onChange={e => onChange(e.target.checked)}
                />
            )

        case 'select':
            return (
                <select
                    value={value || ''}
                    onChange={e => onChange(e.target.value)}
                >
                    <option value="">Select...</option>
                    {param.options?.map((opt: string) => (
                        <option key={opt} value={opt}>{opt}</option>
                    ))}
                </select>
            )

        case 'json':
            return (
                <textarea
                    value={value ? JSON.stringify(value, null, 2) : ''}
                    onChange={e => {
                        try {
                            onChange(JSON.parse(e.target.value))
                        } catch {
                            // Invalid JSON, ignore
                        }
                    }}
                    rows={5}
                />
            )

        default:
            return (
                <input
                    type="text"
                    value={value || ''}
                    onChange={e => onChange(e.target.value)}
                />
            )
    }
}

function validateParameter(name: string, value: any, schema: any): string | null {
    const param = schema?.parameters.find((p: any) => p.name === name)
    if (!param) return null

    if (param.required && !value) {
        return 'This field is required'
    }

    // Add more validation as needed

    return null
}
```

**Acceptance Criteria:**
- [ ] Property panel shows when node is selected
- [ ] Can edit node name
- [ ] Can edit all parameter types (string, number, boolean, select, json)
- [ ] Required fields are marked
- [ ] Validation errors display
- [ ] Changes update node immediately
- [ ] Can delete node

**Files Created:**
- `frontend/src/components/WorkflowEditor/PropertyPanel.tsx`
- `frontend/src/components/WorkflowEditor/PropertyPanel.css`

---

### Task 5: Workflow Serialization (Editor ↔ Bento Format)

**Objective:** Convert React Flow graph to/from Bento workflow format

**Steps:**

1. Create converter utilities
```typescript
// frontend/src/components/WorkflowEditor/converters.ts
import { Node, Edge } from 'reactflow'
import { neta } from '../../wailsjs/go/models'

export function reactFlowToBento(nodes: Node[], edges: Edge[]): neta.Definition {
    return {
        id: generateWorkflowId(),
        type: 'group',
        version: '1.0.0',
        name: 'Untitled Workflow',
        description: '',
        nodes: nodes.map(node => ({
            id: node.id,
            type: node.data.type,
            name: node.data.name,
            parameters: node.data.parameters,
            position: {
                x: node.position.x,
                y: node.position.y,
            },
        })),
        edges: edges.map(edge => ({
            id: edge.id,
            source: edge.source,
            target: edge.target,
            sourceHandle: edge.sourceHandle,
            targetHandle: edge.targetHandle,
        })),
    }
}

export function bentoToReactFlow(def: neta.Definition): {
    nodes: Node[]
    edges: Edge[]
} {
    const nodes: Node[] = (def.nodes || []).map(node => ({
        id: node.id,
        type: 'bento-node',
        position: node.position || { x: 0, y: 0 },
        data: {
            id: node.id,
            type: node.type,
            name: node.name,
            parameters: node.parameters || {},
        },
    }))

    const edges: Edge[] = (def.edges || []).map(edge => ({
        id: edge.id,
        source: edge.source,
        target: edge.target,
        sourceHandle: edge.sourceHandle,
        targetHandle: edge.targetHandle,
    }))

    return { nodes, edges }
}
```

2. Add save/load methods to App
```go
// cmd/bento-desktop/app.go

func (a *App) SaveWorkflowFromEditor(nodes []map[string]interface{}, edges []map[string]interface{}, metadata map[string]interface{}) error {
    // Convert React Flow data to Bento Definition
    def := convertToBentoDefinition(nodes, edges, metadata)

    // Save to file
    path := metadata["path"].(string)
    return hangiri.Save(path, def)
}

func (a *App) LoadWorkflowForEditor(path string) (map[string]interface{}, error) {
    def, err := hangiri.Load(path)
    if err != nil {
        return nil, err
    }

    // Convert to format suitable for React Flow
    return map[string]interface{}{
        "definition": def,
        "metadata": map[string]interface{}{
            "path": path,
        },
    }, nil
}
```

3. Integrate save/load into WorkflowEditor
```typescript
// frontend/src/components/WorkflowEditor/WorkflowEditor.tsx
import { SaveWorkflowFromEditor, LoadWorkflowForEditor } from '../../../wailsjs/go/main/App'
import { reactFlowToBento, bentoToReactFlow } from './converters'

export function WorkflowEditor({ workflowPath }: { workflowPath?: string }) {
    const [nodes, setNodes] = useState<Node[]>([])
    const [edges, setEdges] = useState<Edge[]>([])
    const [metadata, setMetadata] = useState({ name: 'Untitled', path: '' })

    useEffect(() => {
        if (workflowPath) {
            loadWorkflow(workflowPath)
        }
    }, [workflowPath])

    const loadWorkflow = async (path: string) => {
        const data = await LoadWorkflowForEditor(path)
        const { nodes: newNodes, edges: newEdges } = bentoToReactFlow(data.definition)
        setNodes(newNodes)
        setEdges(newEdges)
        setMetadata(data.metadata)
    }

    const saveWorkflow = async () => {
        const def = reactFlowToBento(nodes, edges)
        await SaveWorkflowFromEditor(nodes, edges, { ...metadata, ...def })
    }

    return (
        <div className="workflow-editor">
            <div className="editor-toolbar">
                <input
                    type="text"
                    value={metadata.name}
                    onChange={e => setMetadata({ ...metadata, name: e.target.value })}
                />
                <button onClick={saveWorkflow}>Save</button>
                <button onClick={() => onSaveAs()}>Save As...</button>
            </div>
            {/* ... rest of editor */}
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Can save workflow from editor to .bento.json file
- [ ] Can load workflow from .bento.json into editor
- [ ] Node positions are preserved
- [ ] Node parameters are preserved
- [ ] Edges are preserved
- [ ] Can "Save As" to new file

**Files Created:**
- `frontend/src/components/WorkflowEditor/converters.ts`
- `frontend/src/components/WorkflowEditor/converters.test.ts`

**Files Modified:**
- `cmd/bento-desktop/app.go`
- `frontend/src/components/WorkflowEditor/WorkflowEditor.tsx`

---

### Task 6: Real-time Validation

**Objective:** Validate workflow as user edits

**Steps:**

1. Add validation method to App
```go
// cmd/bento-desktop/app.go

type ValidationResult struct {
    Valid  bool     `json:"valid"`
    Errors []string `json:"errors"`
}

func (a *App) ValidateWorkflowFromEditor(nodes []map[string]interface{}, edges []map[string]interface{}) (*ValidationResult, error) {
    def := convertToBentoDefinition(nodes, edges, nil)

    errors := omakase.Validate(def)

    return &ValidationResult{
        Valid:  len(errors) == 0,
        Errors: errors,
    }, nil
}
```

2. Add validation to WorkflowEditor
```typescript
// frontend/src/components/WorkflowEditor/WorkflowEditor.tsx
import { ValidateWorkflowFromEditor } from '../../../wailsjs/go/main/App'

export function WorkflowEditor() {
    const [validationErrors, setValidationErrors] = useState<string[]>([])

    useEffect(() => {
        // Debounce validation
        const timer = setTimeout(() => {
            validateWorkflow()
        }, 500)

        return () => clearTimeout(timer)
    }, [nodes, edges])

    const validateWorkflow = async () => {
        const result = await ValidateWorkflowFromEditor(nodes, edges)
        setValidationErrors(result.errors)
    }

    return (
        <div className="workflow-editor">
            {validationErrors.length > 0 && (
                <div className="validation-errors">
                    <h4>Validation Errors:</h4>
                    <ul>
                        {validationErrors.map((error, i) => (
                            <li key={i}>{error}</li>
                        ))}
                    </ul>
                </div>
            )}
            {/* ... rest of editor */}
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Validates workflow automatically (debounced)
- [ ] Shows validation errors in UI
- [ ] Highlights invalid nodes
- [ ] Prevents saving invalid workflows
- [ ] Shows specific error messages

**Files Modified:**
- `cmd/bento-desktop/app.go`
- `frontend/src/components/WorkflowEditor/WorkflowEditor.tsx`

---

## Deliverables

### Code Deliverables

- [ ] `frontend/src/components/WorkflowEditor/` - Complete editor
- [ ] `frontend/src/components/WorkflowEditor/WorkflowCanvas.tsx` - Canvas
- [ ] `frontend/src/components/WorkflowEditor/NodePalette.tsx` - Node palette
- [ ] `frontend/src/components/WorkflowEditor/PropertyPanel.tsx` - Property editor
- [ ] `frontend/src/components/WorkflowEditor/nodes/BentoNode.tsx` - Custom nodes
- [ ] `frontend/src/components/WorkflowEditor/converters.ts` - Format conversion
- [ ] Updated `cmd/bento-desktop/app.go` - Editor methods

### Documentation Deliverables

- [ ] `cmd/bento-desktop/EDITOR_GUIDE.md` - Editor usage guide
- [ ] Updated `cmd/bento-desktop/ARCHITECTURE.md` - Document editor architecture

---

## Success Criteria

### Functional Requirements
- [ ] Can create workflows visually
- [ ] Can drag nodes from palette to canvas
- [ ] Can connect nodes with edges
- [ ] Can edit node properties
- [ ] Can delete nodes and edges
- [ ] Can save workflows to file
- [ ] Can load workflows from file
- [ ] Can validate workflows in real-time
- [ ] Node positions are preserved

### Non-Functional Requirements
- [ ] Editor is responsive (no lag with 50+ nodes)
- [ ] Validation doesn't block UI
- [ ] Undo/redo works (React Flow built-in)
- [ ] Zoom/pan is smooth
- [ ] Property changes reflect immediately

### Bento Box Principle Compliance
- [ ] Editor code isolated in WorkflowEditor component tree
- [ ] Converters are pure functions (no side effects)
- [ ] No modifications to pkg/ packages
- [ ] Clear separation: UI ↔ Converter ↔ Backend

---

## Testing Checklist

### Manual Testing

- [ ] Create new workflow
- [ ] Add 10 different node types
- [ ] Connect nodes with edges
- [ ] Edit node properties
- [ ] Save workflow
- [ ] Close and reopen
- [ ] Load saved workflow
- [ ] Delete nodes
- [ ] Delete edges
- [ ] Undo changes
- [ ] Redo changes
- [ ] Validate workflow
- [ ] Fix validation errors

### Complex Scenarios

- [ ] Create workflow with 50+ nodes
- [ ] Create workflow with loops (circular edges)
- [ ] Create workflow with parallel branches
- [ ] Load complex existing workflow
- [ ] Save and reload maintains structure

---

## Performance Targets

- [ ] Render 100 nodes without lag
- [ ] Validation completes in < 500ms
- [ ] Save/load completes in < 1 second
- [ ] Property edits reflect in < 50ms
- [ ] Drag operations are smooth (60 FPS)

---

## Next Steps

After completing Phase 3:

1. **User Testing**
   - Get feedback on visual editor
   - Identify usability issues
   - Test with non-technical users

2. **Decision Point**
   - ✅ Proceed to Phase 4 (Polish & Distribution)
   - ⚠️ Iterate on Phase 3 if major issues
   - 📋 Add advanced features (templates, snippets)

3. **If Proceeding**
   - Review [wails-phase4-polish-distribution.md](./wails-phase4-polish-distribution.md)
   - Allocate 2 weeks for Phase 4
   - Plan distribution strategy

---

## Colossus Review Prompt

```
I've completed Phase 3 (Visual Workflow Editor) for Bento Desktop.

Before marking this phase complete, please:

1. Review all editor code against Bento Box Principle (.claude/BENTO_BOX_PRINCIPLE.md):
   - Check WorkflowEditor component tree structure
   - Verify converters are pure functions
   - Confirm no business logic in UI components

2. Verify Go Standards (.claude/GO_STANDARDS_REVIEW.md):
   - Check converter logic in app.go
   - Verify error handling in save/load operations
   - Check validation integration

3. Review React/TypeScript code:
   - Check component sizes (< 300 lines)
   - Verify proper TypeScript types
   - Check for code duplication
   - Verify React Flow best practices

4. Verify all Task Acceptance Criteria are met (listed in wails-phase3-workflow-editor.md)

5. Run the code-review command: /code-review cmd/bento-desktop/frontend/src/components/WorkflowEditor/

Key areas to scrutinize:
- Are converters.ts functions pure and well-tested?
- Is the React Flow integration clean?
- Are node components properly structured?
- Is the property panel handling all parameter types correctly?
- Does validation work correctly without blocking UI?

Testing requirements:
- Create a workflow with 20+ nodes
- Test save/load preserves structure
- Test validation catches errors
- Test property editing for all parameter types
- Test undo/redo functionality

After review, provide:
- List of issues found (if any)
- Recommendations for improvements
- Approval to proceed to Phase 4 OR items to address first

Do not approve this phase until:
1. The code-review command has been run
2. All manual testing scenarios pass
3. All critical issues are resolved
4. Editor can create, save, and load workflows correctly
```

---

**Phase 3 Status:** Ready to implement
**Previous Phase:** [Phase 2: Core Features](./wails-phase2-core-features.md)
**Next Phase:** [Phase 4: Polish & Distribution](./wails-phase4-polish-distribution.md)
