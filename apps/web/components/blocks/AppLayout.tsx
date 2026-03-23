/**
 * App shell layout — sidebar + content header + main + footer.
 *
 * Client component because mobile nav needs open/close state.
 * The (app)/layout.tsx server component renders this as a thin wrapper.
 */

"use client";

import { useCallback, useState } from "react";
import type { ReactNode } from "react";

import { AppSidebar } from "./AppSidebar";
import { ContentHeader } from "./ContentHeader";
import { Footer } from "./Footer";
import { MobileNavMenu } from "./MobileNavMenu";

interface AppLayoutProps {
  children: ReactNode;
}

export function AppLayout({ children }: AppLayoutProps) {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const openMobileMenu = useCallback(() => setMobileMenuOpen(true), []);

  return (
    <div className="flex h-dvh bg-background">
      <AppSidebar />
      <MobileNavMenu open={mobileMenuOpen} onOpenChange={setMobileMenuOpen} />
      <div className="isolate flex flex-1 flex-col overflow-y-auto">
        <ContentHeader onToggleMobileMenu={openMobileMenu} />
        <main className="flex flex-1 flex-col">{children}</main>
        <Footer />
      </div>
    </div>
  );
}
