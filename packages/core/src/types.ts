import type { Doc, Id } from "@bnto/backend/convex/_generated/dataModel";

// ---------------------------------------------------------------------------
// Convex document types (used by adapters, not exposed to UI components)
// ---------------------------------------------------------------------------

export type WorkflowId = Id<"workflows">;
export type ExecutionId = Id<"executions">;
export type UserId = Id<"users">;

export type WorkflowDoc = Doc<"workflows">;
export type ExecutionDoc = Doc<"executions">;
export type ExecutionLogDoc = Doc<"executionLogs">;
export type UserDoc = Doc<"users">;

/** Projected workflow for list views (returned by workflows.list query). */
export interface WorkflowListItem {
  _id: WorkflowId;
  name: string;
  nodeCount: number;
  updatedAt: number;
}

// ---------------------------------------------------------------------------
// Consumer-facing API interface (transport-agnostic)
// ---------------------------------------------------------------------------

export interface BntoAPI {
  workflows: {
    run(definition: WorkflowDefinition): Promise<RunResponse>;
    validate(definition: WorkflowDefinition): Promise<ValidationResult>;
    list(): Promise<WorkflowSummary[]>;
    get(name: string): Promise<WorkflowDefinition>;
    save(name: string, definition: WorkflowDefinition): Promise<void>;
    delete(name: string): Promise<void>;
  };
  executions: {
    get(id: string): Promise<Execution>;
  };
}

// ---------------------------------------------------------------------------
// Workflow types (matches Go node.Definition JSON)
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

export interface Execution {
  id: string;
  status: "pending" | "running" | "completed" | "failed";
  progress: NodeProgress[];
  result?: RunResult;
  error?: string;
  startedAt: string;
  completedAt?: string;
}

export interface NodeProgress {
  nodeId: string;
  status: string;
}

export interface RunResult {
  status: string;
  nodesExecuted: number;
  nodeOutputs: Record<string, unknown>;
  duration: number;
  error?: string;
}
