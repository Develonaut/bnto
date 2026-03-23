import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

/** Flexible spacer that fills available space along the parent's main axis. */
export function Spacer({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div className={cn("flex-1", className)} {...props} />;
}
