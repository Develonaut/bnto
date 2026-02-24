/**
 * Shared test constants and helpers for @bnto/backend tests.
 *
 * Centralizes values that must stay in sync with production code
 * (auth.ts, quota.ts) so tests don't silently drift.
 */

/** Anonymous user default run limit (matches auth.ts FREE_RUN_LIMIT). */
export const FREE_RUN_LIMIT = 5;

/** Free-tier plan run limit for real (non-anonymous) users. */
export const FREE_PLAN_RUN_LIMIT = 25;

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
