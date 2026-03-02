// ---------------------------------------------------------------------------
// Auth types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** Formatted user profile for authenticated users. */
export interface AuthUser {
  id: string;
  name: string;
  email: string;
  image?: string | null;
}

/** Combined authentication + user state. */
export interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: AuthUser | null;
}

/** Authentication status as tracked by the SessionProvider. */
export type AuthStatus = "loading" | "authenticated" | "unauthenticated";
