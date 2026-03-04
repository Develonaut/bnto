"use client";

import { useEffect } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { useSessionStatus } from "./useSessionStatus";
import { useCurrentUser } from "./useCurrentUser";
import { hasSignoutSignal } from "../lib/hasSignoutSignal";
import { core } from "../core";
import type { AuthUser, AuthState } from "../types/auth";
import type { User } from "../types";

/**
 * Unified auth state — session + persisted user in one hook.
 *
 * Persisted fields (user, hasAccount) are available instantly from localStorage.
 * Once the Convex session resolves, the live user takes precedence and is
 * auto-persisted for next visit. This means `user` is non-null immediately
 * for returning visitors — no flash of unauthenticated state.
 */
export function useAuth(): AuthState {
  const status = useSessionStatus();
  const { data, isLoading: userLoading } = useCurrentUser();

  const persisted = useStore(
    core.auth.store,
    useShallow((s) => ({
      user: s.user,
      hasAccount: s.hasAccount,
    })),
  );

  // convexQuery select returns User | null, but the unknown→select bridge
  // loses the output type. Trust the transform at the boundary.
  const currentUser = data as User | null;

  // During the sign-out window (~10s), the HttpOnly session cookie may
  // still be valid but the user has initiated sign-out. The signout signal
  // cookie bridges this gap — treat as unauthenticated while it's present.
  const signingOut = hasSignoutSignal();
  const isAuthenticated = status === "authenticated" && !signingOut;
  const isLoading = status === "loading" || userLoading;

  // Live user from session takes precedence. Fall back to persisted user
  // so returning visitors see their profile instantly while session loads.
  const liveUser: AuthUser | null = currentUser
    ? {
        id: currentUser.id,
        name: currentUser.name ?? "",
        email: currentUser.email ?? "",
        image: currentUser.image ?? null,
      }
    : null;

  // Auto-persist the live user when session resolves.
  // Guard on isAuthenticated so sign-out clearing the store
  // doesn't get overwritten before the session terminates.
  useEffect(() => {
    if (isAuthenticated && liveUser) {
      core.auth.rememberUser(liveUser);
    }
  }, [isAuthenticated, liveUser?.id]); // eslint-disable-line react-hooks/exhaustive-deps

  return {
    isAuthenticated,
    isLoading,
    user: liveUser ?? persisted.user,
    hasAccount: persisted.hasAccount,
    rememberUser: core.auth.rememberUser,
    clearPersistedAuth: core.auth.signOutSideEffects,
  };
}
