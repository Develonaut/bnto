"use client";

import dynamic from "next/dynamic";

const NavUser = dynamic(
  () => import("../../(app)/_components/NavUser").then((m) => m.NavUser),
  { ssr: false },
);

/**
 * Client island that lazy-loads NavUser.
 *
 * Used in the [bnto] layout (a server component) to keep the layout
 * SSR-safe while still rendering the auth-aware user dropdown.
 */
export function NavUserIsland() {
  return <NavUser />;
}
