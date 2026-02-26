import type { Metadata } from "next";
import { AppShell } from "@/components/ui/AppShell";

export const metadata: Metadata = {
  robots: { index: false, follow: false },
};

export default function DevLayout({ children }: { children: React.ReactNode }) {
  return <AppShell.Main>{children}</AppShell.Main>;
}
