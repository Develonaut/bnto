"use client";

import { useAuthActions } from "@convex-dev/auth/react";

/** Sign out the current user via @convex-dev/auth. */
export function useSignOut() {
  const { signOut } = useAuthActions();
  return signOut;
}
