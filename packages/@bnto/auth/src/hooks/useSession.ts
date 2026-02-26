"use client";

import { useConvexAuth } from "convex/react";

/**
 * Auth session state -- loading and authenticated status.
 *
 * Wraps Convex's `useConvexAuth()` to provide a shape compatible
 * with the rest of the codebase. With @convex-dev/auth, there's no
 * separate "session object" -- auth state is tracked by the Convex client.
 *
 * Returns `data: true` when authenticated (session exists) or `data: null`
 * when unauthenticated. User profile data is fetched separately via
 * `@bnto/core` hooks -- this hook only tracks session presence.
 */
export function useSession() {
  const { isAuthenticated, isLoading } = useConvexAuth();

  return {
    data: isAuthenticated ? true : null,
    isPending: isLoading,
    isAuthenticated,
  };
}
