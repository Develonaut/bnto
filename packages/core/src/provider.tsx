"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ConvexQueryClient } from "@convex-dev/react-query";
import { ConvexBetterAuthProvider } from "@convex-dev/better-auth/react";
import { ConvexReactClient } from "convex/react";
import { authClient } from "@bnto/auth";
import { SessionProvider } from "./providers/SessionProvider";

// Lazy singletons — created once on first access (browser only).
// Can't be eagerly created at module scope because Next.js evaluates
// "use client" modules during SSR/prerendering when env vars may be missing.
let convexClient: ConvexReactClient;
let queryClient: QueryClient;

function getClients() {
  if (!convexClient) {
    const url = process.env.NEXT_PUBLIC_CONVEX_URL;
    if (!url) {
      throw new Error(
        "NEXT_PUBLIC_CONVEX_URL is not set. " +
        "Run `npx convex dev` and ensure .env.local is populated.",
      );
    }
    convexClient = new ConvexReactClient(url);
    const convexQueryClient = new ConvexQueryClient(convexClient);
    queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          queryKeyHashFn: convexQueryClient.hashFn(),
          queryFn: convexQueryClient.queryFn(),
        },
      },
    });
    convexQueryClient.connect(queryClient);
  }
  return { convexClient, queryClient };
}

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
 *
 * - ConvexBetterAuthProvider: wires Convex client with Better Auth token fetching
 * - QueryClientProvider: React Query with Convex query bridging
 * - SessionProvider: tracks auth status, detects session loss
 */
export function BntoCoreProvider({
  children,
  initialToken,
  onSessionLost,
}: BntoCoreProviderProps) {
  const clients = getClients();

  return (
    <ConvexBetterAuthProvider
      client={clients.convexClient}
      authClient={authClient}
      initialToken={initialToken ?? undefined}
    >
      <QueryClientProvider client={clients.queryClient}>
        <SessionProvider onSessionLost={onSessionLost}>
          {children}
        </SessionProvider>
      </QueryClientProvider>
    </ConvexBetterAuthProvider>
  );
}
