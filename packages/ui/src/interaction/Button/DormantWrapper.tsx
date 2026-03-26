/** Wrap a dormant button with hover-zone padding. */

import type { ReactNode } from "react";

export function DormantWrapper({
  disabled,
  children,
}: {
  disabled?: boolean;
  children: ReactNode;
}) {
  if (disabled)
    return <span className="inline-flex opacity-50 pointer-events-none">{children}</span>;
  return <span className="group inline-flex p-4 -m-4">{children}</span>;
}
