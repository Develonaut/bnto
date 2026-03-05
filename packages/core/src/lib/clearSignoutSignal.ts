import { SIGNOUT_COOKIE } from "../constants";

/**
 * Clears the signout signal cookie.
 *
 * Called when the user initiates a sign-in or sign-up — at that point any
 * previous signout signal is stale and must not suppress the new session.
 * Without this, signing back in within the 10-second TTL window causes
 * `useAuth` to treat the fresh session as unauthenticated.
 */
export function clearSignoutSignal() {
  document.cookie = `${SIGNOUT_COOKIE}=; path=/; max-age=0; samesite=lax`;
}
