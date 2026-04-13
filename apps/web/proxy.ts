import { NextResponse } from "next/server";
import { convexAuthNextjsMiddleware } from "@bnto/auth/server";

/**
 * Proxy middleware — canonical URL normalization.
 *
 * Wraps `convexAuthNextjsMiddleware` so @convex-dev/auth handles token
 * refresh and cookie management automatically. Our custom logic handles
 * URL normalization only.
 *
 * Normalization rules:
 *   - Lowercase all path segments
 *   - Convert underscores to hyphens
 *   - Strip trailing slashes
 */
export default convexAuthNextjsMiddleware(async (request) => {
  const { pathname } = request.nextUrl;

  // Canonical URL normalization — lowercase, underscores to hyphens, no trailing slash
  const normalized = pathname.toLowerCase().replace(/_/g, "-").replace(/\/$/, "") || "/";
  if (pathname !== normalized) {
    const url = request.nextUrl.clone();
    url.pathname = normalized;
    return NextResponse.redirect(url, 301);
  }
});

/**
 * Matcher config — skip non-page routes.
 *
 * Excludes: _next (static assets, HMR), api routes, favicon, and
 * any file with an extension (images, fonts, etc.)
 */
export const config = {
  matcher: [
    // Page routes — skip static assets, API routes, PostHog proxy, and files
    "/((?!_next|api|d|favicon.ico|.*\\..*).*)",
    // Auth proxy — @convex-dev/auth proxies signIn/signOut through middleware
    "/api/auth",
  ],
};
