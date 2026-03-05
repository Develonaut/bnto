import type { ComponentProps, ElementType } from "react";

import { cn } from "../../utils/cn";

export function EmptyStateAction({
  children,
  className,
  as: Tag = "div",
  ...props
}: ComponentProps<"div"> & { as?: ElementType }) {
  return (
    <Tag className={cn("mt-1", className)} {...props}>
      {children}
    </Tag>
  );
}
