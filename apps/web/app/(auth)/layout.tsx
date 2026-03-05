import { Suspense } from "react";
import type { ReactNode } from "react";

import { AppShellMain } from "@bnto/ui";

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <AppShellMain clearance="none">
      <Suspense>{children}</Suspense>
    </AppShellMain>
  );
}
