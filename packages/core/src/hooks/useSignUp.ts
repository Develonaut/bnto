"use client";

import { useSignUp as useAuthSignUp } from "@bnto/auth";

/**
 * Sign up with email/password.
 *
 * Thin wrapper around @bnto/auth's useSignUp. Returns a function
 * that accepts { name, email, password } and creates an account.
 */
export function useSignUp() {
  const { email: rawSignUp } = useAuthSignUp();

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
      return rawSignUp({ name, email, password });
    },
  };
}
