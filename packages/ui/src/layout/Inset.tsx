import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

import type { LayoutElement } from "../utils/layoutElement";
import { paddingMap, paddingXMap, paddingYMap } from "../utils/paddingMaps";
import type { GapSize } from "../utils/resolveGap";

type InsetProps = HTMLAttributes<HTMLElement> & {
  /** Padding size. Uses the same token scale as gap. Default `"md"`. */
  size?: GapSize;
  /** Apply padding only on the horizontal axis. */
  horizontal?: boolean;
  /** Apply padding only on the vertical axis. */
  vertical?: boolean;
  /** Render as a different element. Default `"div"`. */
  as?: LayoutElement;
};

export function Inset({
  size = "md",
  horizontal,
  vertical,
  as: Tag = "div",
  className,
  ...props
}: InsetProps) {
  const padClass = horizontal ? paddingXMap[size] : vertical ? paddingYMap[size] : paddingMap[size];

  return <Tag className={cn(padClass, className)} {...props} />;
}
