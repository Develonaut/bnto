"use client";

import { useEffect, useRef } from "react";
import { authClient } from "@bnto/auth";

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
  const session = authClient.useSession();
  const attempted = useRef(false);

  useEffect(() => {
    if (session.isPending) return;
    if (session.data) return;
    if (attempted.current) return;

    attempted.current = true;
    authClient.signIn.anonymous();
  }, [session.isPending, session.data]);

  const isAnonymous =
    !session.isPending &&
    session.data !== null &&
    session.data !== undefined &&
    (session.data.user as Record<string, unknown>)?.isAnonymous === true;

  return {
    isPending: session.isPending,
    isAnonymous,
    isAuthenticated:
      !session.isPending &&
      session.data !== null &&
      session.data !== undefined &&
      !isAnonymous,
    session: session.data,
  };
}
