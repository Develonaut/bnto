/** Align — maps alignment values to Tailwind items-* utilities. */

export type Align = "start" | "center" | "end" | "stretch" | "baseline";

export const alignMap: Record<Align, string> = {
  start: "items-start",
  center: "items-center",
  end: "items-end",
  stretch: "items-stretch",
  baseline: "items-baseline",
};
