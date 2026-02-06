/**
 * @bento/core — Transport-agnostic API layer.
 *
 * Consumers call core methods (run, validate, list).
 * Internally routes to ConvexClient (web), WailsClient (desktop), or RestClient.
 */

export interface BentoAPI {
  workflows: {
    run(id: string): Promise<Execution>;
    validate(definition: WorkflowDefinition): Promise<ValidationResult>;
    list(): Promise<Workflow[]>;
    get(id: string): Promise<Workflow>;
    save(workflow: Workflow): Promise<Workflow>;
  };
  executions: {
    get(id: string): Promise<Execution>;
    list(workflowId?: string): Promise<Execution[]>;
  };
}

export interface Workflow {
  id: string;
  name: string;
  definition: WorkflowDefinition;
  createdAt: string;
  updatedAt: string;
}

export interface WorkflowDefinition {
  name: string;
  nodes: WorkflowNode[];
}

export interface WorkflowNode {
  id: string;
  type: string;
  params: Record<string, unknown>;
}

export interface Execution {
  id: string;
  workflowId: string;
  status: "pending" | "running" | "completed" | "failed";
  progress: number;
  currentNode?: string;
  startedAt?: string;
  completedAt?: string;
  error?: string;
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
}
