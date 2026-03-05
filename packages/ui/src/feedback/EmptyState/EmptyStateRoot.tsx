import type { ComponentProps } from "react";

import { createCn } from "../../utils/createCn";

export type EmptyStateSize = "sm" | "md" | "lg";

const rootCn = createCn({
  base: "flex flex-col items-center text-center",
  variants: {
    size: {
      sm: "gap-2 py-6",
      md: "gap-3 py-10",
      lg: "gap-4 py-16",
    },
  },
  defaultVariants: { size: "md" },
});

export type EmptyStateRootProps = ComponentProps<"div"> & {
  /** Overall size variant. Controls spacing, icon size, and padding. Default `"md"`. */
  size?: EmptyStateSize;
};

export function EmptyStateRoot({ size = "md", className, ...props }: EmptyStateRootProps) {
  return <div className={rootCn({ size }, className)} {...props} />;
}
