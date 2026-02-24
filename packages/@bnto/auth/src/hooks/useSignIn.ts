"use client";

import { useAuthActions } from "@convex-dev/auth/react";

/**
 * Sign in with email/password.
 *
 * Returns a function that accepts { email, password } and signs in
 * via the Password provider with flow: "signIn".
 */
export function useSignIn() {
  const { signIn } = useAuthActions();

  return {
    email: ({ email, password }: { email: string; password: string }) =>
      signIn("password", { email, password, flow: "signIn" }),
  };
}
