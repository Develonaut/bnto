import { NextResponse } from "next/server";
import {
  convexAuthNextjsMiddleware,
  nextjsMiddlewareRedirect,
} from "@bnto/auth/server";
import { isProtectedPath } from "@/lib/routes";

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
 * 2. Public routes (including /signin, /signup): pass through immediately (no auth check)
 * 3. Protected routes (/executions, /settings): redirect to /signin if not authenticated
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
  // Recipe pages, home, pricing, faq, auth pages, etc. don't need auth state.
  if (!isProtectedPath(pathname)) return;

  const isAuthenticated = await convexAuth.isAuthenticated();

  // Protected routes — redirect to /signin if not authenticated.
  //
  // NOTE: We intentionally do NOT redirect authenticated users away from
  // /signin or /signup. Convex auto-creates anonymous sessions, so
  // isAuthenticated() returns true even for users without a real account.
  // The proxy can't distinguish anonymous sessions from real accounts
  // without an extra API call. Auth pages handle the "already signed in"
  // case client-side instead.
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
