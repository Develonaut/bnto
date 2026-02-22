"use client";

import { useRouter } from "next/navigation";
import { useCallback } from "react";
import { BntoCoreProvider } from "@bnto/core";

interface ProvidersProps {
  children: React.ReactNode;
  initialToken?: string | null;
}

/**
 * Root provider stack for the web app.
 *
 * Wires BntoCoreProvider with:
 * - initialToken: server-side JWT for hydration (no flash of unauth state)
 * - onSessionLost: navigate to /signin on mid-session auth loss
 */
export function Providers({ children, initialToken }: ProvidersProps) {
  const router = useRouter();

  const handleSessionLost = useCallback(() => {
    router.replace("/signin");
  }, [router]);

  return (
    <BntoCoreProvider
      initialToken={initialToken}
      onSessionLost={handleSessionLost}
    >
      {children}
    </BntoCoreProvider>
  );
}
