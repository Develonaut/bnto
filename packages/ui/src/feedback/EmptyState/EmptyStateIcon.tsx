import type { ReactNode } from "react";

import { cn } from "../../utils/cn";

import type { EmptyStateSize } from "./EmptyStateRoot";

const iconSizeMap: Record<EmptyStateSize, string> = {
  sm: "size-8",
  md: "size-10",
  lg: "size-12",
};

export function EmptyStateIcon({
  children,
  size = "md",
  className,
}: {
  children: ReactNode;
  size?: EmptyStateSize;
  className?: string;
}) {
  return (
    <div className={cn("text-muted-foreground [&>svg]:size-full", iconSizeMap[size], className)}>
      {children}
    </div>
  );
}
