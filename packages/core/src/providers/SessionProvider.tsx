"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import { useConvexAuth } from "convex/react";
import { SessionContext, type AuthStatus } from "./SessionContext";

interface SessionProviderProps {
  children: React.ReactNode;
  /** Called when auth transitions from authenticated to unauthenticated (session loss). */
  onSessionLost?: () => void;
}

/**
 * Tracks authentication state and detects session loss.
 *
 * Wraps Convex's `useConvexAuth()` and provides a stable `AuthStatus`
 * (`loading` | `authenticated` | `unauthenticated`) via React context.
 *
 * When the status transitions from `authenticated` to `unauthenticated`,
 * the `onSessionLost` callback fires. This lets the app wire navigation
 * (e.g., `router.replace("/signin")`) without coupling core to a router.
 */
export function SessionProvider({ children, onSessionLost }: SessionProviderProps) {
  const { isAuthenticated, isLoading } = useConvexAuth();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const status: AuthStatus = isLoading || !mounted
    ? "loading"
    : isAuthenticated
      ? "authenticated"
      : "unauthenticated";

  // Track previous status to detect auth -> unauth transitions.
  const prevStatusRef = useRef(status);

  useEffect(() => {
    const prev = prevStatusRef.current;
    prevStatusRef.current = status;

    if (prev === "authenticated" && status === "unauthenticated") {
      onSessionLost?.();
    }
  }, [status, onSessionLost]);

  const ready = mounted && !isLoading;

  const value = useMemo(
    () => ({ status, ready }),
    [status, ready],
  );

  return (
    <SessionContext value={value}>
      {children}
    </SessionContext>
  );
}
