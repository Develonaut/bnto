/**
 * Route definitions -- single source of truth for all route paths.
 *
 * The proxy imports `isPublicPath` and `isAuthPath` to decide redirects.
 * Private-by-default: only paths listed in PUBLIC_PATHS are accessible
 * without authentication.
 */

export const ROUTES = {
  home: "/",
  signin: "/signin",
  waitlist: "/waitlist",
  workflows: "/workflows",
  executions: "/executions",
  settings: "/settings",
} as const satisfies Record<string, string>;

/**
 * Paths accessible without authentication.
 * The home page (/) is public because it renders a landing page for
 * unauthenticated users and a dashboard for authenticated users.
 */
export const PUBLIC_PATHS = [
  ROUTES.home,
  ROUTES.signin,
  ROUTES.waitlist,
] as const;

/**
 * Paths intended only for unauthenticated users.
 * Authenticated users visiting these paths are redirected to home.
 */
export const AUTH_PATHS = [ROUTES.signin] as const;

type PublicPath = (typeof PUBLIC_PATHS)[number];
type AuthPath = (typeof AUTH_PATHS)[number];

/** Returns true if the pathname does not require authentication. */
export function isPublicPath(pathname: string): pathname is PublicPath {
  return (PUBLIC_PATHS as readonly string[]).includes(pathname);
}

/**
 * Returns true if the pathname is an auth-flow page that authenticated
 * users should be redirected away from (e.g. /signin).
 */
export function isAuthPath(pathname: string): pathname is AuthPath {
  return (AUTH_PATHS as readonly string[]).includes(pathname);
}
