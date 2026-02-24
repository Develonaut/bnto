/**
 * Run limit constants — single source of truth.
 *
 * All run limits are driven by Convex env vars. Change them in the Convex
 * dashboard (or via `npx convex env set`) — no code deploy needed.
 *
 * | Env Var              | Default | Who it affects          |
 * |----------------------|---------|-------------------------|
 * | ANONYMOUS_RUN_LIMIT  | 3       | Anonymous (pre-signup)  |
 * | FREE_PLAN_RUN_LIMIT  | 25      | Free-tier signed-up     |
 */

const DEFAULT_ANONYMOUS_RUN_LIMIT = 3;
const DEFAULT_FREE_PLAN_RUN_LIMIT = 25;

/** Runs allowed for anonymous users before "sign up to continue". */
export function getAnonymousRunLimit(): number {
  const val = process.env.ANONYMOUS_RUN_LIMIT;
  const parsed = val ? parseInt(val, 10) : NaN;
  return Number.isNaN(parsed) ? DEFAULT_ANONYMOUS_RUN_LIMIT : parsed;
}

/** Runs allowed for free-tier signed-up users per month. */
export function getFreePlanRunLimit(): number {
  const val = process.env.FREE_PLAN_RUN_LIMIT;
  const parsed = val ? parseInt(val, 10) : NaN;
  return Number.isNaN(parsed) ? DEFAULT_FREE_PLAN_RUN_LIMIT : parsed;
}
