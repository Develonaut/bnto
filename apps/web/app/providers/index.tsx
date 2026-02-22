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
 * Wires BntoCoreProvider with session-loss handling:
 * when auth is lost mid-session, navigate to /signin.
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
