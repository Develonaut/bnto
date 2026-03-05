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

/** Map a Convex User to the AuthUser shape (profile subset). */
function toAuthUser(user: User): AuthUser {
  return {
    id: user.id,
    name: user.name ?? "",
    email: user.email ?? "",
    image: user.image ?? null,
  };
}

/** Read persisted auth state from the localStorage-backed store. */
function usePersistedAuth() {
  return useStore(
    core.auth.store,
    useShallow((s) => ({ user: s.user, hasAccount: s.hasAccount })),
  );
}

/**
 * Unified auth state — session + persisted user in one hook.
 *
 * Persisted fields (user, hasAccount) are available instantly from localStorage.
 * Once the Convex session resolves, the live user takes precedence and is
 * auto-persisted for next visit.
 */
export function useAuth(): AuthState {
  const status = useSessionStatus();
  const { data, isLoading: userLoading } = useCurrentUser();
  const persisted = usePersistedAuth();
  const currentUser = data as User | null;

  const signingOut = hasSignoutSignal();
  const isAuthenticated = status === "authenticated" && !signingOut;
  const isLoading = status === "loading" || userLoading;
  const liveUser = currentUser ? toAuthUser(currentUser) : null;

  // Auto-persist live user when session resolves.
  const liveUserId = liveUser?.id;
  useEffect(() => {
    if (isAuthenticated && liveUser) core.auth.rememberUser(liveUser);
  }, [isAuthenticated, liveUserId]);

  return {
    isAuthenticated,
    isLoading,
    user: liveUser ?? persisted.user,
    hasAccount: persisted.hasAccount,
    rememberUser: core.auth.rememberUser,
    clearPersistedAuth: core.auth.signOutSideEffects,
  };
}
