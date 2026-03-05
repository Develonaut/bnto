import type { RawUserDoc } from "../types/raw";
import type { User } from "../types";

export function toUser(doc: RawUserDoc): User {
  return {
    id: String(doc._id),
    email: doc.email ?? undefined,
    name: doc.name ?? undefined,
    image: doc.image ?? undefined,
    plan: doc.plan === "starter" ? "free" : (doc.plan ?? undefined),
    totalRuns: doc.totalRuns,
    lastRunAt: doc.lastRunAt ?? undefined,
  };
}
