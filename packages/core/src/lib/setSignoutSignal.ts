import { SIGNOUT_COOKIE } from "../constants";

/** Default TTL for the signout signal cookie (10 seconds). */
const SIGNOUT_SIGNAL_TTL_SECONDS = 10;

/**
 * Sets the signout signal cookie.
 *
 * This non-HttpOnly cookie tells the proxy middleware to let the user
 * through to /signin even though the HttpOnly session cookie hasn't
 * been cleared by the server yet. It expires after 10 seconds — just
 * long enough for the server to process the signout and clear the
 * session cookie.
 */
export function setSignoutSignal() {
  document.cookie = `${SIGNOUT_COOKIE}=1; path=/; max-age=${SIGNOUT_SIGNAL_TTL_SECONDS}; samesite=lax`;
}
