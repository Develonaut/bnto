"use client";

import { AppHeader } from "./AppHeader";
import { Footer } from "../../_components/Footer";

export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <>
      <AppHeader />
      <main>{children}</main>
      <Footer />
    </>
  );
}
