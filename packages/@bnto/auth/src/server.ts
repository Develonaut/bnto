/**
 * Server-side auth exports for Next.js integration.
 *
 * These are imported directly by `apps/web/proxy.ts` and `apps/web/app/layout.tsx`.
 * This is an intentional exception to the "@bnto/core internals only" rule:
 * Next.js requires the auth provider to wrap the component tree at the root
 * layout level, and middleware/proxy must run before any React code executes.
 * Neither of these integration points can go through @bnto/core — they are
 * framework bootstrap code that must be at the app level.
 */
export {
  ConvexAuthNextjsServerProvider,
  convexAuthNextjsMiddleware,
  convexAuthNextjsToken,
  isAuthenticatedNextjs,
  nextjsMiddlewareRedirect,
} from "@convex-dev/auth/nextjs/server";
