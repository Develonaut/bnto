import { createStore } from "zustand/vanilla";
import { persist } from "zustand/middleware";
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
// Singleton store — persisted to localStorage
// ---------------------------------------------------------------------------

const STORE_KEY = "bnto-auth";

export const authStore = createStore<AuthStoreState>()(
  persist(
    (set) => ({
      user: null,
      hasAccount: false,

      setUser: (user) => set({ user, hasAccount: true }),

      clear: () => set({ user: null }),
    }),
    { name: STORE_KEY },
  ),
);

export type { AuthStoreState };
