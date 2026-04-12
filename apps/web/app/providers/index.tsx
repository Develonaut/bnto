"use client";

import { usePathname } from "next/navigation";
import { BntoCoreProvider, TelemetryProvider } from "@bnto/core";

interface ProvidersProps {
  children: React.ReactNode;
}

/**
 * Root provider stack for the web app.
 *
 * Provider order:
 *   TelemetryProvider (PostHog init + page views) -> BntoCoreProvider (Convex + RQ + session)
 *
 * Server-side auth token is managed by ConvexAuthNextjsServerProvider
 * in the root layout (server component).
 */
export function Providers({ children }: ProvidersProps) {
  const pathname = usePathname();

  return (
    <TelemetryProvider
      apiKey={process.env.NEXT_PUBLIC_POSTHOG_KEY}
      host={process.env.NEXT_PUBLIC_POSTHOG_HOST}
      pathname={pathname}
    >
      <BntoCoreProvider>{children}</BntoCoreProvider>
    </TelemetryProvider>
  );
}
