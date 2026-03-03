"use client";

import { QueryClientProvider } from "@tanstack/react-query";
import { ConvexAuthNextjsProvider } from "@bnto/auth";
import { getConvexClient, getQueryClient } from "./client";
import { SessionProvider } from "./providers/SessionProvider";
import { useHistorySync } from "./hooks/useHistorySync";

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

/** Syncs execution history between local (IndexedDB) and server (Convex). */
function HistoryProvider({ children }: { children: React.ReactNode }) {
  useHistorySync();
  return <>{children}</>;
}

/**
 * Provides Convex, React Query, auth, and session state for the app.
 *
 * Provider stack order:
 *   ConvexAuthNextjsProvider -> QueryClientProvider -> SessionProvider -> HistoryProvider
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
          <HistoryProvider>{children}</HistoryProvider>
        </SessionProvider>
      </QueryClientProvider>
    </ConvexAuthNextjsProvider>
  );
}
