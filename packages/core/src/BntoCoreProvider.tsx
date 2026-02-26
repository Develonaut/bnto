"use client";

import { QueryClientProvider } from "@tanstack/react-query";
// Pragmatic exception: BntoCoreProvider is Convex-specific bootstrap
// infrastructure — it wires the Convex provider, React Query, and session
// state together. When desktop ships (Wails adapter), this provider will be
// swapped for a Wails-specific variant. This is the adapter boundary for
// the provider stack, not a leak of Convex into business logic.
import { ConvexAuthNextjsProvider } from "@convex-dev/auth/nextjs";
import { getConvexClient, getQueryClient } from "./client";
import { SessionProvider } from "./providers/SessionProvider";

interface BntoCoreProviderProps {
  children: React.ReactNode;
  /**
   * Called when an authenticated session is lost (auth -> unauth transition).
   *
   * The app wires this to navigation (e.g., `router.replace("/signin")`).
   * Core fires the callback -- the app decides what to do with it.
   */
  onSessionLost?: () => void;
}

/**
 * Provides Convex, React Query, @convex-dev/auth, and session state for the app.
 *
 * Provider stack order:
 *   ConvexAuthNextjsProvider -> QueryClientProvider -> SessionProvider
 *
 * Must be wrapped by ConvexAuthNextjsServerProvider in the root layout
 * (server component) for server-side auth token management.
 */
export function BntoCoreProvider({
  children,
  onSessionLost,
}: BntoCoreProviderProps) {
  return (
    <ConvexAuthNextjsProvider client={getConvexClient()}>
      <QueryClientProvider client={getQueryClient()}>
        <SessionProvider onSessionLost={onSessionLost}>
          {children}
        </SessionProvider>
      </QueryClientProvider>
    </ConvexAuthNextjsProvider>
  );
}
