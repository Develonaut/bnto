"use client";

import { useRouter, usePathname } from "next/navigation";
import { useCallback, useEffect, useRef } from "react";
import { BntoCoreProvider, TelemetryProvider } from "@bnto/core";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";
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

    // During explicit sign-out, the signout signal cookie is set before
    // the session drops. Don't add returnTo — signout code handles its own redirect.
    const isSigningOut =
      typeof document !== "undefined" &&
      document.cookie.split(";").some((c) => c.trim().startsWith(`${SIGNOUT_COOKIE}=`));
    if (isSigningOut) return;

    const returnTo = encodeURIComponent(pathnameRef.current);
    router.replace(`/signin?returnTo=${returnTo}`);
  }, [router]);

  return (
    <TelemetryProvider
      apiKey={process.env.NEXT_PUBLIC_POSTHOG_KEY}
      host={process.env.NEXT_PUBLIC_POSTHOG_HOST}
      pathname={pathname}
    >
      <BntoCoreProvider onSessionLost={handleSessionLost}>{children}</BntoCoreProvider>
    </TelemetryProvider>
  );
}
