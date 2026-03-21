"use client";

import { useEffect, useRef } from "react";
import { core } from "../core";
import { useIsAuthenticated } from "./useIsAuthenticated";

/**
 * Triggers local→server history migration on auth state change (unauth→auth).
 *
 * Must be rendered once inside BntoProvider.
 */
export function useHistorySync() {
  const isAuthenticated = useIsAuthenticated();
  const prevAuthRef = useRef(isAuthenticated);
  const migratedRef = useRef(false);

  useEffect(() => {
    const wasUnauth = !prevAuthRef.current;
    const isNowAuth = isAuthenticated;
    prevAuthRef.current = isAuthenticated;

    if (wasUnauth && isNowAuth && !migratedRef.current) {
      migratedRef.current = true;
      core.executions.migrateHistory().catch(() => {});
    }
  }, [isAuthenticated]);
}
