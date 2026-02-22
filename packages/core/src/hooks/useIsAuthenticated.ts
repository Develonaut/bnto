"use client";

import { useContext } from "react";
import { SessionContext } from "../providers/SessionContext";

/**
 * Boolean shortcut for authentication status.
 *
 * Returns `true` when the user is authenticated, `false` otherwise.
 * While auth state is loading, returns `false`.
 */
export function useIsAuthenticated() {
  const ctx = useContext(SessionContext);
  if (!ctx) {
    throw new Error("useIsAuthenticated must be used within a BntoCoreProvider");
  }
  return ctx.status === "authenticated";
}
