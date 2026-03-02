import { NextResponse } from "next/server";
import {
  convexAuthNextjsMiddleware,
  nextjsMiddlewareRedirect,
} from "@bnto/auth/server";
import { isAuthPath, isProtectedPath } from "@/lib/routes";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";

function hasSignoutSignal(
  request: Request & { cookies: { has(name: string): boolean } },
) {
  return request.cookies.has(SIGNOUT_COOKIE);
}

/**
 * Proxy middleware — three-tier route protection.
 *
 * Wraps `convexAuthNextjsMiddleware` so @convex-dev/auth handles token
 * refresh and cookie management automatically. Our custom logic handles
 * route protection on the small set of routes that need it.
 *
 * PERFORMANCE: `convexAuth.isAuthenticated()` makes a network round-trip
 * to Convex Cloud. We only call it for auth paths and protected paths —
 * public routes (recipes, home, pricing, faq) skip the check entirely.
 *
 * Tiers:
 * 1. Canonical URL normalization (lowercase, no trailing slash)
 * 2. Public routes: pass through immediately (no auth check)
 * 3. Auth routes (/signin, /signup): redirect to / if already authenticated
 * 4. Protected routes (/workflows, etc.): redirect to /signin if not authenticated
 */
export default convexAuthNextjsMiddleware(async (request, { convexAuth }) => {
  const { pathname } = request.nextUrl;

  // Canonical URL normalization — lowercase, underscores to hyphens, no trailing slash
  const normalized =
    pathname.toLowerCase().replace(/_/g, "-").replace(/\/$/, "") || "/";
  if (pathname !== normalized) {
    const url = request.nextUrl.clone();
    url.pathname = normalized;
    return NextResponse.redirect(url, 301);
  }

  // Public routes — skip the expensive Convex auth check entirely.
  // Recipe pages, home, pricing, faq, etc. don't need auth state.
  const needsAuthCheck = isAuthPath(pathname) || isProtectedPath(pathname);
  if (!needsAuthCheck) return;

  const isAuthenticated = await convexAuth.isAuthenticated();

  // Auth routes — redirect to home if already authenticated.
  // Skip redirect when signout signal cookie is set — the user is signing
  // out and needs to reach /signin despite the stale session cookie.
  if (isAuthenticated && isAuthPath(pathname) && !hasSignoutSignal(request)) {
    return nextjsMiddlewareRedirect(request, "/");
  }

  // Protected routes — redirect to /signin if not authenticated
  if (!isAuthenticated && isProtectedPath(pathname)) {
    return nextjsMiddlewareRedirect(request, "/signin");
  }
});

/**
 * Matcher config -- skip non-page routes.
 *
 * Excludes: _next (static assets, HMR), api routes, favicon, and
 * any file with an extension (images, fonts, etc.)
 */
export const config = {
  matcher: [
    // Page routes — skip static assets, other API routes, and files
    "/((?!_next|api|favicon.ico|.*\\..*).*)",
    // Auth proxy — @convex-dev/auth proxies signIn/signOut through middleware
    "/api/auth",
  ],
};
