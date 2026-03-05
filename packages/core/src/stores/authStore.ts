import { createEnhancedStore } from "./createEnhancedStore";
import type { AuthUser } from "../types/auth";

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface AuthStoreState {
  /** Last known user profile. Available instantly from localStorage. */
  user: AuthUser | null;
  /** Whether this browser has successfully authenticated before. */
  hasAccount: boolean;
  /** Persist user profile after successful auth. */
  setUser: (user: AuthUser) => void;
  /** Clear all stored auth state (e.g., on sign-out). */
  clear: () => void;
}

// ---------------------------------------------------------------------------
// Singleton store — persisted to localStorage via createEnhancedStore
// ---------------------------------------------------------------------------

export const authStore = createEnhancedStore<AuthStoreState>({
  persist: { name: "bnto-auth" },
})((set) => ({
  user: null,
  hasAccount: false,

  setUser: (user) => set({ user, hasAccount: true }),

  clear: () => set({ user: null }),
}));

export type { AuthStoreState };
