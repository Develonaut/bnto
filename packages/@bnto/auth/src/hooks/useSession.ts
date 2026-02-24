"use client";

import { useConvexAuth } from "convex/react";

/**
 * Auth session state — loading and authenticated status.
 *
 * Wraps Convex's `useConvexAuth()` to provide a shape compatible
 * with the rest of the codebase. With @convex-dev/auth, there's no
 * separate "session object" — auth state is tracked by the Convex client.
 */
export function useSession() {
  const { isAuthenticated, isLoading } = useConvexAuth();

  return {
    data: isAuthenticated ? { user: null } : null,
    isPending: isLoading,
    isAuthenticated,
  };
}
