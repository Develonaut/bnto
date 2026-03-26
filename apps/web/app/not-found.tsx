import { AppShellHeader, AppShellMain, AppShellContent } from "@bnto/ui";

import { Navbar } from "@/components/blocks/Navbar";
import { Footer } from "@/components/blocks/Footer";
import { NotFoundContent } from "./NotFoundContent";

/**
 * Root 404 page — self-contained with Navbar + Footer.
 *
 * In production (Vercel), `dynamicParams = false` 404s render inside the
 * (app) layout automatically. In dev mode, Next.js bypasses the route
 * group layout and renders this root not-found directly. Including the
 * full shell here ensures consistent rendering in both environments.
 */
export default function NotFound() {
  return (
    <>
      <AppShellHeader>
        <Navbar />
      </AppShellHeader>
      <AppShellMain>
        <AppShellContent>
          <NotFoundContent />
        </AppShellContent>
      </AppShellMain>
      <Footer />
    </>
  );
}
