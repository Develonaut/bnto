"use client";

import { useContext } from "react";
import { SessionContext } from "../providers/SessionContext";
import type { AuthStatus } from "../types/auth";

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
