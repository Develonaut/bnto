"use client";

import { useCurrentUser, useSignOut } from "@bnto/auth";
import { Button } from "@bnto/ui";

export default function DashboardPage() {
  const user = useCurrentUser();
  const signOut = useSignOut();

  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-background text-foreground">
      <h1 className="text-3xl font-bold">Dashboard</h1>
      <p className="mt-4 text-muted-foreground">
        Signed in as {user?.email ?? "..."}
      </p>
      <Button variant="outline" onClick={() => signOut()} className="mt-8">
        Sign Out
      </Button>
    </main>
  );
}
