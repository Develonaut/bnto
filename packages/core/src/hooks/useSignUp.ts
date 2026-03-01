"use client";

import { useRef, useEffect } from "react";
import { useSignUp as useAuthSignUp } from "@bnto/auth";
import { useAuth } from "./useAuth";

/**
 * Sign up with automatic anonymous userId capture.
 *
 * Wraps @bnto/auth's low-level useSignUp with anonymous session detection.
 * When an anonymous user signs up:
 *
 * 1. This hook captures the anonymous userId via useAuth() as soon as available
 * 2. A useRef persists the value across re-renders (survives timing gaps)
 * 3. signUpEmail() automatically includes the anonymous userId
 * 4. The backend's createOrUpdateUser callback patches the user in-place
 *
 * If signUpEmail() is called before the session resolves (e.g., user navigated
 * to /signin and submitted quickly), it awaits a Promise that resolves once
 * isReady becomes true. The latest anonymousUserId is read from a ref at call
 * time, so it always has the most current value.
 *
 * @returns email - sign-up function that waits for session if needed
 * @returns anonymousUserId - captured value for E2E observability (data-anon-uid)
 * @returns isReady - true once auth state resolves (anonymous userId captured or no session)
 */
export function useSignUp() {
  const { email: rawSignUp } = useAuthSignUp();
  const { user, isLoading, isAuthenticated } = useAuth();

  const capturedAnonId = useRef<string | undefined>(undefined);
  if (user?.id && !capturedAnonId.current) {
    capturedAnonId.current = user.id;
  }

  const anonymousUserId = capturedAnonId.current ?? user?.id;

  // "Ready" means we can safely call signUpEmail with the correct anonymousUserId.
  // There's a one-render gap where isAuthenticated is true (Convex session recognized)
  // but user is still null (subscription hasn't delivered the updated user doc yet).
  // In that gap, isLoading is false but anonymousUserId is undefined — firing now
  // would create a fresh user instead of upgrading the anonymous one.
  //
  // Fix: require anonymousUserId when authenticated (ensures subscription caught up).
  // For unauthenticated users (no anonymous session), proceed immediately.
  const isReady =
    !isLoading && (!!anonymousUserId || !isAuthenticated);

  // Refs for async access inside signUpEmail — always up to date
  const anonymousUserIdRef = useRef(anonymousUserId);
  anonymousUserIdRef.current = anonymousUserId;
  const readyRef = useRef(isReady);
  readyRef.current = isReady;

  // Resolver for any in-flight signUpEmail call that's waiting for ready
  const waitResolverRef = useRef<(() => void) | null>(null);
  useEffect(() => {
    if (isReady && waitResolverRef.current) {
      waitResolverRef.current();
      waitResolverRef.current = null;
    }
  }, [isReady]);

  return {
    email: async ({
      name,
      email,
      password,
    }: {
      name: string;
      email: string;
      password: string;
    }) => {
      // If session hasn't resolved yet, wait for the effect to signal ready
      if (!readyRef.current) {
        await new Promise<void>((resolve) => {
          waitResolverRef.current = resolve;
        });
      }
      // Read the latest anonymousUserId from the ref — not the stale closure
      return rawSignUp({
        name,
        email,
        password,
        anonymousUserId: anonymousUserIdRef.current,
      });
    },
    anonymousUserId,
    isReady,
  };
}
