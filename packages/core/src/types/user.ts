// ---------------------------------------------------------------------------
// User types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

export interface User {
  id: string;
  email?: string;
  name?: string;
  image?: string;
  isAnonymous?: boolean;
  plan?: "free" | "pro";
  // Usage analytics (Sprint 3)
  totalRuns?: number;
  lastRunAt?: number;
}
