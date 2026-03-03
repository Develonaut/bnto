import { defineSchema, defineTable } from "convex/server";
import { authTables } from "@convex-dev/auth/server";
import { v } from "convex/values";

export default defineSchema({
  ...authTables,

  // Override the auth-managed users table with app-specific fields.
  // Auth fields (name, email, image) are managed by @convex-dev/auth via
  // the createOrUpdateUser callback. App fields (plan, totalRuns, etc.)
  // are set in the same callback on user creation.
  users: defineTable({
    // Auth-managed fields
    name: v.optional(v.string()),
    email: v.optional(v.string()),
    image: v.optional(v.string()),
    emailVerificationTime: v.optional(v.number()),
    phone: v.optional(v.string()),
    phoneVerificationTime: v.optional(v.number()),
    // App fields — set by createOrUpdateUser callback in auth.ts
    plan: v.union(v.literal("free"), v.literal("pro")),
    // Usage analytics
    totalRuns: v.number(), // all-time total, never resets
    lastRunAt: v.optional(v.number()), // timestamp of most recent run
  }).index("email", ["email"]),

  recipes: defineTable({
    userId: v.id("users"),
    name: v.string(),
    // RecipeDefinition JSON — validated by engine, stored opaquely here
    definition: v.any(),
    version: v.number(),
    isPublic: v.boolean(),
    createdAt: v.number(),
    updatedAt: v.number(),
  })
    .index("by_user", ["userId"])
    .index("by_user_name", ["userId", "name"]),

  executions: defineTable({
    userId: v.id("users"),
    recipeId: v.optional(v.id("recipes")),
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
    result: v.optional(v.any()),
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
    .index("by_recipe", ["recipeId"])
    .index("by_status_startedAt", ["status", "startedAt"]),

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
  // Captures every run on predefined Bnto tool pages.
  // Separate from `executions` (lifecycle) — this is the billing/usage data layer.
  executionEvents: defineTable({
    userId: v.id("users"),
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
    .index("by_slug", ["slug"])
    .index("by_userId_timestamp", ["userId", "timestamp"]),
});
