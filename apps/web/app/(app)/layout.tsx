import { Footer } from "@/components/blocks/footer";

export default function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <main>{children}</main>
      <Footer />
    </>
  );
}
