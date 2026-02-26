import { Suspense } from "react";

import { AppShell } from "@/components/ui/AppShell";

export default function AuthLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <AppShell.Main>
      <Suspense>{children}</Suspense>
    </AppShell.Main>
  );
}
