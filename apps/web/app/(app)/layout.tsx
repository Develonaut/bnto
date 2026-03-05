import type { ReactNode } from "react";

import { AppShellHeader, AppShellMain } from "@bnto/ui";
import { Navbar } from "@/components/blocks/Navbar";
import { Footer } from "@/components/blocks/Footer";

export default function AppLayout({ children }: { children: ReactNode }) {
  return (
    <>
      <AppShellHeader>
        <Navbar />
      </AppShellHeader>
      <AppShellMain>{children}</AppShellMain>
      <Footer />
    </>
  );
}
