"use client";

import { useContext } from "react";
import { SessionContext, type AuthStatus } from "../providers/SessionContext";

/**
 * Returns the current authentication status.
 *
 * - `"loading"` -- auth state is being resolved (provider mounting, session check in flight)
 * - `"authenticated"` -- user has an active session
 * - `"unauthenticated"` -- no active session
 */
export function useSessionStatus(): AuthStatus {
  const ctx = useContext(SessionContext);
  if (!ctx) {
    throw new Error("useSessionStatus must be used within a BntoCoreProvider");
  }
  return ctx.status;
}
