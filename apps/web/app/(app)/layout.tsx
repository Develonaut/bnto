import type { ReactNode } from "react";

import { AppLayout } from "@/components/blocks/AppLayout";

export default function AppRouteLayout({ children }: { children: ReactNode }) {
  return <AppLayout>{children}</AppLayout>;
}
