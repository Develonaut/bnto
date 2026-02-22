"use client";

import { AppHeader } from "./AppHeader";
import { Footer } from "../../_components/Footer";

/**
 * App shell — always renders header + main wrapper + footer.
 *
 * Every user (auth or unauth) sees the full app shell.
 * Loaded via ssr: false in the (app) layout because AppHeader
 * uses Convex hooks via NavUser.
 */
export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex min-h-svh flex-col">
      <AppHeader />
      <main className="flex flex-1 flex-col pt-24">{children}</main>
      <Footer />
    </div>
  );
}
