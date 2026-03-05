import type { ReactNode } from "react";

import { AppShell } from "@bnto/ui";
import { Navbar } from "@/components/blocks/Navbar";

/**
 * Editor layout — full viewport, no footer.
 *
 * Lives outside `(app)` to avoid the default Navbar + Footer shell.
 * The editor needs maximum vertical space — navbar stays for navigation
 * but footer is omitted. `pt-20` clears the fixed navbar (same as
 * AppShell.Main default clearance).
 */
export default function EditorLayout({ children }: { children: ReactNode }) {
  return (
    <div className="flex h-dvh flex-col">
      <AppShell.Header>
        <Navbar />
      </AppShell.Header>
      <main className="flex-1 overflow-hidden">{children}</main>
    </div>
  );
}
