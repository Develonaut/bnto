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
    // Bnto-specific fields (set by ensureUser after first sign-in)
    plan: v.optional(
      v.union(v.literal("free"), v.literal("starter"), v.literal("pro")),
    ),
    runsUsed: v.optional(v.number()),
    runLimit: v.optional(v.number()),
    runsResetAt: v.optional(v.number()),
    isWhitelisted: v.optional(v.boolean()),
  })
    .index("by_userId", ["userId"])
    .index("email", ["email"]),

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
});
