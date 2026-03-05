import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";

/** A thin horizontal line to separate groups of items. */
export function MenuSeparator({ className, ...props }: ComponentProps<"div">) {
  return <div role="separator" className={cn("my-1 h-px bg-border", className)} {...props} />;
}
