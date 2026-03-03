"use client";

import { useEffect, useRef } from "react";
import { core } from "../core";
import { useIsAuthenticated } from "./useIsAuthenticated";

/**
 * Wires auth status into the execution client and triggers
 * local→server history migration on auth state change (unauth→auth).
 *
 * Must be rendered once inside BntoProvider.
 */
export function useHistorySync() {
  const isAuthenticated = useIsAuthenticated();
  const prevAuthRef = useRef(isAuthenticated);
  const migratedRef = useRef(false);

  // Keep auth status getter in sync for the execution client
  useEffect(() => {
    core.executions.setAuthStatusGetter(() => isAuthenticated);
  }, [isAuthenticated]);

  // Migrate local history on unauth→auth transition (signup/signin)
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
