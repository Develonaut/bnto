"use client";

import { useConvexAuth } from "convex/react";
import { useCurrentUser } from "./useCurrentUser";

export interface AuthUser {
  id: string;
  name: string;
  email: string;
  image?: string | null;
}

export interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: AuthUser | null;
}

/**
 * Formatted auth state — maps Convex auth + user data to a clean interface.
 *
 * Components use this instead of raw `useConvexAuth()` to get user profile
 * data alongside authentication status.
 */
export function useAuth(): AuthState {
  const { isAuthenticated, isLoading: authLoading } = useConvexAuth();
  const { data: currentUser, isLoading: userLoading } = useCurrentUser();

  const user: AuthUser | null = currentUser
    ? {
        id: currentUser.id,
        name: currentUser.name ?? "",
        email: currentUser.email ?? "",
        image: currentUser.image ?? null,
      }
    : null;

  return {
    isAuthenticated,
    isLoading: authLoading || userLoading,
    user,
  };
}
