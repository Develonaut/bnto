"use client";

import { useCallback } from "react";
import { useSignOut as useAuthSignOut } from "@bnto/auth";
import { core } from "../core";

/**
 * Orchestrated sign-out hook.
 *
 * Returns a memoized function that performs the full sign-out sequence:
 * signal cookie + cache clear + persisted auth clear + Convex session termination.
 *
 * The caller is responsible for navigation (e.g., `router.replace("/signin")`).
 */
export function useSignOut() {
  const authSignOut = useAuthSignOut();

  return useCallback(() => {
    core.auth.signOutSideEffects();
    void authSignOut();
  }, [authSignOut]);
}
