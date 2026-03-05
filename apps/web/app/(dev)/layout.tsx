import type { Metadata } from "next";
import { AppShell } from "@bnto/ui";
import { Navbar } from "@/components/blocks/Navbar";

export const metadata: Metadata = {
  robots: { index: false, follow: false },
};

export default function DevLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <AppShell.Header>
        <Navbar />
      </AppShell.Header>
      <AppShell.Main>{children}</AppShell.Main>
    </>
  );
}
