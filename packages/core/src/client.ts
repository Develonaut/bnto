"use client";

import { QueryClient, QueryCache, MutationCache } from "@tanstack/react-query";
// Pragmatic exception: client.ts is Convex-specific bootstrap infrastructure.
// It creates the singleton Convex + React Query clients that the entire adapter
// layer depends on. This is the root of the Convex adapter tree — when desktop
// ships, it will be replaced with a Wails-specific client module.
import { ConvexQueryClient } from "@convex-dev/react-query";
import { ConvexReactClient } from "convex/react";
import { emitAuthError, isAuthError } from "./authError";

// ---------------------------------------------------------------------------
// Lazy singletons — created once on first access (browser only).
//
// Can't be eagerly created at module scope because Next.js evaluates
// "use client" modules during SSR/prerendering when env vars may be missing.
// ---------------------------------------------------------------------------

let convexClient: ConvexReactClient;
let convexQueryClient: ConvexQueryClient;
let queryClient: QueryClient;

function ensureClients() {
  if (!convexClient) {
    const url = process.env.NEXT_PUBLIC_CONVEX_URL;
    if (!url) {
      throw new Error(
        "NEXT_PUBLIC_CONVEX_URL is not set. " +
          "Run `npx convex dev` and ensure .env.local is populated.",
      );
    }
    convexClient = new ConvexReactClient(url);
    convexQueryClient = new ConvexQueryClient(convexClient);
    queryClient = new QueryClient({
      queryCache: new QueryCache({
        onError: (error) => {
          if (isAuthError(error)) emitAuthError();
        },
      }),
      mutationCache: new MutationCache({
        onError: (error) => {
          if (isAuthError(error)) emitAuthError();
        },
      }),
      defaultOptions: {
        queries: {
          queryKeyHashFn: convexQueryClient.hashFn(),
          queryFn: convexQueryClient.queryFn(),
        },
      },
    });
    convexQueryClient.connect(queryClient);
  }
  return { convexClient, convexQueryClient, queryClient };
}

/** Get the Convex client singleton (for imperative mutations). */
export function getConvexClient() {
  return ensureClients().convexClient;
}

/** Get the React Query client singleton (for cache invalidation). */
export function getQueryClient() {
  return ensureClients().queryClient;
}
