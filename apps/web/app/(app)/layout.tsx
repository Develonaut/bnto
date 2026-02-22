"use client";

import dynamic from "next/dynamic";

const AppShell = dynamic(
  () => import("./_components/AppShell").then((m) => m.AppShell),
  { ssr: false },
);

/**
 * App layout — wraps all (app) routes.
 *
 * Dynamically imports AppShell (ssr: false) because it uses Convex hooks
 * that require ConvexProvider to be mounted.
 */
export default function AppLayout({ children }: { children: React.ReactNode }) {
  return <AppShell>{children}</AppShell>;
}
