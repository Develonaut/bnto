import type { ComponentProps, ElementType } from "react";

import { createCn } from "./createCn";

type HeadingLevel = 1 | 2 | 3 | 4 | 5 | 6;
type HeadingSize = "xs" | "sm" | "md" | "lg" | "xl" | "display";

const levelToTag: Record<HeadingLevel, `h${HeadingLevel}`> = {
  1: "h1",
  2: "h2",
  3: "h3",
  4: "h4",
  5: "h5",
  6: "h6",
};

const levelToSize: Record<HeadingLevel, HeadingSize> = {
  1: "display",
  2: "lg",
  3: "md",
  4: "sm",
  5: "xs",
  6: "xs",
};

const headingCn = createCn({
  base: "font-display font-black tracking-tight",
  variants: {
    size: {
      xs: "text-lg",
      sm: "text-xl",
      md: "text-xl md:text-2xl lg:text-3xl",
      lg: "text-2xl tracking-tight md:text-4xl lg:text-5xl",
      xl: "text-3xl md:text-4xl lg:text-5xl",
      display: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl",
    },
  },
  defaultVariants: {
    size: "lg",
  },
});

type HeadingProps = ComponentProps<"h1"> & {
  /** Semantic heading level. Determines the HTML tag (h1-h6). Required. */
  level: HeadingLevel;
  /** Typography size. Auto-derived from level if omitted. */
  size?: HeadingSize;
  /** Override the rendered HTML element. */
  as?: ElementType;
};

export function Heading({
  level,
  size,
  as,
  className,
  ...props
}: HeadingProps) {
  const Tag = as ?? levelToTag[level];
  const resolvedSize = size ?? levelToSize[level];
  return (
    <Tag
      className={headingCn({ size: resolvedSize }, className)}
      {...props}
    />
  );
}
