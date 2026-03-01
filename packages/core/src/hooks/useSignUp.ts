"use client";

import { useRef } from "react";
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
 * The ref protects against a race condition: after full page navigation to
 * /signin, ConvexProvider takes ~3s to re-establish the session from cookies.
 * If the user submits before then (e.g., browser auto-fill), user?.id would
 * be undefined. The ref captures the first non-null value so it's never lost.
 *
 * @returns email - sign-up function (name, email, password — no anonymousUserId needed)
 * @returns anonymousUserId - captured value for E2E observability (data-anon-uid)
 */
export function useSignUp() {
  const { email: rawSignUp } = useAuthSignUp();
  const { user } = useAuth();

  const capturedAnonId = useRef<string | undefined>(undefined);
  if (user?.id && !capturedAnonId.current) {
    capturedAnonId.current = user.id;
  }

  const anonymousUserId = capturedAnonId.current ?? user?.id;

  return {
    email: ({
      name,
      email,
      password,
    }: {
      name: string;
      email: string;
      password: string;
    }) =>
      rawSignUp({
        name,
        email,
        password,
        anonymousUserId,
      }),
    anonymousUserId,
  };
}
