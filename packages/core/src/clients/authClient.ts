"use client";

import { authClient } from "@bnto/auth";
import { getQueryClient } from "../client";
import { setSignoutSignal } from "../lib/setSignoutSignal";
import { onAuthError } from "../authError";

/**
 * Auth client — public API for authentication operations.
 *
 * Unlike other clients, auth doesn't have a service layer. It composes
 * the Better Auth client, query cache, and signout signal directly.
 */
export function createAuthClient() {
  return {
    /** Imperative sign-out: signal cookie + cache clear + server cleanup. */
    signOut: () => {
      setSignoutSignal();
      getQueryClient().clear();
      void authClient.signOut();
    },

    /** Imperative anonymous sign-in. */
    signInAnonymous: () => authClient.signIn.anonymous(),

    /** Subscribe to auth errors from queries/mutations. */
    onAuthError,
  } as const;
}

export type AuthClient = ReturnType<typeof createAuthClient>;
