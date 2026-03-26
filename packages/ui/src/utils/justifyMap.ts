/** Justify — maps justification values to Tailwind justify-* utilities. */

export type Justify = "start" | "center" | "end" | "between" | "around" | "evenly";

export const justifyMap: Record<Justify, string> = {
  start: "justify-start",
  center: "justify-center",
  end: "justify-end",
  between: "justify-between",
  around: "justify-around",
  evenly: "justify-evenly",
};
