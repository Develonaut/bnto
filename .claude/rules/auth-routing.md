# Auth Routing

Auth is enforced at two layers: proxy (before render) and layout (UI concern). No runtime JS branching for security -- the proxy handles it.

## Principle: Proxy Is the Auth Boundary

```
app/
|-- proxy.ts                # Route protection -- runs before page renders
|-- layout.tsx              # Root -- Providers. NO auth logic.
|-- (app)/                  # App shell
|   |-- layout.tsx          # AppShell (always renders header + main)
|   |-- page.tsx            # / -- BntoGallery (tool grid for all users)
|   |-- workflows/          # Private (PROTECTED_PATHS -> redirect)
|   |-- executions/         # Private (PROTECTED_PATHS -> redirect)
|   +-- settings/           # Private (PROTECTED_PATHS -> redirect)
|-- [bnto]/                 # Tool pages -- public, SSR-safe, own layout
|   |-- layout.tsx          # Simplified header (logo + NavUser island)
|   +-- page.tsx            # Per-slug static generation + metadata
|-- (auth)/                 # Auth flows -- no auth gate
|   |-- layout.tsx          # Suspense wrapper
|   +-- signin/, signup/, waitlist/
```

## Two-Layer Auth

### Layer 1: Proxy (route protection)

`proxy.ts` runs on the server before HTML is sent. Lightweight cookie-presence check (not full session validation -- Convex validates at the data layer).

Two rules:
- **Auth on `/signin`** -> redirect to `/` (skipped when signout signal cookie is set)
- **Unauth on private route** -> redirect to `/signin`
- Everything else -> pass through

No flash possible -- the redirect happens before any HTML reaches the browser.

### Layer 2: App shell (UI concern)

The `(app)/layout.tsx` always renders the full AppShell (header + main). **No auth branching in the layout.** Every user -- authenticated or anonymous -- sees the same shell.

NavUser handles the auth-aware piece: authenticated users see a dropdown with account/sign-out; unauthenticated users see a "Sign in" link.

## How It Works

### Home page (/)

```
Any user visits /
  -> proxy.ts: / is not protected, pass through
  -> (app)/layout.tsx renders AppShell (header + main)
  -> (app)/page.tsx renders BntoGallery (tool grid)
```

### Bnto tool pages (/compress-images, /clean-csv, etc.)

```
Any user visits /compress-images
  -> proxy.ts: not a protected path, pass through
  -> [bnto]/layout.tsx renders simplified header (SSR-safe)
  -> [bnto]/page.tsx renders tool page with metadata + JSON-LD
```

### Private routes (workflows, settings)

```
Unauth user visits /workflows
  -> proxy.ts: no session cookie + /workflows is private -> redirect to /signin
  -> User never sees /workflows at all
```

### Auth user on /signin

```
Auth user visits /signin directly
  -> proxy.ts: session cookie present + no signout signal -> redirect to /
  -> User never sees /signin
```

### Mid-session auth loss

```
User's session expires while browsing /settings
  -> SessionProvider detects auth -> unauth transition
  -> fires onSessionLost callback
  -> Providers handler calls router.replace("/signin")
```

### Sign-out (instant, no await)

```
User clicks Sign Out
  -> signOut() sets bnto-signout signal cookie (non-HttpOnly, 10s TTL)
  -> signOut() clears React Query cache (client state invalidated)
  -> signOut() fires authSignOut() in background (server session cleanup)
  -> router.replace("/signin") navigates immediately
  -> proxy.ts sees session cookie BUT also sees bnto-signout signal -> skips auth-on-signin redirect
  -> User sees /signin instantly, no delay
  -> Server clears session cookie in background
  -> bnto-signout cookie expires after 10 seconds
```

**Why the signal cookie?** The session cookie is HttpOnly -- JavaScript can't delete it. Only the server can clear it via `Set-Cookie` on the `/api/auth/sign-out` response. But we don't want to `await` that network call because it adds noticeable delay. The signal cookie bridges client-side intent ("I'm signing out") with server-side proxy logic ("let them through to /signin despite having a session cookie").

## Key Files

| File | Responsibility |
|---|---|
| `proxy.ts` | Route protection -- runs before render. Cookie-presence check. |
| `app/(app)/layout.tsx` | AppShell wrapper (always renders header, no auth logic) |
| `app/(app)/_components/AppShell.tsx` | Header + main layout shell |
| `app/(app)/_components/NavUser.tsx` | Auth-aware: dropdown (auth) or "Sign in" link (unauth) |
| `app/[bnto]/layout.tsx` | SSR-safe tool page layout with simplified header |
| `app/providers/index.tsx` | Wires `onSessionLost` -> router redirect to /signin |
| `core/providers/SessionProvider.tsx` | Detects auth -> unauth transition, fires `onSessionLost` |
| `lib/routes.ts` | `isAuthPath()`, `isProtectedPath()`, route definitions |
| `lib/bnto-registry.ts` | `isValidBntoSlug()`, `getBntoBySlug()`, all predefined slugs |
| `core/lib/signoutSignal.ts` | Sets the `bnto-signout` signal cookie |
| `core/constants.ts` | `SIGNOUT_COOKIE` constant shared between core and proxy |

## Rules

### No returnTo redirects

We don't redirect unauth users to `/signin?returnTo=...`. If you're not signed in, you land on `/signin`. After signing in, you land on `/`.

### No auth checks in individual pages

```tsx
// BAD -- page checks auth
export default function WorkflowsPage() {
  const { isAuthenticated } = useAuth();
  if (!isAuthenticated) redirect("/signin");
  return ...;
}

// GOOD -- proxy handles it, pages trust the boundary
export default function WorkflowsPage() {
  return <WorkflowsContent />;
}
```

### No auth redirects in page components

All auth routing is handled by the proxy. Pages render their content and trust the boundary. Mid-session auth loss is detected by `SessionProvider` inside `@bnto/core`, which fires the `onSessionLost` callback -- the app wires this to `router.replace("/signin")` in `Providers`.

## Route Definitions

Single source of truth in `lib/routes.ts`:

```typescript
// lib/routes.ts
export const AUTH_PATHS = ["/signin", "/signup"];      // redirect away if authenticated
export const PROTECTED_PATHS = ["/workflows", "/executions", "/settings"]; // require auth
```

Everything else passes through -- bnto slugs, home page, unknown paths (404 at page level).

When adding a new protected route, add it to `PROTECTED_PATHS`. When adding a new auth-only route, add it to `AUTH_PATHS`. Public routes need no changes -- public-by-default.
