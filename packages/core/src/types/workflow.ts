// ---------------------------------------------------------------------------
// Workflow types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** Full workflow as returned by detail queries. */
export interface Workflow {
  id: string;
  userId: string;
  name: string;
  definition: WorkflowDefinition;
  version: number;
  isPublic: boolean;
  createdAt: number;
  updatedAt: number;
}

/** Projected workflow for list views. */
export interface WorkflowListItem {
  id: string;
  name: string;
  nodeCount: number;
  updatedAt: number;
}

/** Input for creating or updating a workflow. */
export interface SaveWorkflowInput {
  name: string;
  definition: WorkflowDefinition;
  isPublic?: boolean;
}

// ---------------------------------------------------------------------------
// Workflow definition types (matches Go node.Definition JSON)
// ---------------------------------------------------------------------------

export interface WorkflowDefinition {
  id: string;
  type: string;
  version: string;
  parentId?: string;
  name: string;
  position: Position;
  metadata: Metadata;
  parameters: Record<string, unknown>;
  fields?: FieldsConfig;
  inputPorts: Port[];
  outputPorts: Port[];
  nodes?: WorkflowDefinition[];
  edges?: Edge[];
}

export interface Position {
  x: number;
  y: number;
}

export interface Metadata {
  createdAt?: string;
  updatedAt?: string;
  tags?: string[];
  customData?: Record<string, string>;
}

export interface Port {
  id: string;
  name: string;
  handle?: string;
}

export interface Edge {
  id: string;
  source: string;
  target: string;
  sourceHandle?: string;
  targetHandle?: string;
}

export interface FieldsConfig {
  values: Record<string, unknown>;
  keepOnlySet?: boolean;
}

// ---------------------------------------------------------------------------
// API response types (matches Go JSON responses)
// ---------------------------------------------------------------------------

export interface WorkflowSummary {
  name: string;
  nodeCount: number;
}

export interface ValidationResult {
  valid: boolean;
  errors?: string[];
}

export interface RunResponse {
  id: string;
}
