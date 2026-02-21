# Convex Function Standards

## Validators

All inputs validated with Convex validators (`v.string()`, `v.id()`, etc.). Never trust client input.

## Auth Checks

- Mutations that modify user data MUST verify the authenticated user owns the resource
- Public queries (workflow listings, public profiles) can be unauthenticated

## Error Handling

- Throw `ConvexError` with meaningful messages for user-facing errors
- Don't swallow errors silently

## Query Performance

Every Convex query and mutation must be reviewed for these patterns:

### Use `ctx.db.get()` for direct ID lookups

**Never** use `.filter()` or `.query().filter(q => q.eq(q.field("_id"), id))` to look up a document by its `_id`. This is a full table scan. Use `ctx.db.get(id)` instead -- it's O(1).

```typescript
// BAD -- full table scan to find one document
const workflow = await ctx.db
  .query("workflows")
  .filter((q) => q.eq(q.field("_id"), args.workflowId))
  .first();

// GOOD -- direct O(1) lookup
const workflow = await ctx.db.get(args.workflowId as Id<"workflows">);
```

When the ID is stored as `v.string()`, cast to `Id<"tableName">` at the call site.

### No N+1 queries -- batch fetch related data

When fetching related data for a list of items (e.g., workflows for a list of executions), **never** query individually per item. Instead: deduplicate IDs -> batch fetch unique -> build a Map -> join back.

```typescript
// BAD -- N+1: one workflow query per execution
const withWorkflows = await Promise.all(
  executions.map(async (execution) => {
    const workflow = await ctx.db
      .query("workflows")
      .withIndex("by_id", (q) => q.eq("_id", execution.workflowId))
      .first();
    return { ...execution, workflow };
  }),
);

// GOOD -- batch: one query per unique workflow
const workflowIds = [...new Set(executions.map((e) => e.workflowId))];
const workflows = await Promise.all(
  workflowIds.map((id) => ctx.db.get(id as Id<"workflows">)),
);
const workflowMap = new Map(
  workflows.filter((w): w is NonNullable<typeof w> => w !== null).map((w) => [w._id, w]),
);
return executions.map((e) => ({ ...e, workflow: workflowMap.get(e.workflowId) ?? null }));
```

### Always use `.withIndex()` over `.filter()`

If an index exists for the field you're filtering on, use `.withIndex()`. The `.filter()` method scans the entire table in-memory. Check `schema.ts` for available indexes before writing queries.

### Be mindful of `.collect()` on growing tables

`.collect()` loads every matching document into memory. For tables that grow unboundedly (executions, executionLogs), prefer `.take(n)` or paginated queries. Acceptable for bounded result sets (e.g., a workflow's node definitions).

## Package Boundary

`@bnto/backend` is consumed by `@bnto/core` internals only. App code NEVER imports from it directly.
