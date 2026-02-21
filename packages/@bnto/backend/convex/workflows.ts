import { v } from "convex/values";
import { query, mutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** List all workflows for the current user. */
export const list = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];
    const workflows = await ctx.db
      .query("workflows")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .collect();
    return workflows.map((w) => ({
      _id: w._id,
      name: w.name,
      nodeCount: Array.isArray(w.definition?.nodes)
        ? w.definition.nodes.length
        : 0,
      updatedAt: w.updatedAt,
    }));
  },
});

/** Get a single workflow by ID. Verifies ownership. */
export const get = query({
  args: { id: v.id("workflows") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const workflow = await ctx.db.get(args.id);
    if (workflow === null || workflow.userId !== userId) return null;
    return workflow;
  },
});

/** Get a workflow by name for the current user. */
export const getByName = query({
  args: { name: v.string() },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    return ctx.db
      .query("workflows")
      .withIndex("by_user_name", (q) =>
        q.eq("userId", userId).eq("name", args.name),
      )
      .unique();
  },
});

/** Create or update a workflow. */
export const save = mutation({
  args: {
    name: v.string(),
    definition: v.any(), // eslint-disable-line @typescript-eslint/no-explicit-any
    isPublic: v.optional(v.boolean()),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new Error("Not authenticated");

    const existing = await ctx.db
      .query("workflows")
      .withIndex("by_user_name", (q) =>
        q.eq("userId", userId).eq("name", args.name),
      )
      .unique();

    const now = Date.now();

    if (existing !== null) {
      await ctx.db.patch(existing._id, {
        definition: args.definition,
        isPublic: args.isPublic ?? existing.isPublic,
        version: existing.version + 1,
        updatedAt: now,
      });
      return existing._id;
    }

    return ctx.db.insert("workflows", {
      userId,
      name: args.name,
      definition: args.definition,
      version: 1,
      isPublic: args.isPublic ?? false,
      createdAt: now,
      updatedAt: now,
    });
  },
});

/** Delete a workflow. Verifies ownership. */
export const remove = mutation({
  args: { id: v.id("workflows") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new Error("Not authenticated");
    const workflow = await ctx.db.get(args.id);
    if (workflow === null || workflow.userId !== userId) {
      throw new Error("Workflow not found");
    }
    await ctx.db.delete(args.id);
  },
});
