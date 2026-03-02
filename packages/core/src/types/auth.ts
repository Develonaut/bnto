// ---------------------------------------------------------------------------
// Auth types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** Formatted user profile for authenticated users. */
export interface AuthUser {
  id: string;
  name: string;
  email: string;
  image?: string | null;
  /** True when the user has a Convex anonymous session (no email/password). */
  isAnonymous: boolean;
}

/** Combined authentication + user state. */
export interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: AuthUser | null;
}

/** Authentication status as tracked by the SessionProvider. */
export type AuthStatus = "loading" | "authenticated" | "unauthenticated";
