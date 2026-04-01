# @bnto/backend

Convex data layer. Schema, server functions, and business logic.

## Overview

`@bnto/backend` is the data layer for bnto's cloud features. It defines the database schema, server-side queries, mutations, and actions using Convex. Named by role (not technology) so internals can be swapped without changing consumers.

Consumed by `@bnto/core` adapters only. App code never imports from this package directly.

## Directory Structure

```
convex/
├── schema.ts                 # Database schema (tables, indexes, validators)
├── recipes.ts                # Recipe CRUD (list, get, save, remove)
├── executions.ts             # Execution lifecycle (start, progress, complete, fail)
├── execution_events.ts       # Lightweight billing/usage event log
├── execution_analytics.ts    # Execution aggregates and stats
├── analytics.ts              # Usage analytics queries
├── users.ts                  # User profile queries
├── uploads.ts                # Presigned upload URL generation (R2)
├── downloads.ts              # Presigned download URL generation (R2)
├── auth.ts                   # Auth lifecycle callbacks
├── auth.config.ts            # @convex-dev/auth provider configuration
├── crons.ts                  # Scheduled jobs (stale upload cleanup)
├── cleanup.ts                # Manual cleanup actions
├── cleanup_stale.ts          # Stale upload cleanup logic
├── http.ts                   # HTTP routes
├── _helpers/                 # Internal helpers (auth, R2 client, validation)
└── _generated/               # Convex codegen output (api.d.ts, dataModel.d.ts)
```

## Schema

| Table             | Purpose                                                  | Key Indexes                                   |
| ----------------- | -------------------------------------------------------- | --------------------------------------------- |
| `users`           | User profiles + usage stats (plan, totalRuns)            | `by_email`                                    |
| `recipes`         | Saved recipe definitions (JSON)                          | `by_user`, `by_user_name`                     |
| `executions`      | Execution lifecycle tracking (status, progress, results) | `by_user`, `by_recipe`, `by_status_startedAt` |
| `executionLogs`   | Per-node execution logs                                  | `by_execution`                                |
| `executionEvents` | Lightweight usage events for billing                     | `by_userId`, `by_slug`, `by_userId_timestamp` |

Auth tables are imported from `@convex-dev/auth`.

## Development

```bash
# Start Convex dev server (usually via task dev)
cd packages/@bnto/backend && npx convex dev

# Run tests
cd packages/@bnto/backend && pnpm test

# Deploy to production (automated on merge to main)
npx convex deploy --yes
```

## Testing

Tests use `convex-test` (Vitest + Convex test harness) for isolated function testing. Test files are co-located in `convex/` alongside the functions they test.
