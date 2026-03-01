import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

import type { LayoutElement } from "./layoutTypes";

type CenterProps = HTMLAttributes<HTMLElement> & {
  /** Use inline-flex instead of flex. */
  inline?: boolean;
  /** Render as a different element. Default `"div"`. */
  as?: LayoutElement;
};

export function Center({
  inline,
  as: Tag = "div",
  className,
  ...props
}: CenterProps) {
  return (
    <Tag
      className={cn(
        inline ? "inline-flex" : "flex",
        "items-center justify-center",
        className,
      )}
      {...props}
    />
  );
}
