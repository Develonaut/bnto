"use client";

import { useAuthActions } from "@convex-dev/auth/react";

/**
 * Sign up with name/email/password.
 *
 * Returns a function that accepts { name, email, password } and creates
 * an account via the Password provider with flow: "signUp".
 *
 * If the caller has an active anonymous session, pass `anonymousUserId`
 * to preserve the same user record during the upgrade. The backend
 * patches the anonymous user in-place instead of creating a new one.
 */
export function useSignUp() {
  const { signIn } = useAuthActions();

  return {
    email: ({
      name,
      email,
      password,
      anonymousUserId,
    }: {
      name: string;
      email: string;
      password: string;
      anonymousUserId?: string;
    }) =>
      signIn("password", {
        name,
        email,
        password,
        flow: "signUp",
        ...(anonymousUserId ? { anonymousUserId } : {}),
      }),
  };
}
