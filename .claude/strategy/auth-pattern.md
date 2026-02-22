# Bnto Auth Pattern — Better Auth + Convex Cloud

**Date:** 2026-02-21
**Status:** Planned (Sprint 1, Wave 1)
**Reference:** Adapted from darkmatter's proven auth implementation

---

## Overview

Two-layer auth: **proxy** (route protection) + **AppGate** (UI concern). Better Auth handles sessions, Convex Cloud handles data, proxy keeps unauthorized users off private routes.

```
Route Request
    ↓
[1] Proxy (middleware.ts) — Cookie-presence check
    ├─ Auth user on /signin? → Redirect to /
    ├─ Unauth user on private route? → Redirect to /signin
    └─ Otherwise → Pass through
    ↓
[2] Root Layout — Render Providers
    ↓
[3] BntoProvider — Initialize backends
    ├─ ConvexClientProvider (hydrates after mount)
    ├─ SessionProvider (watches auth state)
    └─ AppGate (splash screen gate)
    ↓
[4] AppGate — Show splash OR content
    ├─ Auth pages → Show immediately
    └─ App pages → Show splash until session ready
    ↓
[5] App Content
```

---

## Layer 1: Proxy (Server-Side Route Protection)

Lightweight cookie-presence check before page renders. NOT full session validation — Convex validates actual sessions at the data layer.

**File:** `apps/web/src/proxy.ts` (used as Next.js middleware)

```typescript
import { NextResponse } from "next/server";
import { getSessionCookie } from "@bnto/auth/middleware";
import { isPublicPath, SIGNOUT_COOKIE } from "./lib/routes";

export function proxy(request: NextRequest) {
  const sessionCookie = getSessionCookie(request);
  const isSigningOut = request.cookies.has(SIGNOUT_COOKIE);
  const isSigninPath = request.nextUrl.pathname === "/signin";

  // Auth user trying to reach signin (and not mid-signout) → redirect home
  if (sessionCookie && !isSigningOut && isSigninPath) {
    return NextResponse.redirect(new URL("/", request.url));
  }

  // Unauth user on private route → redirect to signin
  if (!sessionCookie && !isPublicPath(request.nextUrl.pathname)) {
    return NextResponse.redirect(new URL("/signin", request.url));
  }

  return NextResponse.next();
}
```

**Route definitions** (`apps/web/src/lib/routes.ts`):

```typescript
export const PUBLIC_PATHS = ["/", "/signin"];
export const AUTH_PATHS = ["/signin"];  // Skip splash screen
export const SIGNOUT_COOKIE = "bnto-signout";

export function isPublicPath(pathname: string): boolean {
  return PUBLIC_PATHS.some(p => pathname === p || pathname.startsWith(p + "/"));
}

export function isAuthPath(pathname: string): boolean {
  return AUTH_PATHS.some(p => pathname === p || pathname.startsWith(p + "/"));
}
```

---

## Layer 2: Provider Stack

### BntoProvider (composition root)

```typescript
// packages/@bnto/core/src/providers/BntoProvider.tsx
export function BntoProvider({ children, onSessionLost }: BntoProviderProps) {
  return (
    <ConvexClientProvider>
      <SessionProvider onSessionLost={onSessionLost}>
        {children}
      </SessionProvider>
    </ConvexClientProvider>
  );
}
```

### ConvexClientProvider (hydration-safe)

Defers Convex client creation until after mount to avoid React hydration mismatches (Radix useId() issue).

```typescript
// packages/@bnto/core/src/providers/ConvexClientProvider.tsx
export function ConvexClientProvider({ children }: { children: ReactNode }) {
  const [client, setClient] = useState<ConvexReactClient | null>(null);

  useEffect(() => {
    setClient(getConvexClient());
  }, []);

  if (!client) return <>{children}</>;

  return (
    <ConvexBetterAuthProvider client={client} authClient={authClient}>
      {children}
    </ConvexBetterAuthProvider>
  );
}
```

### SessionProvider (auth state watcher)

Watches Better Auth session and fires `onSessionLost` when auth → unauth transition occurs.

```typescript
// packages/@bnto/core/src/providers/SessionProvider.tsx
export function SessionProvider({ children, onSessionLost }: SessionProviderProps) {
  const { isPending, data: session } = authClient.useSession();
  const isAuthenticated = !!session;
  const wasAuthenticated = useRef(isAuthenticated);

  useEffect(() => {
    if (!isAuthenticated && wasAuthenticated.current) {
      onSessionLost?.();
    }
    wasAuthenticated.current = isAuthenticated;
  }, [isAuthenticated, onSessionLost]);

  return (
    <SessionContext.Provider value={{ isReady: !isPending }}>
      {children}
    </SessionContext.Provider>
  );
}
```

---

## Layer 3: AppGate (Splash Screen Guard)

Guards children behind auth state resolution. Shows splash screen on protected routes until session is ready.

