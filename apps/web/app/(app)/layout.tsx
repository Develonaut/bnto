"use client";

import { useAuth, useIsWhitelisted } from "@bento/auth";
import { Card, CardHeader, CardTitle, CardContent } from "@bento/ui";

export default function AppLayout({ children }: { children: React.ReactNode }) {
  const { isLoading, isAuthenticated } = useAuth();
  const isWhitelisted = useIsWhitelisted();

  if (isLoading || isWhitelisted === undefined) {
    return (
      <main className="flex min-h-screen items-center justify-center bg-background text-foreground">
        <p className="text-muted-foreground">Loading...</p>
      </main>
    );
  }

  if (!isAuthenticated) {
    return null;
  }

  if (!isWhitelisted) {
    return (
      <main className="flex min-h-screen flex-col items-center justify-center bg-background text-foreground">
        <Card className="text-center">
          <CardHeader>
            <CardTitle className="text-2xl">You're on the list</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              We'll let you know when your account is ready.
            </p>
          </CardContent>
        </Card>
      </main>
    );
  }

  return <>{children}</>;
}
