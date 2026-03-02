/**
 * Route definitions -- single source of truth for all route paths.
 *
 * Three-tier routing model:
 *   1. Auth routes: redirect away if already authenticated
 *   2. Protected routes: redirect to /signin if not authenticated
 *   3. Everything else: pass through (public — bnto slugs, landing, etc.)
 *
 * Unknown paths pass through middleware and 404 at the page level.
 */

export const ROUTES = {
  home: "/",
  signin: "/signin",
  signup: "/signup",
  waitlist: "/waitlist",
  workflows: "/workflows",
  executions: "/executions",
  settings: "/settings",
} as const satisfies Record<string, string>;

/**
 * Paths intended only for unauthenticated users.
 * Authenticated users visiting these paths are redirected to home.
 */
export const AUTH_PATHS = [ROUTES.signin, ROUTES.signup] as const;

/**
 * Paths that require a real authenticated account (not anonymous).
 * Unauthenticated users are redirected to /signin.
 */
export const PROTECTED_PATHS = [
  ROUTES.executions,
  ROUTES.settings,
] as const;

type AuthPath = (typeof AUTH_PATHS)[number];
type ProtectedPath = (typeof PROTECTED_PATHS)[number];

/**
 * Returns true if the pathname is an auth-flow page that authenticated
 * users should be redirected away from (e.g. /signin, /signup).
 */
export function isAuthPath(pathname: string): pathname is AuthPath {
  return (AUTH_PATHS as readonly string[]).includes(pathname);
}

/**
 * Returns true if the pathname requires authentication.
 * Matches exact paths and sub-paths (e.g. /workflows/123).
 */
export function isProtectedPath(pathname: string): pathname is ProtectedPath {
  return (PROTECTED_PATHS as readonly string[]).some(
    (p) => pathname === p || pathname.startsWith(`${p}/`),
  );
}
