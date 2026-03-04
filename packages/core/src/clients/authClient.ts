"use client";

import { getQueryClient } from "../client";
import { setSignoutSignal } from "../lib/setSignoutSignal";
import { onAuthError } from "../authError";
import { authStore } from "../stores/authStore";
import type { AuthUser } from "../types/auth";

/**
 * Auth client — public API for authentication operations.
 *
 * Owns the persisted auth store (localStorage-backed). Handles cache clearing,
 * signout signal, and persisted user lifecycle. Actual Convex auth operations
 * (signIn, signOut) are handled by @convex-dev/auth hooks at the React layer.
 */
export function createAuthClient() {
  return {
    /** The persisted auth store. Hooks subscribe to this for reactive state. */
    store: authStore,

    /** Persist user profile to localStorage (auto-sets hasAccount). */
    rememberUser: (user: AuthUser) => {
      authStore.getState().setUser(user);
    },

    /**
     * Full sign-out side effects: signal cookie + cache clear.
     *
     * The persisted user is intentionally NOT cleared — it's cached UX data
     * (email pre-fill, hasAccount flag), not an auth boundary. The signout
     * signal cookie + session status handle authentication state.
     *
     * The actual Convex session termination is performed by the React hook
     * layer via useAuthActions().signOut(). This handles everything else.
     */
    signOutSideEffects: () => {
      setSignoutSignal();
      getQueryClient().clear();
    },

    /** Clear all persisted auth data (e.g., account deletion, "forget me"). */
    clearPersistedAuth: () => {
      authStore.getState().clear();
    },

    /** Subscribe to auth errors from queries/mutations. */
    onAuthError,
  } as const;
}

export type AuthClient = ReturnType<typeof createAuthClient>;
