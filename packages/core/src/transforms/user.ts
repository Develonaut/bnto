import type { Doc } from "@bnto/backend/convex/_generated/dataModel";
import type { User } from "../types";

type UserDoc = Doc<"users">;

export function toUser(doc: UserDoc): User {
  return {
    id: String(doc._id),
    userId: doc.userId,
    email: doc.email ?? undefined,
    name: doc.name ?? undefined,
    image: doc.image ?? undefined,
    isAnonymous: doc.isAnonymous ?? undefined,
    plan: doc.plan ?? undefined,
    runsUsed: doc.runsUsed ?? undefined,
    runLimit: doc.runLimit ?? undefined,
    runsResetAt: doc.runsResetAt ?? undefined,
  };
}
