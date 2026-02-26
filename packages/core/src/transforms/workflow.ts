import type {
  RawWorkflowDoc,
  RawWorkflowListProjection,
} from "../types/raw";
import type { Workflow, WorkflowListItem } from "../types";

export function toWorkflow(doc: RawWorkflowDoc): Workflow {
  return {
    id: String(doc._id),
    userId: String(doc.userId),
    name: doc.name,
    definition: doc.definition,
    version: doc.version,
    isPublic: doc.isPublic,
    createdAt: doc.createdAt,
    updatedAt: doc.updatedAt,
  };
}

export function toWorkflowListItem(doc: RawWorkflowListProjection): WorkflowListItem {
  return {
    id: String(doc._id),
    name: doc.name,
    nodeCount: doc.nodeCount,
    updatedAt: doc.updatedAt,
  };
}
