"use client";

import { useRouter } from "next/navigation";
import { useCallback } from "react";
import { BntoCoreProvider } from "@bnto/core";

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

  const handleSessionLost = useCallback(() => {
    router.replace("/signin");
  }, [router]);

  return (
    <BntoCoreProvider onSessionLost={handleSessionLost}>
      {children}
    </BntoCoreProvider>
  );
}
