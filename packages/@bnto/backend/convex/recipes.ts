import { ConvexError, v } from "convex/values";
import { query, mutation } from "./_generated/server";
import type { MutationCtx } from "./_generated/server";
import type { Id } from "./_generated/dataModel";
import { getAppUserId } from "./_helpers/auth";
import { NODE_TYPE_LABELS } from "./_helpers/nodeTypeLabels";

/** Extract unique processing node type labels from a definition's nodes. */
function extractNodeTypeLabels(nodes: Array<{ type?: string }>): string[] {
  const seen = new Set<string>();
  const labels: string[] = [];
  for (const node of nodes) {
    const type = node.type;
    if (!type) continue;
    const label = NODE_TYPE_LABELS[type];
    if (!label || seen.has(type)) continue;
    seen.add(type);
    labels.push(label);
  }
  return labels;
}

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
    const nodes = (r: (typeof recipes)[number]) =>
      Array.isArray(r.definition?.nodes) ? r.definition.nodes : [];
    return recipes.map((r) => ({
      _id: r._id,
      name: r.name,
      nodeCount: nodes(r).length,
      nodeTypes: extractNodeTypeLabels(nodes(r)),
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

/** Extract format version string from a definition, if present. */
function extractFormatVersion(definition: { version?: unknown }): string | undefined {
  return typeof definition?.version === "string" ? definition.version : undefined;
}

/** Create or update a recipe. When `id` is provided, patches by ID with ownership check. */
export const save = mutation({
  args: {
    id: v.optional(v.id("recipes")),
    name: v.string(),
    definition: v.any(),
    isPublic: v.optional(v.boolean()),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new ConvexError("Not authenticated");

    const now = Date.now();
    const formatVersion = extractFormatVersion(args.definition);

    if (args.id) {
      return updateById(ctx, args.id, userId, args, formatVersion, now);
    }
    return upsertByName(ctx, userId, args, formatVersion, now);
  },
});

/** ID-based update — direct patch with ownership verification. */
async function updateById(
  ctx: MutationCtx,
  id: Id<"recipes">,
  userId: Id<"users">,
  args: { name: string; definition: unknown; isPublic?: boolean },
  formatVersion: string | undefined,
  now: number,
) {
  const existing = await ctx.db.get(id);
  if (existing === null || existing.userId !== userId) {
    throw new ConvexError("Recipe not found");
  }
  await ctx.db.patch(id, {
    name: args.name,
    definition: args.definition,
    isPublic: args.isPublic ?? existing.isPublic,
    version: existing.version + 1,
    formatVersion: formatVersion ?? existing.formatVersion,
    updatedAt: now,
  });
  return id;
}

type SaveArgs = { name: string; definition: unknown; isPublic?: boolean };

/** Name-based upsert — update existing or create new. */
async function upsertByName(
  ctx: MutationCtx,
  userId: Id<"users">,
  args: SaveArgs,
  formatVersion: string | undefined,
  now: number,
) {
  const existing = await ctx.db
    .query("recipes")
    .withIndex("by_user_name", (q) => q.eq("userId", userId).eq("name", args.name))
    .unique();

  if (existing !== null) {
    await patchRecipe(ctx, existing, args, formatVersion, now);
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
}

/** Patch an existing recipe with new values, incrementing the version. */
async function patchRecipe(
  ctx: MutationCtx,
  existing: { _id: Id<"recipes">; isPublic: boolean; version: number; formatVersion?: string },
  args: SaveArgs,
  formatVersion: string | undefined,
  now: number,
) {
  await ctx.db.patch(existing._id, {
    definition: args.definition,
    isPublic: args.isPublic ?? existing.isPublic,
    version: existing.version + 1,
    formatVersion: formatVersion ?? existing.formatVersion,
    updatedAt: now,
  });
}

/** Delete a recipe. Idempotent — already-deleted is a no-op. */
export const remove = mutation({
  args: { id: v.id("recipes") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new ConvexError("Not authenticated");
    const recipe = await ctx.db.get(args.id);
    if (recipe === null) return; // Already deleted — idempotent
    if (recipe.userId !== userId) {
      throw new ConvexError("Not authorized");
    }
    await ctx.db.delete(args.id);
  },
});
