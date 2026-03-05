import { Suspense } from "react";

import { AppShell } from "@bnto/ui";

export default function AuthLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <AppShell.Main clearance="none">
      <Suspense>{children}</Suspense>
    </AppShell.Main>
  );
}
