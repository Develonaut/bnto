import type { Doc, Id } from "@bnto/backend/convex/_generated/dataModel";
import type { Workflow, WorkflowListItem } from "../types";

type WorkflowDoc = Doc<"workflows">;

/** Convex list query returns projected fields, not a full doc. */
interface WorkflowListProjection {
  _id: Id<"workflows">;
  name: string;
  nodeCount: number;
  updatedAt: number;
}

export function toWorkflow(doc: WorkflowDoc): Workflow {
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

export function toWorkflowListItem(doc: WorkflowListProjection): WorkflowListItem {
  return {
    id: String(doc._id),
    name: doc.name,
    nodeCount: doc.nodeCount,
    updatedAt: doc.updatedAt,
  };
}
