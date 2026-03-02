import type { RawUserDoc } from "../types/raw";
import type { User } from "../types";

/** Map legacy "starter" plan to "free" — one paid tier now. */
function normalizePlan(
  plan: "free" | "starter" | "pro" | null | undefined,
): "free" | "pro" | undefined {
  if (plan === "starter") return "free";
  if (plan === null || plan === undefined) return undefined;
  return plan;
}

export function toUser(doc: RawUserDoc): User {
  return {
    id: String(doc._id),
    email: doc.email ?? undefined,
    name: doc.name ?? undefined,
    image: doc.image ?? undefined,
    plan: normalizePlan(doc.plan),
    totalRuns: doc.totalRuns ?? undefined,
    lastRunAt: doc.lastRunAt ?? undefined,
  };
}
