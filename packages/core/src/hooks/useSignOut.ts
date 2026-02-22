"use client";

import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useSignOut as useAuthSignOut } from "@bnto/auth";
import { setSignoutSignal } from "../lib/setSignoutSignal";

/**
 * Orchestrated sign-out hook.
 *
 * Returns a function that performs the full sign-out sequence:
 *
 * 1. Sets the signout signal cookie (non-HttpOnly, 10s TTL) so the proxy
 *    lets the user through to /signin despite the HttpOnly session cookie
 *    still being present.
 * 2. Clears the React Query cache (invalidates all server state).
 * 3. Fires `authClient.signOut()` in the background (server session cleanup).
 *
 * The caller is responsible for navigation (e.g., `router.replace("/signin")`).
 * This hook does NOT navigate — it returns a function the caller invokes,
 * then the caller navigates immediately without awaiting.
 */
export function useSignOut() {
  const authSignOut = useAuthSignOut();
  const queryClient = useQueryClient();

  return useCallback(() => {
    setSignoutSignal();
    queryClient.clear();
    void authSignOut();
  }, [authSignOut, queryClient]);
}
