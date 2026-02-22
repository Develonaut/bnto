"use client";

import { useCallback } from "react";
import { core } from "../core";

/**
 * Orchestrated sign-out hook.
 *
 * Returns a memoized function that performs the full sign-out sequence:
 * signal cookie + cache clear + server cleanup.
 *
 * The caller is responsible for navigation (e.g., `router.replace("/signin")`).
 */
export function useSignOut() {
  return useCallback(() => {
    core.auth.signOut();
  }, []);
}
