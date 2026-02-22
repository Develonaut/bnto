"use client";

import { QueryClientProvider } from "@tanstack/react-query";
import { ConvexBetterAuthProvider } from "@convex-dev/better-auth/react";
import { authClient } from "@bnto/auth";
import { getConvexClient, getQueryClient } from "./client";
import { SessionProvider } from "./providers/SessionProvider";

interface BntoCoreProviderProps {
  children: React.ReactNode;
  /**
   * Server-side JWT for hydration.
   *
   * Fetched via `getToken()` in the root layout (Server Component) and passed
   * down to eliminate the flash of unauthenticated state on initial load.
   */
  initialToken?: string | null;
  /**
   * Called when an authenticated session is lost (auth -> unauth transition).
   *
   * The app wires this to navigation (e.g., `router.replace("/signin")`).
   * Core fires the callback -- the app decides what to do with it.
   */
  onSessionLost?: () => void;
}

/**
 * Provides Convex, React Query, Better Auth, and session state for the app.
 *
 * Provider stack order:
 *   ConvexBetterAuthProvider -> QueryClientProvider -> SessionProvider
 */
export function BntoCoreProvider({
  children,
  initialToken,
  onSessionLost,
}: BntoCoreProviderProps) {
  return (
    <ConvexBetterAuthProvider
      client={getConvexClient()}
      authClient={authClient}
      initialToken={initialToken ?? undefined}
    >
      <QueryClientProvider client={getQueryClient()}>
        <SessionProvider onSessionLost={onSessionLost}>
          {children}
        </SessionProvider>
      </QueryClientProvider>
    </ConvexBetterAuthProvider>
  );
}
