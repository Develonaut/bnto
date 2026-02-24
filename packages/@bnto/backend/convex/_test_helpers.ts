/**
 * Shared test constants and helpers for @bnto/backend tests.
 *
 * Run limits are pulled from `_helpers/run_limits.ts` — the single source
 * of truth. In tests, env vars are typically unset so the defaults apply.
 */

import {
  getAnonymousRunLimit,
  getFreePlanRunLimit,
} from "./_helpers/run_limits";

/** Anonymous user default run limit (driven by ANONYMOUS_RUN_LIMIT env var). */
export const FREE_RUN_LIMIT = getAnonymousRunLimit();

/** Free-tier plan run limit for real (non-anonymous) users (driven by FREE_PLAN_RUN_LIMIT env var). */
export const FREE_PLAN_RUN_LIMIT = getFreePlanRunLimit();

/** Compute next month's reset timestamp (same logic as auth.ts callback). */
export function nextMonthReset() {
  const now = Date.now();
  const nextMonth = new Date(now);
  nextMonth.setMonth(nextMonth.getMonth() + 1);
  nextMonth.setDate(1);
  nextMonth.setHours(0, 0, 0, 0);
  return nextMonth.getTime();
}

/** Compute a reset timestamp in the past (for testing reset logic). */
export function pastReset() {
  return Date.now() - 1000;
}
