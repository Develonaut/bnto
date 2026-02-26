import { AppShell } from "@/components/ui/AppShell";
import { Footer } from "@/components/blocks/Footer";

export default function BntoLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <AppShell.Main>{children}</AppShell.Main>
      <Footer />
    </>
  );
}