```typescript
// apps/web/src/gates/AppGate.tsx
export function AppGate({ children }: { children: ReactNode }) {
  const pathname = usePathname();
  const { isReady } = useSessionReady();
  const showApp = useMinDelay(isReady, 1200); // Minimum splash duration

  // Auth pages skip splash — show immediately
  if (isAuthPath(pathname)) {
    return <>{children}</>;
  }

  return (
    <Crossfade show={showApp} fallback={<SplashScreen />}>
      {children}
    </Crossfade>
  );
}
```

---

## Layer 4: Sign-Out Flow

The tricky part — session cookie is HttpOnly, so proxy can't tell the difference between "still logged in" and "logging out" without a signal.

```
User clicks signOut
  ↓
setSignoutSignal()  →  Write non-HttpOnly "bnto-signout" cookie (10s TTL)
  ↓
Clear React Query cache (cancelQueries + removeQueries)
  ↓
authSignOut() in background (clears HttpOnly session cookie)
  ↓
Navigate to /signin immediately
  ↓
Proxy sees: sessionCookie (still present) + signout cookie → allows /signin
  ↓
10s later: session cookie gone, signout cookie expired
```

```typescript
// packages/@bnto/core/src/lib/signoutSignal.ts
export function setSignoutSignal() {
  document.cookie = `${SIGNOUT_COOKIE}=1; path=/; max-age=10`;
}

// packages/@bnto/core/src/hooks/useSignOut.ts
export function useSignOut() {
  const { signOut: authSignOut } = useAuthSignOut();

  const signOut = useCallback(() => {
    setSignoutSignal();
    queryClient.cancelQueries();
    queryClient.removeQueries();
    authSignOut(); // Background, doesn't await
  }, [authSignOut]);

  return { signOut };
}
```

---

## Better Auth + Convex Setup

### @bnto/auth package

```typescript
// packages/@bnto/auth/src/authClient.ts
import { createAuthClient } from "better-auth/react";
import { convexClient } from "@convex-dev/better-auth/client/plugins";

export const authClient = createAuthClient({
  plugins: [convexClient()],
});

// packages/@bnto/auth/src/authServer.ts
export { handler, preloadAuthQuery, isAuthenticated } from convexBetterAuthNextJs({
  convexUrl: process.env.NEXT_PUBLIC_CONVEX_URL!,
  convexSiteUrl: process.env.NEXT_PUBLIC_CONVEX_SITE_URL!,
});

// packages/@bnto/auth/src/middleware.ts
export { getSessionCookie } from "better-auth/cookies";
```

### @bnto/backend (Convex functions)

```typescript
// packages/@bnto/backend/convex/auth.config.ts
import { getAuthConfigProvider } from "@convex-dev/better-auth/auth-config";
export default {
  providers: [getAuthConfigProvider()],
} satisfies AuthConfig;

// packages/@bnto/backend/convex/auth.ts
import { createClient } from "@convex-dev/better-auth";
import { convex } from "@convex-dev/better-auth/plugins";
import { betterAuth } from "better-auth/minimal";

export const authComponent = createClient<DataModel>(components.betterAuth);

export const createAuth = (ctx: GenericCtx<DataModel>) => {
  return betterAuth({
    baseURL: process.env.SITE_URL,
    database: authComponent.adapter(ctx),
    emailAndPassword: { enabled: true },
    socialProviders: {
      google: { clientId: process.env.AUTH_GOOGLE_ID!, clientSecret: process.env.AUTH_GOOGLE_SECRET! },
      discord: { clientId: process.env.AUTH_DISCORD_ID!, clientSecret: process.env.AUTH_DISCORD_SECRET! },
    },
    plugins: [convex({ authConfig })],
  });
};

// packages/@bnto/backend/convex/http.ts
import { httpRouter } from "convex/server";
import { authComponent, createAuth } from "./auth";

const http = httpRouter();
authComponent.registerRoutes(http, createAuth);
export default http;
```

---

## Environment Variables

### Frontend (Vercel)
```
NEXT_PUBLIC_CONVEX_URL=https://your-instance.convex.cloud
NEXT_PUBLIC_CONVEX_SITE_URL=https://your-app.vercel.app
```

### Backend (Convex Dashboard)
```
SITE_URL=https://your-app.vercel.app
AUTH_GOOGLE_ID=...
AUTH_GOOGLE_SECRET=...
AUTH_DISCORD_ID=...
AUTH_DISCORD_SECRET=...
```

---

## Dependencies

```
# @bnto/auth
better-auth
@convex-dev/better-auth

# @bnto/core
convex
@convex-dev/react-query
@tanstack/react-query

# @bnto/backend
convex
better-auth
@convex-dev/better-auth
```

---

## Key Insights

1. **Proxy is cheap** — just cookie presence, no DB call. Convex validates real sessions at data layer.
2. **Hydration-safe** — ConvexClientProvider defers until mount to prevent React ID mismatches.
3. **Sign-out is instant** — signal cookie + background cleanup. No waiting for server response.
4. **AppGate is the UX** — splash screen until auth state resolves, skipped on auth pages.
5. **Session loss detection** — SessionProvider watches transitions, fires callback for navigation.
6. **Desktop skips all of this** — Wails desktop has no auth, no proxy, no AppGate. @bnto/auth is web-only.
