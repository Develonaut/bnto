"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ConvexQueryClient } from "@convex-dev/react-query";
import { ConvexAuthNextjsProvider } from "@convex-dev/auth/nextjs";
import { ConvexReactClient } from "convex/react";
import { useMemo } from "react";

/**
 * Provides the Convex client, React Query, and auth context for the app.
 *
 * Must be rendered inside BntoAuthProvider (which handles server-side auth
 * state). Creates a ConvexReactClient and wires it through:
 *   ConvexAuthNextjsProvider → ConvexQueryClient → QueryClientProvider
 *
 * This makes useConvex(), useConvexAuth(), and all @bnto/core hooks work.
 */
export function BntoCoreProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { convexClient, queryClient } = useMemo(() => {
    const convex = new ConvexReactClient(
      process.env.NEXT_PUBLIC_CONVEX_URL!,
    );
    const convexQueryClient = new ConvexQueryClient(convex);
    const query = new QueryClient({
      defaultOptions: {
        queries: {
          queryKeyHashFn: convexQueryClient.hashFn(),
          queryFn: convexQueryClient.queryFn(),
        },
      },
    });
    convexQueryClient.connect(query);
    return { convexClient: convex, queryClient: query };
  }, []);

  return (
    <ConvexAuthNextjsProvider client={convexClient}>
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    </ConvexAuthNextjsProvider>
  );
}
