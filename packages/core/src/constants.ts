/**
 * Cookie name for the signout signal.
 *
 * The session cookie is HttpOnly -- JavaScript can't delete it. During sign-out,
 * this non-HttpOnly cookie is set with a short TTL (10s) to signal to the proxy
 * that the user is signing out. The proxy sees this cookie and skips the
 * "auth user on /signin -> redirect to /" rule.
 */
export const SIGNOUT_COOKIE = "bnto-signout";
