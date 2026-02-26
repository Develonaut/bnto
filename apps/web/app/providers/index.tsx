"use client";

import { useRouter } from "next/navigation";
import { usePathname } from "next/navigation";
import { useCallback, useEffect, useRef } from "react";
import { BntoCoreProvider } from "@bnto/core";
import { isAuthPath } from "@/lib/routes";

interface ProvidersProps {
  children: React.ReactNode;
}

/**
 * Root provider stack for the web app.
 *
 * Wires BntoCoreProvider with:
 * - onSessionLost: navigate to /signin on mid-session auth loss
 *
 * Server-side auth token is managed by ConvexAuthNextjsServerProvider
 * in the root layout (server component).
 */
export function Providers({ children }: ProvidersProps) {
  const router = useRouter();
  const pathname = usePathname();
  const pathnameRef = useRef(pathname);
  // FIXME: This is a code smell please do research on the recommended
  // way to properly handle this case with next.js and better auth.
  useEffect(() => {
    pathnameRef.current = pathname;
  }, [pathname]);

  const handleSessionLost = useCallback(() => {
    // During sign-up/sign-in, the auth session briefly drops as the token
    // transitions (e.g., anonymous → password upgrade). Don't redirect to
    // /signin if already on an auth page — the session drop is expected.
    if (isAuthPath(pathnameRef.current)) return;
    router.replace("/signin");
  }, [router]);

  return (
    <BntoCoreProvider onSessionLost={handleSessionLost}>
      {children}
    </BntoCoreProvider>
  );
}
