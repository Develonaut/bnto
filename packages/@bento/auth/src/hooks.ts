"use client";

import { useConvexAuth } from "convex/react";
import { useAuthActions } from "@convex-dev/auth/react";
import { useQuery } from "convex/react";
import { api } from "@bento/backend/convex/_generated/api";

/** Auth state — whether the user is authenticated and loading. */
export function useAuth() {
  return useConvexAuth();
}

/** Sign in with email/password. */
export function useSignIn() {
  const { signIn } = useAuthActions();
  return (params: { email: string; password: string; flow: "signIn" | "signUp" }) =>
    signIn("password", params);
}

/** Sign out the current user. */
export function useSignOut() {
  const { signOut } = useAuthActions();
  return signOut;
}

/** Get the current authenticated user's data. */
export function useCurrentUser() {
  return useQuery(api.users.getMe);
}

/** Check if the current user is whitelisted for app access. */
export function useIsWhitelisted() {
  return useQuery(api.users.isWhitelisted);
}
