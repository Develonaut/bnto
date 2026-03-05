import type { ComponentProps, ElementType } from "react";

import { createCn } from "../utils/createCn";

const textCn = createCn({
  base: "",
  variants: {
    size: {
      xs: "text-xs",
      sm: "text-sm",
      base: "text-base",
      lg: "text-lg",
      xl: "text-xl",
    },
    color: {
      default: "text-foreground",
      muted: "text-muted-foreground",
      primary: "text-primary",
      inherit: "text-inherit",
    },
    leading: {
      none: "leading-none",
      tight: "leading-tight",
      snug: "leading-snug",
      normal: "leading-normal",
      relaxed: "leading-relaxed",
      loose: "leading-loose",
    },
    weight: {
      normal: "font-normal",
      medium: "font-medium",
      semibold: "font-semibold",
      bold: "font-bold",
    },
    balance: {
      true: "text-balance",
    },
    mono: {
      true: "font-mono",
    },
  },
  defaultVariants: {
    size: "base",
    color: "default",
  },
});

type TextProps = ComponentProps<"p"> & {
  /** Font size. Default `"base"`. */
  size?: "xs" | "sm" | "base" | "lg" | "xl";
  /** Text color. Default `"default"`. */
  color?: "default" | "muted" | "primary" | "inherit";
  /** Line height. */
  leading?: "none" | "tight" | "snug" | "normal" | "relaxed" | "loose";
  /** Font weight. */
  weight?: "normal" | "medium" | "semibold" | "bold";
  /** Apply `text-balance` for optical line-breaking. */
  balance?: boolean;
  /** Use monospace font. */
  mono?: boolean;
  /** Override the rendered HTML element. Default `"p"`. */
  as?: ElementType;
};

export function Text({
  size,
  color,
  leading,
  weight,
  balance,
  mono,
  as: Tag = "p",
  className,
  ...props
}: TextProps) {
  return (
    <Tag
      className={textCn({ size, color, leading, weight, balance, mono }, className)}
      {...props}
    />
  );
}
