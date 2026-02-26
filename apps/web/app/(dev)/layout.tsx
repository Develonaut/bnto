import { AppShell } from "@/components/ui/AppShell";

export default function DevLayout({ children }: { children: React.ReactNode }) {
  return <AppShell.Main>{children}</AppShell.Main>;
}
