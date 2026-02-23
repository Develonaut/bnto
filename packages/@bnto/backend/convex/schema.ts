import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  // App-level user profiles linked to Better Auth component user.
  // Auth fields (name, email, image) live in the component's user table.
  // Copies are cached here for convenience queries.
  users: defineTable({
    userId: v.string(), // Better Auth component user _id
    email: v.optional(v.string()),
    name: v.optional(v.string()),
    image: v.optional(v.string()),
    isAnonymous: v.boolean(),
    plan: v.union(v.literal("free"), v.literal("starter"), v.literal("pro")),
    runsUsed: v.number(),
    runLimit: v.number(),
    runsResetAt: v.number(),
  })
    .index("by_userId", ["userId"])
    .index("email", ["email"])
    .index("by_anonymous", ["isAnonymous"]),

  workflows: defineTable({
    userId: v.id("users"),
    name: v.string(),
    // WorkflowDefinition JSON — validated by Go engine, stored opaquely here
    definition: v.any(), // eslint-disable-line @typescript-eslint/no-explicit-any
    version: v.number(),
    isPublic: v.boolean(),
    createdAt: v.number(),
    updatedAt: v.number(),
  })
    .index("by_user", ["userId"])
    .index("by_user_name", ["userId", "name"]),

  executions: defineTable({
    userId: v.id("users"),
    workflowId: v.id("workflows"),
    status: v.union(
      v.literal("pending"),
      v.literal("running"),
      v.literal("completed"),
      v.literal("failed"),
    ),
    progress: v.array(
      v.object({ nodeId: v.string(), status: v.string() }),
    ),
    // Result JSON from Go engine — shape varies
    result: v.optional(v.any()), // eslint-disable-line @typescript-eslint/no-explicit-any
    error: v.optional(v.string()),
    goExecutionId: v.optional(v.string()),
    // R2 file transit — links execution to uploaded input files and output results
    sessionId: v.optional(v.string()),
    outputFiles: v.optional(
      v.array(
        v.object({
          key: v.string(),
          name: v.string(),
          sizeBytes: v.number(),
          contentType: v.string(),
        }),
      ),
    ),
    startedAt: v.number(),
    completedAt: v.optional(v.number()),
  })
    .index("by_user", ["userId"])
    .index("by_workflow", ["workflowId"]),

  executionLogs: defineTable({
    executionId: v.id("executions"),
    nodeId: v.string(),
    level: v.union(
      v.literal("info"),
      v.literal("warn"),
      v.literal("error"),
      v.literal("debug"),
    ),
    message: v.string(),
    timestamp: v.number(),
  }).index("by_execution", ["executionId"]),

  // Lightweight analytics/billing event log.
  // Captures every run including anonymous users on predefined Bnto tool pages.
  // Separate from `executions` (lifecycle) — this is the billing/usage data layer.
  executionEvents: defineTable({
    userId: v.optional(v.id("users")),
    fingerprint: v.optional(v.string()),
    slug: v.string(),
    timestamp: v.number(),
    durationMs: v.optional(v.number()),
    status: v.union(
      v.literal("started"),
      v.literal("completed"),
      v.literal("failed"),
    ),
    executionId: v.optional(v.id("executions")),
  })
    .index("by_userId", ["userId"])
    .index("by_fingerprint", ["fingerprint"])
    .index("by_fingerprint_timestamp", ["fingerprint", "timestamp"])
    .index("by_slug", ["slug"])
    .index("by_userId_timestamp", ["userId", "timestamp"]),
});
