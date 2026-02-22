# Next.js App Router + Convex + Better Auth: Setup & Architecture Research

**Date:** February 2026
**Status:** Research complete — informs Sprint 1 auth architecture and Sprint 2 anonymous session implementation

---

## Overview

This document captures how Next.js 15 App Router, Convex, and Better Auth compose together correctly. The stack has a specific initialization order, provider hierarchy, and middleware pattern that must be followed exactly. Deviations cause hydration errors, stale auth state, or WebSocket reconnection thrashing.

---

## 1. The provider hierarchy

The root of the app requires exactly this structure:

```tsx
// app/layout.tsx (Server Component)
import { ConvexClientProvider } from "./ConvexClientProvider";

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body className={fontSans.variable}>
        <ConvexClientProvider initialToken={initialToken}>
          {children}
        </ConvexClientProvider>
      </body>
    </html>
  );
}
```

```tsx
// app/ConvexClientProvider.tsx ("use client")
"use client";
import { ConvexReactClient } from "convex/react";
import { ConvexBetterAuthProvider } from "@convex-dev/better-auth/react";
import { authClient } from "@/lib/auth-client";

// CRITICAL: instantiate outside the component — never inside
const convex = new ConvexReactClient(process.env.NEXT_PUBLIC_CONVEX_URL!);

export function ConvexClientProvider({ children, initialToken }) {
  return (
    <ConvexBetterAuthProvider
      client={convex}
      authClient={authClient}
      initialToken={initialToken}
    >
      {children}
    </ConvexBetterAuthProvider>
  );
}
```

**Why this structure:**
- `layout.tsx` is a Server Component — it cannot use React Context directly
- `ConvexClientProvider` must be a Client Component (`"use client"`) to hold the WebSocket client
- `ConvexBetterAuthProvider` replaces `ConvexProvider` — it handles both Convex and Better Auth session wiring
- `ConvexReactClient` instantiated outside the component = singleton, survives re-renders, no WebSocket thrashing

---

## 2. File structure for the auth stack

```
convex/
├── convex.config.ts        # Register betterAuth component
├── auth.config.ts          # Configure Better Auth as Convex auth provider
├── auth.ts                 # Better Auth instance + createAuth + getCurrentUser query
└── http.ts                 # Mount auth routes on HTTP router

src/lib/
├── auth-client.ts          # Better Auth client (browser-side)
└── auth-server.ts          # Server utilities: preloadAuthQuery, isAuthenticated, getToken, etc.

app/
├── api/auth/[...all]/
│   └── route.ts            # Proxy auth requests from Next.js → Convex
├── ConvexClientProvider.tsx # "use client" wrapper with ConvexBetterAuthProvider
└── layout.tsx              # Root layout (Server Component)
```

---

## 3. Middleware pattern for three-tier routing

Bnto has three route tiers that need distinct handling:

| Tier | Routes | Who can access |
|------|--------|---------------|
| Public SEO | `/compress-images`, `/clean-csv`, etc. | Anyone — anonymous session auto-created |
| Auth routes | `/sign-in`, `/sign-up` | Unauthenticated only — redirect if already authenticated |
| Protected | `/dashboard`, `/settings`, `/workflows` | Authenticated accounts only — not anonymous sessions |

**Key insight:** Better Auth's middleware is cookie-based and can read the session directly — no API call needed. The `isAuthenticated` helper from `auth-server.ts` reads the Better Auth cookie synchronously in middleware context.

```ts
// middleware.ts
import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { isAuthenticated } from "@/lib/auth-server";

const publicSeoRoutes = createRouteMatcher([
  "/compress-images",
  "/resize-images",
  "/clean-csv",
  "/rename-files",
  // ... all predefined Bnto slugs
  "/",
]);

const authRoutes = createRouteMatcher(["/sign-in", "/sign-up"]);

const protectedRoutes = createRouteMatcher([
  "/dashboard(.*)",
  "/settings(.*)",
  "/workflows(.*)",
]);

export default async function middleware(request: NextRequest) {
  const authenticated = await isAuthenticated(request);

  // Auth routes: redirect to dashboard if already authenticated
  if (authRoutes(request) && authenticated) {
    return NextResponse.redirect(new URL("/dashboard", request.url));
  }

  // Protected routes: redirect to sign-in if not authenticated
  // NOTE: anonymous sessions are NOT "authenticated" for this check
  if (protectedRoutes(request) && !authenticated) {
    return NextResponse.redirect(new URL("/sign-in", request.url));
  }

  // Public SEO routes: allow through — anonymous session handled in the page
  return NextResponse.next();
}

export const config = {
  matcher: ["/((?!.*\\..*|_next).*)", "/", "/(api|trpc)(.*)"],
};
```

