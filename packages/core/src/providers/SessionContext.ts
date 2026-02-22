"use client";

import { createContext } from "react";

/** Authentication status as tracked by the SessionProvider. */
export type AuthStatus = "loading" | "authenticated" | "unauthenticated";

export interface SessionContextValue {
  /** Current authentication status. */
  status: AuthStatus;
  /** Whether the provider stack is fully mounted and ready for queries. */
  ready: boolean;
}

/**
 * React context for session state.
 *
 * Consumed via `useReady()`, `useIsAuthenticated()`, and `useSessionStatus()`.
 * Provided by `SessionProvider` inside `BntoCoreProvider`.
 */
export const SessionContext = createContext<SessionContextValue | null>(null);
