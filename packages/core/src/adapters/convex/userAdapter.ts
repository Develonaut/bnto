"use client";

import { convexQuery } from "@convex-dev/react-query";
import { api } from "@bnto/backend/convex/_generated/api";

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

export function getCurrentUserQuery() {
  return convexQuery(api.users.getMe, {});
}

/**
 * @deprecated Use `getServerQuotaQuery` from analyticsAdapter instead.
 * Kept during migration — will be removed once all consumers switch.
 */
export function getRunsRemainingQuery() {
  return convexQuery(api.users.getRunsRemaining, {});
}
