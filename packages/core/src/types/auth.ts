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
  /** Live user from Convex session, falls back to persisted user from localStorage. */
  user: AuthUser | null;
  /** Whether this browser has successfully authenticated before. */
  hasAccount: boolean;
  /** Persist user profile to localStorage (call after successful auth). */
  rememberUser: (user: AuthUser) => void;
  /** Clear persisted auth state (call on sign-out). */
  clearPersistedAuth: () => void;
}

/** Authentication status as tracked by the SessionProvider. */
export type AuthStatus = "loading" | "authenticated" | "unauthenticated";
