import type { Metadata } from "next";
import { AppShellHeader, AppShellMain } from "@bnto/ui";
import { Navbar } from "@/components/blocks/Navbar";

export const metadata: Metadata = {
  robots: { index: false, follow: false },
};

export default function DevLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <AppShellHeader>
        <Navbar />
      </AppShellHeader>
      <AppShellMain>{children}</AppShellMain>
    </>
  );
}
