"use client";

import dynamic from "next/dynamic";

const ThemeToggle = dynamic(
  () => import("@bnto/ui/theme-toggle").then((m) => m.ThemeToggle),
  { ssr: false },
);

/**
 * Client island that lazy-loads ThemeToggle.
 *
 * Used in the [bnto] layout (a server component) to keep the layout
 * SSR-safe while still rendering the theme toggle button.
 */
export function ThemeToggleIsland() {
  return <ThemeToggle />;
}
