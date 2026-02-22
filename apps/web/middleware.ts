import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { isAuthPath, isPublicPath } from "./lib/routes";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";

/**
 * Better Auth session cookie name.
 *
 * Better Auth uses "better-auth.session_token" by default (non-HTTPS) or
 * "__Secure-better-auth.session_token" in production (HTTPS). We check both
 * since we don't know at build time which environment the middleware runs in.
 */
const SESSION_COOKIE = "better-auth.session_token";
const SECURE_SESSION_COOKIE = `__Secure-${SESSION_COOKIE}`;

/**
 * Check whether the request has a Better Auth session cookie.
 *
 * This is a presence check only -- the proxy does NOT validate the session.
 * Convex validates at the data layer. The proxy just prevents unauthenticated
 * users from seeing private pages (no flash).
 */
function hasSessionCookie(request: NextRequest) {
  return (
    request.cookies.has(SESSION_COOKIE) ||
    request.cookies.has(SECURE_SESSION_COOKIE)
  );
}

/**
 * Check whether the signout signal cookie is present.
 *
 * During sign-out, the client sets this non-HttpOnly cookie (10s TTL) so the
 * proxy knows to let the user through to /signin even though the HttpOnly
 * session cookie hasn't been cleared by the server yet.
 */
function hasSignoutSignal(request: NextRequest) {
  return request.cookies.has(SIGNOUT_COOKIE);
}

/**
 * Proxy middleware -- server-side route protection.
 *
 * Runs before any page renders. Two rules:
 *
 * 1. Auth user on /signin (without signout signal) -> redirect to /
 * 2. Unauth user on private route -> redirect to /signin
 *
 * Everything else passes through.
 */
export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;
  const isAuthenticated = hasSessionCookie(request);

  // Rule 1: Auth user on an auth-only page (e.g. /signin)
  // Redirect to home unless the signout signal is present (user is signing out)
  if (isAuthenticated && isAuthPath(pathname)) {
    if (hasSignoutSignal(request)) {
      return NextResponse.next();
    }
    return NextResponse.redirect(new URL("/", request.url));
  }

  // Rule 2: Unauth user on a private route -> redirect to /signin
  if (!isAuthenticated && !isPublicPath(pathname)) {
    return NextResponse.redirect(new URL("/signin", request.url));
  }

  // Everything else passes through
  return NextResponse.next();
}

/**
 * Matcher config -- skip non-page routes.
 *
 * Excludes: _next (static assets, HMR), api routes, favicon, and
 * any file with an extension (images, fonts, etc.)
 */
export const config = {
  matcher: ["/((?!_next|api|favicon.ico|.*\\..*).*)"],
};
