import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

import {
  type Align,
  type Justify,
  type LayoutElement,
  type ResponsiveGap,
  alignMap,
  justifyMap,
  resolveGap,
} from "./layoutTypes";

type RowProps = HTMLAttributes<HTMLElement> & {
  /** Spacing between children. T-shirt size or responsive object. Default `"sm"`. */
  gap?: ResponsiveGap;
  /** Cross-axis alignment (`items-*`). Default `"center"`. */
  align?: Align;
  /** Main-axis alignment (`justify-*`). */
  justify?: Justify;
  /** Enable flex-wrap. */
  wrap?: boolean;
  /** Render as a different element. Default `"div"`. */
  as?: LayoutElement;
};

export function Row({
  gap = "sm",
  align = "center",
  justify,
  wrap,
  as: Tag = "div",
  className,
  ...props
}: RowProps) {
  return (
    <Tag
      className={cn(
        "flex flex-row",
        resolveGap(gap),
        alignMap[align],
        justify && justifyMap[justify],
        wrap && "flex-wrap",
        className,
      )}
      {...props}
    />
  );
}
