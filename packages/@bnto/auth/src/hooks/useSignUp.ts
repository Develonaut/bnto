"use client";

import { authClient } from "../client";

/** Sign up with email/password. */
export function useSignUp() {
  return authClient.signUp;
}
