"use client";

import { useAuthActions } from "@convex-dev/auth/react";

/**
 * Sign up with name/email/password.
 *
 * Returns a function that accepts { name, email, password } and creates
 * an account via the Password provider with flow: "signUp".
 */
export function useSignUp() {
  const { signIn } = useAuthActions();

  return {
    email: ({
      name,
      email,
      password,
    }: {
      name: string;
      email: string;
      password: string;
    }) => signIn("password", { name, email, password, flow: "signUp" }),
  };
}
