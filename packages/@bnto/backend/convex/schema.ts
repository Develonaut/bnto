import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";
import { authTables } from "@convex-dev/auth/server";

export default defineSchema({
  ...authTables,

  // Extend the auth users table with Bnto-specific fields.
  // Auth fields (name, email, image, etc.) are optional per authTables.
  // Our fields are also optional so ensureUser can set them after first sign-in.
  users: defineTable({
    // Auth-provided fields
    name: v.optional(v.string()),
    image: v.optional(v.string()),
    email: v.optional(v.string()),
    emailVerificationTime: v.optional(v.number()),
    phone: v.optional(v.string()),
    phoneVerificationTime: v.optional(v.number()),
    isAnonymous: v.optional(v.boolean()),
    // Bnto-specific fields (set by ensureUser after first sign-in)
    plan: v.optional(
      v.union(v.literal("free"), v.literal("starter"), v.literal("pro")),
    ),
    runsUsed: v.optional(v.number()),
    runLimit: v.optional(v.number()),
    runsResetAt: v.optional(v.number()),
    isWhitelisted: v.optional(v.boolean()),
  }).index("email", ["email"]),

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
