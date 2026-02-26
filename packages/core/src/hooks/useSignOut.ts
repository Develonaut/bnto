"use client";

import { useCallback } from "react";
// Pragmatic exception: useAuthActions must bind directly to the Convex auth
// provider to call signOut(). This is auth-infrastructure that manages the
// session lifecycle — it cannot be abstracted through adapters.
import { useAuthActions } from "@convex-dev/auth/react";
import { core } from "../core";

/**
 * Orchestrated sign-out hook.
 *
 * Returns a memoized function that performs the full sign-out sequence:
 * signal cookie + cache clear + Convex session termination.
 *
 * The caller is responsible for navigation (e.g., `router.replace("/signin")`).
 */
export function useSignOut() {
  const { signOut } = useAuthActions();

  return useCallback(() => {
    core.auth.signOutSideEffects();
    void signOut();
  }, [signOut]);
}
