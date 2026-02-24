"use client";

import { useEffect, useRef } from "react";
import { useConvexAuth } from "convex/react";
import { useAuthActions } from "@convex-dev/auth/react";

/**
 * Lazily creates an anonymous session if no session exists.
 *
 * Use in bnto tool pages to enable anonymous-first access.
 * Only triggers once per mount — guarded by a ref to prevent double-fire.
 *
 * Returns session state so consumers can distinguish between:
 * - Loading (session check in progress)
 * - Anonymous (auto-created session)
 * - Authenticated (real account)
 */
export function useAnonymousSession() {
  const { isAuthenticated, isLoading } = useConvexAuth();
  const { signIn } = useAuthActions();
  const attempted = useRef(false);

  useEffect(() => {
    if (isLoading) return;
    if (isAuthenticated) return;
    if (attempted.current) return;

    attempted.current = true;
    void signIn("anonymous");
  }, [isLoading, isAuthenticated, signIn]);

  return {
    isPending: isLoading,
    isAnonymous: !isLoading && isAuthenticated,
    isAuthenticated: !isLoading && isAuthenticated,
    session: isAuthenticated ? {} : null,
  };
}
