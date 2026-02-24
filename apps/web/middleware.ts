import { NextResponse } from "next/server";
import {
  convexAuthNextjsMiddleware,
  nextjsMiddlewareRedirect,
} from "@bnto/auth/server";
import { isAuthPath, isProtectedPath } from "@/lib/routes";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";

function hasSignoutSignal(request: Request & { cookies: { has(name: string): boolean } }) {
  return request.cookies.has(SIGNOUT_COOKIE);
}

/**
 * Proxy middleware — three-tier route protection.
 *
 * Wraps `convexAuthNextjsMiddleware` so @convex-dev/auth handles token
 * refresh and cookie management automatically. Our custom logic handles
 * route protection using `convexAuth.isAuthenticated()` for real token
 * validation (not just cookie-presence checks).
 *
 * Tiers:
 * 1. Canonical URL normalization (lowercase, no trailing slash)
 * 2. Auth routes (/signin, /signup): redirect to / if already authenticated
 * 3. Protected routes (/workflows, etc.): redirect to /signin if not authenticated
 * 4. Everything else (bnto slugs, public pages, unknown paths): pass through
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

  const isAuthenticated = await convexAuth.isAuthenticated();

  // Tier 1: Auth routes — redirect to home if already authenticated
  if (isAuthenticated && isAuthPath(pathname)) {
    if (hasSignoutSignal(request)) {
      return nextjsMiddlewareRedirect(request, pathname);
    }
    return nextjsMiddlewareRedirect(request, "/");
  }

  // Tier 2: Protected routes — redirect to /signin if not authenticated
  if (!isAuthenticated && isProtectedPath(pathname)) {
    return nextjsMiddlewareRedirect(request, "/signin");
  }

  // Tier 3: Everything else — pass through (bnto slugs, public pages, unknown paths)
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
