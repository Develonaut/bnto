import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

import { type Align, alignMap } from "../utils/alignMap";
import { type Justify, justifyMap } from "../utils/justifyMap";
import type { LayoutElement } from "../utils/layoutElement";
import { type ResponsiveGap, resolveGap } from "../utils/resolveGap";

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
