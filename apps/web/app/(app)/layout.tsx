import { AppShell } from "@bnto/ui";
import { Navbar } from "@/components/blocks/Navbar";
import { Footer } from "@/components/blocks/Footer";

export default function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <AppShell.Header>
        <Navbar />
      </AppShell.Header>
      <AppShell.Main>{children}</AppShell.Main>
      <Footer />
    </>
  );
}
