"use client";

import { usePathname, useRouter } from "next/navigation";
import { useCallback } from "react";
import { BntoCoreProvider, useIsAuthenticated } from "@bnto/core";
import { AppHeader } from "./AppHeader";

/**
 * Inner shell — decides whether to show the top nav.
 *
 * Separated from AppShell so the auth hook runs inside BntoCoreProvider.
 */
function ShellLayout({ children }: { children: React.ReactNode }) {
  const isAuthenticated = useIsAuthenticated();
  const pathname = usePathname();

  // Unauth on / → passthrough (landing page has its own layout)
  if (!isAuthenticated && pathname === "/") {
    return <>{children}</>;
  }

  // Auth → full shell with top nav
  return (
    <div className="flex min-h-svh flex-col">
      <AppHeader />
      <main className="flex flex-1 flex-col">{children}</main>
    </div>
  );
}

/**
 * App shell with providers.
 *
 * Wraps children in BntoCoreProvider + shell layout.
 * Loaded via ssr: false in the (app) layout to avoid prerender crashes.
 */
export function AppShell({ children }: { children: React.ReactNode }) {
  const router = useRouter();

  const handleSessionLost = useCallback(() => {
    router.replace("/signin");
  }, [router]);

  return (
    <BntoCoreProvider onSessionLost={handleSessionLost}>
      <ShellLayout>{children}</ShellLayout>
    </BntoCoreProvider>
  );
}