**Critical warning from research:** Do not check auth state in layouts — layouts won't stop nested pages from rendering. Auth enforcement must happen in middleware, not in layout components.

---

## 4. Anonymous session initialization

**Where to trigger it:** Lazily, in the Bnto page component itself — not in middleware, not in root layout.

**Why not middleware:** Middleware runs on every request. Creating an anonymous session on every page load without a cookie would create thousands of orphaned user records. Middleware cannot set cookies in a way that persists the session back to Better Auth's cookie store cleanly.

**Why not root layout:** Same problem — layout runs on every render including static assets and API routes.

**Correct pattern — trigger on first interaction with a Bnto page:**

```tsx
// app/[bnto]/page.tsx or a shared BntoShell component
"use client";
import { useEffect } from "react";
import { authClient } from "@/lib/auth-client";
import { useSession } from "@convex-dev/better-auth/react";

export function BntoPageShell({ children }) {
  const { data: session, isPending } = useSession();

  useEffect(() => {
    // Only create anonymous session if no session exists at all
    if (!isPending && !session) {
      authClient.signIn.anonymous();
    }
  }, [session, isPending]);

  return <>{children}</>;
}
```

This creates one anonymous user per device on first visit to any Bnto page, sets a session cookie, and all subsequent requests carry that cookie. When the user creates an account, Better Auth's `onLinkAccount` fires and migrates their data.

---

## 5. Server Components vs Client Components split

**Convex queries cannot run in React Server Components.** The `ConvexReactClient` maintains a WebSocket connection that only exists on the client. All `useQuery`, `useMutation`, and `useConvex` hooks are client-only.

**What runs server-side:** `preloadAuthQuery` from `auth-server.ts` — this fetches initial data server-side and passes it to client components as serialized props. The client component then hydrates with `usePreloadedAuthQuery`, which subscribes to real-time updates.

```tsx
// app/(dashboard)/page.tsx — Server Component
import { preloadAuthQuery } from "@/lib/auth-server";
import { api } from "@/convex/_generated/api";
import { DashboardContent } from "./DashboardContent";

export default async function DashboardPage() {
  const preloadedUser = await preloadAuthQuery(api.auth.getCurrentUser);
  return <DashboardContent preloadedUser={preloadedUser} />;
}

// app/(dashboard)/DashboardContent.tsx — Client Component
"use client";
import { usePreloadedAuthQuery } from "@convex-dev/better-auth/nextjs/client";

export function DashboardContent({ preloadedUser }) {
  const user = usePreloadedAuthQuery(preloadedUser); // real-time after initial load
  return <div>{user?.name}</div>;
}
```

**For Bnto SEO pages** (`/compress-images` etc.): the page metadata (title, description, og tags) must render server-side for SEO. The workflow execution UI renders client-side. Split accordingly:

```tsx
// app/[bnto]/page.tsx — Server Component for metadata
export async function generateMetadata({ params }) {
  const bnto = getBntoConfig(params.bnto); // static lookup, no Convex needed
  return { title: bnto.title, description: bnto.description };
}

export default function BntoPage({ params }) {
  return <BntoRunner slug={params.bnto} />; // Client Component for execution
}
```

---

## 6. JWT refresh and Convex authentication

The Convex + Better Auth adapter handles JWT refresh automatically. The flow:

1. Better Auth issues a session cookie (7-day lifetime by default)
2. The `ConvexBetterAuthProvider` reads this cookie and exchanges it for a short-lived Convex JWT (15-min default)
3. Before the JWT expires, `ConvexBetterAuthProvider` transparently refreshes it using the session cookie
4. Components never see the JWT — they interact with Convex through hooks

**Pitfall:** Static JWKS is strongly recommended over dynamic JWKS. Dynamic JWKS adds 100–400ms latency on every page load because Convex fetches the public key on each JWT verification. Set static JWKS during deployment setup.

