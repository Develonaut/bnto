// ---------------------------------------------------------------------------
// User types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

export interface User {
  id: string;
  email?: string;
  name?: string;
  image?: string;
  isAnonymous?: boolean;
  plan?: "free" | "starter" | "pro";
  runsUsed?: number;
  runLimit?: number;
  runsResetAt?: number;
}
