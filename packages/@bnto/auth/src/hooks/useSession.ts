"use client";

import { authClient } from "../client";

/** Auth session state — user, session, and loading status. */
export function useSession() {
  return authClient.useSession();
}
