import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

import { type Align, alignMap } from "../utils/alignMap";
import { type Justify, justifyMap } from "../utils/justifyMap";
import type { LayoutElement } from "../utils/layoutElement";
import { type ResponsiveGap, resolveGap } from "../utils/resolveGap";

type StackProps = HTMLAttributes<HTMLElement> & {
  /** Spacing between children. T-shirt size or responsive object. Default `"md"`. */
  gap?: ResponsiveGap;
  /** Cross-axis alignment (`items-*`). */
  align?: Align;
  /** Main-axis alignment (`justify-*`). */
  justify?: Justify;
  /** Render as a different element. Default `"div"`. */
  as?: LayoutElement;
};

export function Stack({
  gap = "md",
  align,
  justify,
  as: Tag = "div",
  className,
  ...props
}: StackProps) {
  return (
    <Tag
      className={cn(
        "flex flex-col",
        resolveGap(gap),
        align && alignMap[align],
        justify && justifyMap[justify],
        className,
      )}
      {...props}
    />
  );
}
