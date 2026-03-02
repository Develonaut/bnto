"use client";

import { useSessionStatus } from "./useSessionStatus";
import { useCurrentUser } from "./useCurrentUser";
import type { AuthUser, AuthState } from "../types/auth";
import type { User } from "../types";

/**
 * Formatted auth state — maps session status + user data to a clean interface.
 *
 * Uses the existing SessionContext (via useSessionStatus) instead of creating
 * a duplicate Convex auth subscription. Components use this instead of raw
 * session hooks to get user profile data alongside authentication status.
 */
export function useAuth(): AuthState {
  const status = useSessionStatus();
  const { data, isLoading: userLoading } = useCurrentUser();

  // convexQuery select returns User | null, but the unknown→select bridge
  // loses the output type. Trust the transform at the boundary.
  const currentUser = data as User | null;

  const isAuthenticated = status === "authenticated";
  const isLoading = status === "loading" || userLoading;

  const user: AuthUser | null = currentUser
    ? {
        id: currentUser.id,
        name: currentUser.name ?? "",
        email: currentUser.email ?? "",
        image: currentUser.image ?? null,
        isAnonymous: currentUser.isAnonymous ?? !currentUser.email,
      }
    : null;

  return {
    isAuthenticated,
    isLoading,
    user,
  };
}
