"use client";

import { useEffect, useRef, useState } from "react";
// Pragmatic exception: useConvexAuth and useAuthActions are auth-infrastructure
// hooks that must bind directly to the Convex provider. They cannot be abstracted
// through the core adapter layer because they manage the auth session lifecycle
// itself — the very thing the adapter layer depends on being initialized.
import { useConvexAuth } from "convex/react";
import { useAuthActions } from "@convex-dev/auth/react";

/**
 * Lazily creates an anonymous session if no session exists.
 *
 * Use in bnto tool pages to enable anonymous-first access.
 * Only triggers once per mount — guarded by a ref to prevent double-fire.
 *
 * Returns session state so consumers can distinguish between:
 * - Loading (session check or anonymous sign-in in progress)
 * - Anonymous (auto-created session)
 * - Authenticated (real account)
 *
 * `isPending` stays `true` while the anonymous sign-in is in flight,
 * not just during the initial session check. This prevents consumers
 * from acting on a "no session" state before the sign-in completes.
 */
export function useAnonymousSession() {
  const { isAuthenticated, isLoading } = useConvexAuth();
  const { signIn } = useAuthActions();
  const attempted = useRef(false);
  const [signingIn, setSigningIn] = useState(false);

  useEffect(() => {
    if (isLoading) return;
    if (isAuthenticated) return;
    if (attempted.current) return;

    attempted.current = true;
    setSigningIn(true);
    signIn("anonymous")
      .catch((error: unknown) => {
        console.error("[useAnonymousSession] signIn failed:", error);
      })
      .finally(() => setSigningIn(false));
  }, [isLoading, isAuthenticated, signIn]);

  const pending = isLoading || signingIn;

  return {
    isPending: pending,
    isAnonymous: !pending && isAuthenticated,
    isAuthenticated: !pending && isAuthenticated,
    session: isAuthenticated ? {} : null,
  };
}
