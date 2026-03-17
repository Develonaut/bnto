# @bnto/auth

Auth client — the abstraction boundary for authentication.

## Overview

`@bnto/auth` wraps the authentication provider (currently `@convex-dev/auth`) so that consumers never import auth infrastructure directly. If the auth provider changes, only this package needs updating.

Consumed by `@bnto/core` internals. App code accesses auth through `core.auth.*`.

## Directory Structure

```
src/
├── index.ts          # Client-side exports (hooks, provider)
├── server.ts         # Server-side auth utilities
├── client.ts         # Client-side auth utilities
├── middleware.ts      # Next.js middleware helper
└── hooks/
    ├── useSession.ts  # Session state
    ├── useSignIn.ts   # Sign-in action
    ├── useSignOut.ts  # Sign-out action
    └── useSignUp.ts   # Sign-up action
```

## Key Exports

**Client (from `@bnto/auth`):**

- `ConvexAuthNextjsProvider` — auth provider component
- `useAuthActions` — sign in/out actions
- `useAuthToken` — current auth token
- `useSession`, `useSignIn`, `useSignOut`, `useSignUp` — auth hooks

**Server (from `@bnto/auth/server`):**

- Server-side auth utilities for route protection

## Development

```bash
task ui:build       # TypeScript compilation
task ui:test        # Run tests
```
