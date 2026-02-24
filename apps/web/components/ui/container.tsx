import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

type ContainerSize = "sm" | "md" | "lg" | "xl" | "full";

const sizeClasses: Record<ContainerSize, string> = {
  sm: "max-w-2xl",
  md: "max-w-5xl",
  lg: "max-w-7xl",
  xl: "max-w-screen-2xl",
  full: "max-w-[1220px]",
};

type ContainerProps = ComponentProps<"div"> & {
  /** Max-width constraint. Defaults to "full" (responsive breakpoint max-widths). */
  size?: ContainerSize;
  /** Render as a different element (e.g. "section", "nav"). */
  as?: "div" | "section" | "nav" | "main" | "footer" | "header";
};

export function Container({
  size = "full",
  as: Tag = "div",
  className,
  ...props
}: ContainerProps) {
  return (
    <Tag
      className={cn(
        "mx-auto w-full px-6",
        sizeClasses[size],
        className,
      )}
      {...props}
    />
  );
}
