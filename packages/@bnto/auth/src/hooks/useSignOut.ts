"use client";

import { authClient } from "../client";

/** Sign out the current user. */
export function useSignOut() {
  return authClient.signOut;
}
