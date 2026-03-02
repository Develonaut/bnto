/**
 * Route definitions -- single source of truth for all route paths.
 *
 * Two-tier routing model:
 *   1. Protected routes: redirect to /signin if not authenticated (proxy)
 *   2. Everything else: pass through (public — auth pages, bnto slugs, landing)
 *
 * Auth pages (/signin, /signup) are public at the proxy level. The redirect
 * for already-authenticated users is handled client-side by SignInForm.
 * Unknown paths pass through middleware and 404 at the page level.
 */

export const ROUTES = {
  home: "/",
  signin: "/signin",
  signup: "/signup",
  waitlist: "/waitlist",
  myRecipes: "/my-recipes",
  executions: "/executions",
  settings: "/settings",
} as const satisfies Record<string, string>;

/**
 * Auth flow paths. Public at the proxy level — no server-side redirect.
 * Client-side redirect (SignInForm) handles already-authenticated users.
 * Used by SessionProvider to skip session-lost redirects on auth pages.
 */
export const AUTH_PATHS = [ROUTES.signin, ROUTES.signup] as const;

/**
 * Paths that require authentication.
 * Unauthenticated users are redirected to /signin.
 */
export const PROTECTED_PATHS = [
  ROUTES.executions,
  ROUTES.settings,
] as const;

type AuthPath = (typeof AUTH_PATHS)[number];
type ProtectedPath = (typeof PROTECTED_PATHS)[number];

/**
 * Returns true if the pathname is an auth-flow page (e.g. /signin, /signup).
 * Used by SessionProvider to skip the session-lost redirect on auth pages.
 */
export function isAuthPath(pathname: string): pathname is AuthPath {
  return (AUTH_PATHS as readonly string[]).includes(pathname);
}

/**
 * Returns true if the pathname requires authentication.
 * Matches exact paths and sub-paths (e.g. /settings/account).
 */
export function isProtectedPath(pathname: string): pathname is ProtectedPath {
  return (PROTECTED_PATHS as readonly string[]).some(
    (p) => pathname === p || pathname.startsWith(`${p}/`),
  );
}
