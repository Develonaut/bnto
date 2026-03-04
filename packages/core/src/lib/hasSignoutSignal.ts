import { SIGNOUT_COOKIE } from "../constants";

/**
 * Checks whether the signout signal cookie is present.
 *
 * During the sign-out window (~10s), this cookie indicates the user has
 * initiated sign-out even though the HttpOnly session cookie may not be
 * cleared yet. Used by `useAuth` to suppress auto-persist and treat
 * the session as unauthenticated during this window.
 *
 * Returns `false` during SSR (no `document`).
 */
export function hasSignoutSignal(): boolean {
  if (typeof document === "undefined") return false;
  return document.cookie
    .split(";")
    .some((c) => c.trim().startsWith(`${SIGNOUT_COOKIE}=`));
}