**On page load with SSR:** Pass `initialToken` from the server to `ConvexClientProvider` to avoid a flash of unauthenticated state during hydration:

```tsx
// app/layout.tsx (Server Component)
import { getToken } from "@/lib/auth-server";

export default async function RootLayout({ children }) {
  const initialToken = await getToken(); // reads cookie server-side
  return (
    <html>
      <body>
        <ConvexClientProvider initialToken={initialToken}>
          {children}
        </ConvexClientProvider>
      </body>
    </html>
  );
}
```

---

## 7. Route groups for the three-tier model

App Router route groups (`(name)`) let you apply different layouts to different route tiers without affecting the URL:

```
app/
├── layout.tsx                    # Root layout — ConvexClientProvider, fonts, globals
├── (public)/                     # No auth requirement
│   ├── layout.tsx                # Minimal layout (nav, no auth gate)
│   ├── page.tsx                  # Landing/home
│   └── [bnto]/
│       └── page.tsx              # SEO Bnto pages — anonymous session auto-created
├── (auth)/                       # Unauthenticated only
│   ├── layout.tsx                # Redirects to /dashboard if already authenticated
│   ├── sign-in/
│   │   └── page.tsx
│   └── sign-up/
│       └── page.tsx
└── (protected)/                  # Authenticated accounts only
    ├── layout.tsx                # Redirects to /sign-in if not authenticated
    ├── dashboard/
    │   └── page.tsx
    ├── settings/
    │   └── page.tsx
    └── workflows/
        └── page.tsx
```

The middleware handles the redirects — layout groups provide the structural separation and allow different shell UIs (nav, sidebars) per tier without duplicating provider setup.

---

## 8. Common pitfalls

| Pitfall | Consequence | Fix |
|---------|-------------|-----|
| `new ConvexReactClient()` inside a component | New WebSocket on every render, memory leak | Instantiate outside component at module scope |
| Auth checks in layout components | Nested pages render before redirect fires | Enforce auth in middleware only |
| Dynamic JWKS | 100–400ms latency per page load | Use static JWKS during deployment |
| Not passing `initialToken` to provider | Flash of unauthenticated state on initial load | Fetch token server-side in root layout, pass as prop |
| Creating anonymous session in middleware | Thousands of orphaned user records | Trigger lazily in page component on first interaction |
| Using `ConvexProvider` instead of `ConvexBetterAuthProvider` | Auth state not wired to Convex JWT refresh | Always use `ConvexBetterAuthProvider` from `@convex-dev/better-auth/react` |
| Calling Better Auth `auth.api` methods from Next.js server code | Methods need Convex DB context | Call from Convex functions (query/mutation), invoke via `fetchAuthMutation` |
| Forgetting `router.refresh()` on session change | Protected routes accessible from cache | Call `router.refresh()` in `onSessionChange` callback |

---

## 9. Environment variables required

```bash
# .env.local
CONVEX_DEPLOYMENT=dev:your-project-123
NEXT_PUBLIC_CONVEX_URL=https://your-project-123.convex.cloud
NEXT_PUBLIC_CONVEX_SITE_URL=https://your-project-123.convex.site
NEXT_PUBLIC_SITE_URL=http://localhost:3000

# Set via Convex CLI (not .env.local — these run on Convex, not Next.js)
# npx convex env set BETTER_AUTH_SECRET=$(openssl rand -base64 32)
# npx convex env set SITE_URL http://localhost:3000
```

---

## References

- Official Convex + Better Auth Next.js guide: https://labs.convex.dev/better-auth/framework-guides/next
- Convex Next.js App Router docs: https://docs.convex.dev/client/nextjs/app-router/
- Convex + Better Auth example repo: https://github.com/get-convex/better-auth/tree/main/examples/next
- Convex Auth Next.js server-side auth: https://labs.convex.dev/auth/authz/nextjs
- ConvexReactClient initialization guide: https://www.schemets.com/blog/convex-nextjs-initialization-guide-app-router
- Better Auth Next.js integration: https://www.better-auth.com/docs/integrations/next-js
- Known issue — middleware isAuthenticated always false: https://github.com/get-convex/convex-auth/issues/271
