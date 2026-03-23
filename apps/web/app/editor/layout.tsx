import type { ReactNode } from "react";

/**
 * Editor layout — full viewport, no chrome.
 *
 * Lives outside `(app)` to avoid the default Navbar + Footer shell.
 * The editor owns all its own chrome (left panel, bottom toolbar,
 * right panels) — no site Navbar needed.
 */
export default function EditorLayout({ children }: { children: ReactNode }) {
  return <main className="h-dvh overflow-hidden">{children}</main>;
}
