import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { isAuthPath, isProtectedPath } from "#lib/routes";
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

function hasSessionCookie(request: NextRequest) {
  return (
    request.cookies.has(SESSION_COOKIE) ||
    request.cookies.has(SECURE_SESSION_COOKIE)
  );
}

function hasSignoutSignal(request: NextRequest) {
  return request.cookies.has(SIGNOUT_COOKIE);
}

/**
 * Proxy middleware -- three-tier route protection.
 *
 * Runs before any page renders. Three tiers:
 *
 * 1. Canonical URL normalization (lowercase, no trailing slash)
 * 2. Auth routes (/signin, /signup): redirect to / if already authenticated
 * 3. Protected routes (/workflows, etc.): redirect to /signin if not authenticated
 * 4. Everything else (bnto slugs, public pages, unknown paths): pass through
 */
export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Canonical URL normalization — lowercase, underscores to hyphens, no trailing slash
  const normalized =
    pathname.toLowerCase().replace(/_/g, "-").replace(/\/$/, "") || "/";
  if (pathname !== normalized) {
    const url = request.nextUrl.clone();
    url.pathname = normalized;
    return NextResponse.redirect(url, 301);
  }

  const isAuthenticated = hasSessionCookie(request);

  // Tier 1: Auth routes — redirect to home if already authenticated
  if (isAuthenticated && isAuthPath(pathname)) {
    if (hasSignoutSignal(request)) {
      return NextResponse.next();
    }
    return NextResponse.redirect(new URL("/", request.url));
  }

  // Tier 2: Protected routes — redirect to /signin if not authenticated
  if (!isAuthenticated && isProtectedPath(pathname)) {
    return NextResponse.redirect(new URL("/signin", request.url));
  }

  // Tier 3: Everything else — pass through (bnto slugs, public pages, unknown paths)
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
