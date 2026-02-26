"use client";

import { createContext } from "react";
import type { AuthStatus } from "../types/auth";

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
