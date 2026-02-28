import type { ComponentProps, ElementType } from "react";

import { createCn } from "./createCn";

const containerCn = createCn({
  base: "mx-auto w-full px-4 sm:px-6",
  variants: {
    size: {
      sm: "max-w-2xl",
      md: "max-w-5xl",
      lg: "max-w-7xl",
      xl: "max-w-screen-2xl",
      full: "max-w-[1220px]",
    },
  },
  defaultVariants: {
    size: "full",
  },
});

type ContainerProps = ComponentProps<"div"> & {
  /** Max-width constraint. Defaults to "full" (responsive breakpoint max-widths). */
  size?: "sm" | "md" | "lg" | "xl" | "full";
  /** Render as a different element (e.g. "section", "nav"). */
  as?: ElementType;
};

export function Container({
  size,
  as: Tag = "div",
  className,
  ...props
}: ContainerProps) {
  return (
    <Tag
      className={containerCn({ size }, className)}
      {...props}
    />
  );
}
