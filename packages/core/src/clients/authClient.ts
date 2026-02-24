"use client";

import { getQueryClient } from "../client";
import { setSignoutSignal } from "../lib/setSignoutSignal";
import { onAuthError } from "../authError";

/**
 * Auth client — public API for authentication operations.
 *
 * Handles cache clearing and signout signal. Actual Convex auth operations
 * (signIn, signOut) are handled by @convex-dev/auth hooks at the React layer.
 */
export function createAuthClient() {
  return {
    /**
     * Imperative sign-out side effects: signal cookie + cache clear.
     *
     * The actual Convex session termination is performed by the React hook
     * layer via useAuthActions().signOut(). This handles the non-auth parts.
     */
    signOutSideEffects: () => {
      setSignoutSignal();
      getQueryClient().clear();
    },

    /** Subscribe to auth errors from queries/mutations. */
    onAuthError,
  } as const;
}

export type AuthClient = ReturnType<typeof createAuthClient>;
