"use client";

import { useRouter } from "next/navigation";
import { usePathname } from "next/navigation";
import { useCallback, useEffect, useRef } from "react";
import { BntoCoreProvider, TelemetryProvider } from "@bnto/core";
import { isAuthPath } from "@/lib/routes";

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
  const router = useRouter();
  const pathname = usePathname();
  // Ref tracks latest pathname so handleSessionLost can read it without
  // being recreated on every navigation (keeps BntoCoreProvider stable).
  const pathnameRef = useRef(pathname);
  useEffect(() => {
    pathnameRef.current = pathname;
  }, [pathname]);

  const handleSessionLost = useCallback(() => {
    // During sign-up/sign-in, the auth session briefly drops as the token
    // transitions. Don't redirect to /signin if already on an auth page —
    // the session drop is expected.
    if (isAuthPath(pathnameRef.current)) return;
    router.replace("/signin");
  }, [router]);

  return (
    <TelemetryProvider
      apiKey={process.env.NEXT_PUBLIC_POSTHOG_KEY}
      host={process.env.NEXT_PUBLIC_POSTHOG_HOST}
      pathname={pathname}
    >
      <BntoCoreProvider onSessionLost={handleSessionLost}>
        {children}
      </BntoCoreProvider>
    </TelemetryProvider>
  );
}
