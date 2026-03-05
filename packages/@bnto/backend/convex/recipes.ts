import { ConvexError, v } from "convex/values";
import { query, mutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** List all recipes for the current user. */
export const list = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];
    const recipes = await ctx.db
      .query("recipes")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .collect();
    return recipes.map((r) => ({
      _id: r._id,
      name: r.name,
      nodeCount: Array.isArray(r.definition?.nodes) ? r.definition.nodes.length : 0,
      updatedAt: r.updatedAt,
    }));
  },
});

/** Get a single recipe by ID. Verifies ownership. */
export const get = query({
  args: { id: v.id("recipes") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const recipe = await ctx.db.get(args.id);
    if (recipe === null || recipe.userId !== userId) return null;
    return recipe;
  },
});

/** Get a recipe by name for the current user. */
export const getByName = query({
  args: { name: v.string() },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    return ctx.db
      .query("recipes")
      .withIndex("by_user_name", (q) => q.eq("userId", userId).eq("name", args.name))
      .unique();
  },
});

/** Create or update a recipe. */
export const save = mutation({
  args: {
    name: v.string(),
    definition: v.any(),
    isPublic: v.optional(v.boolean()),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new ConvexError("Not authenticated");

    const existing = await ctx.db
      .query("recipes")
      .withIndex("by_user_name", (q) => q.eq("userId", userId).eq("name", args.name))
      .unique();

    const now = Date.now();

    // Extract format version from the definition if present
    const formatVersion =
      typeof args.definition?.version === "string" ? args.definition.version : undefined;

    if (existing !== null) {
      await ctx.db.patch(existing._id, {
        definition: args.definition,
        isPublic: args.isPublic ?? existing.isPublic,
        version: existing.version + 1,
        formatVersion: formatVersion ?? existing.formatVersion,
        updatedAt: now,
      });
      return existing._id;
    }

    return ctx.db.insert("recipes", {
      userId,
      name: args.name,
      definition: args.definition,
      version: 1,
      formatVersion,
      isPublic: args.isPublic ?? false,
      createdAt: now,
      updatedAt: now,
    });
  },
});

/** Delete a recipe. Verifies ownership. */
export const remove = mutation({
  args: { id: v.id("recipes") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new ConvexError("Not authenticated");
    const recipe = await ctx.db.get(args.id);
    if (recipe === null || recipe.userId !== userId) {
      throw new ConvexError("Recipe not found");
    }
    await ctx.db.delete(args.id);
  },
});
