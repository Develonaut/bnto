"use client";

import { useSession } from "@bnto/auth";

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
 * Formatted auth state — maps Better Auth session to a clean interface.
 *
 * Components use this instead of raw `useSession()` to avoid coupling
 * to Better Auth's session shape.
 */
export function useAuth(): AuthState {
  const { data: session, isPending } = useSession();

  const user: AuthUser | null = session?.user
    ? {
        id: session.user.id,
        name: session.user.name,
        email: session.user.email,
        image: session.user.image ?? null,
      }
    : null;

  return {
    isAuthenticated: !!user,
    isLoading: isPending,
    user,
  };
}
