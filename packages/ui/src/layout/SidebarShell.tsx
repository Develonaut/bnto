import type { ReactNode } from "react";

import { cn } from "../utils/cn";
import { Card } from "../surface/Card";

/**
 * SidebarShell — shared sidebar frame for app nav and editor.
 *
 * Owns the Card surface, uniform `p-4` inner padding, logo area
 * spacing (`mb-4`), mt-auto push-to-bottom, and footer separator.
 * Consumers fill the `header`, `children`, and `footer` slots.
 *
 * The shell ensures both contexts use identical spacing so
 * switching between editor and home feels like the content
 * changed, not the container.
 */

interface SidebarShellProps {
  /** Logo + top-level controls (rendered above content with mb-4 spacing). */
  header?: ReactNode;
  /** Main scrollable content area. */
  children?: ReactNode;
  /** Pinned footer (theme toggle, user avatar, external links). */
  footer?: ReactNode;
  className?: string;
}

/** Tailwind width class shared by AppSidebar and EditorLeftPanel. */
const SIDEBAR_WIDTH = "w-72";

function SidebarShell({ header, children, footer, className }: SidebarShellProps) {
  return (
    <Card elevation="md" className={cn("flex flex-1 flex-col", className)}>
      <div className="flex flex-1 flex-col overflow-y-auto p-4">
        {header && <div className="mb-4 shrink-0">{header}</div>}
        <div className="min-h-0 flex-1">{children}</div>
        {footer && <div className="mt-auto shrink-0 border-t border-border pt-3">{footer}</div>}
      </div>
    </Card>
  );
}

export { SidebarShell, SIDEBAR_WIDTH };
export type { SidebarShellProps };
