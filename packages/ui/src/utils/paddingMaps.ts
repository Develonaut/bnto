/** Padding maps — maps t-shirt sizes to Tailwind padding utilities. */

import type { GapSize } from "./resolveGap";

export const paddingMap: Record<GapSize, string> = {
  xs: "p-1.5",
  sm: "p-2",
  md: "p-4",
  lg: "p-8",
  xl: "p-12",
};

export const paddingXMap: Record<GapSize, string> = {
  xs: "px-1.5",
  sm: "px-2",
  md: "px-4",
  lg: "px-8",
  xl: "px-12",
};

export const paddingYMap: Record<GapSize, string> = {
  xs: "py-1.5",
  sm: "py-2",
  md: "py-4",
  lg: "py-8",
  xl: "py-12",
};
