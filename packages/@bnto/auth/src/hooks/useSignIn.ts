"use client";

import { authClient } from "../client";

/** Sign in with email/password. */
export function useSignIn() {
  return authClient.signIn;
}
