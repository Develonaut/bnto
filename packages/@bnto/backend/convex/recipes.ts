import { ConvexError, v } from "convex/values";
import { query, mutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** Node types excluded from recipe tags (I/O declarations + control containers). */
const EXCLUDED_NODE_TYPES = new Set(["input", "output", "group", "loop", "parallel"]);

/**
 * Per-operation node type → display label.
 *
 * Keep in sync with NODE_TYPE_INFO in packages/@bnto/nodes/src/generated/catalog.ts.
 * Backend can't import from @bnto/nodes (Convex bundling constraint), so this map
 * is maintained manually. The canonical source is the Rust engine's metadata.
 *
 * Duplicated in: packages/core/src/transforms/recipe.ts (extractNodeTypeLabels)
 * which uses NODE_TYPE_INFO directly for the same purpose.
 */
const NODE_TYPE_LABELS: Record<string, string> = {
  "edit-fields": "Edit Fields",
  "file-rename": "Rename Files",
  "http-request": "HTTP Request",
  "image-compress": "Compress Images",
  "image-convert": "Convert Image Format",
  "image-resize": "Resize Images",
  "shell-command": "Shell Command",
  "spreadsheet-clean": "Clean CSV",
  "spreadsheet-rename": "Rename CSV Columns",
  transform: "Transform",
};

/** Extract unique processing node type labels from a definition's nodes. */
function extractNodeTypeLabels(nodes: Array<{ type?: string }>): string[] {
  const seen = new Set<string>();
  const labels: string[] = [];
  for (const node of nodes) {
    if (!node.type || seen.has(node.type) || EXCLUDED_NODE_TYPES.has(node.type)) continue;
    seen.add(node.type);
    labels.push(NODE_TYPE_LABELS[node.type] ?? node.type);
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

    // Extract format version from the definition if present
    const formatVersion =
      typeof args.definition?.version === "string" ? args.definition.version : undefined;

    // ID-based update — direct patch with ownership verification
    if (args.id) {
      const existing = await ctx.db.get(args.id);
      if (existing === null || existing.userId !== userId) {
        throw new ConvexError("Recipe not found");
      }
      await ctx.db.patch(args.id, {
        name: args.name,
        definition: args.definition,
        isPublic: args.isPublic ?? existing.isPublic,
        version: existing.version + 1,
        formatVersion: formatVersion ?? existing.formatVersion,
        updatedAt: now,
      });
      return args.id;
    }

    // Name-based upsert — find existing by user + name
    const existing = await ctx.db
      .query("recipes")
      .withIndex("by_user_name", (q) => q.eq("userId", userId).eq("name", args.name))
      .unique();

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
