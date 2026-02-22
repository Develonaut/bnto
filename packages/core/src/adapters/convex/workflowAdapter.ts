"use client";

import { convexQuery } from "@convex-dev/react-query";
import { api } from "@bnto/backend/convex/_generated/api";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import { getConvexClient } from "../../client";

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

export function getWorkflowsQuery() {
  return convexQuery(api.workflows.list, {});
}

export function getWorkflowQuery(id: string) {
  return convexQuery(api.workflows.get, { id: id as Id<"workflows"> });
}

// ---------------------------------------------------------------------------
// Mutations (imperative — no React hooks)
// ---------------------------------------------------------------------------

export function saveWorkflow(args: {
  name: string;
  definition: unknown;
  isPublic?: boolean;
}) {
  return getConvexClient().mutation(api.workflows.save, args);
}

export function removeWorkflow(id: string) {
  return getConvexClient().mutation(api.workflows.remove, {
    id: id as Id<"workflows">,
  });
}
