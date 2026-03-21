import { createEnhancedStore } from "./createEnhancedStore";
import type { AuthUser, AuthStatus } from "../types/auth";

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface AuthStoreState {
  /** Last known user profile. Available instantly from localStorage. */
  user: AuthUser | null;
  /** Whether this browser has successfully authenticated before. */
  hasAccount: boolean;
  /** Current session status (ephemeral — not persisted). */
  sessionStatus: AuthStatus;
  /** Persist user profile after successful auth. */
  setUser: (user: AuthUser) => void;
  /** Update session status from the SessionProvider. */
  setSessionStatus: (status: AuthStatus) => void;
  /** Clear all stored auth state (e.g., on sign-out). */
  clear: () => void;
}

// ---------------------------------------------------------------------------
// Singleton store — persisted to localStorage via createEnhancedStore
// ---------------------------------------------------------------------------

export const authStore = createEnhancedStore<AuthStoreState>({
  persist: {
    name: "bnto-auth",
    partialize: (state) => ({ user: state.user, hasAccount: state.hasAccount }) as AuthStoreState,
  },
})((set) => ({
  user: null,
  hasAccount: false,
  sessionStatus: "loading" as AuthStatus,

  setUser: (user) => set({ user, hasAccount: true }),
  setSessionStatus: (status) => set({ sessionStatus: status }),

  clear: () => set({ user: null }),
}));

export type { AuthStoreState };
