"use client";

import { useSession } from "@bnto/auth";

interface AppGateProps {
  children: React.ReactNode;
}

/**
 * Gates the app behind provider readiness.
 *
 * Shows a splash screen until auth state resolves, preventing the flash
 * where unauthenticated users briefly see app UI before the proxy redirect
 * takes effect on the client side.
 *
 * Once auth state is determined (authenticated or not), renders children.
 * This component does NOT make auth decisions -- that's the proxy's job.
 * It only waits for the auth state to be known.
 */
export function AppGate({ children }: AppGateProps) {
  const { isPending } = useSession();

  if (isPending) {
    return <SplashScreen />;
  }

  return <>{children}</>;
}

/** Full-screen splash shown while auth state resolves. */
function SplashScreen() {
  return (
    <div className="fixed inset-0 flex items-center justify-center bg-background">
      <div className="flex flex-col items-center gap-4">
        <span className="text-2xl font-bold tracking-tight text-foreground">
          bnto
        </span>
        <div
          className="motion-safe:animate-pulse text-sm text-muted-foreground"
          role="status"
          aria-label="Loading"
        >
          Loading...
        </div>
      </div>
    </div>
  );
}
