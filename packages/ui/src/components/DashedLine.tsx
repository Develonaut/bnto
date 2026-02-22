import { cn } from "../lib/utils";

interface DashedLineProps {
  orientation?: "horizontal" | "vertical";
  className?: string;
}

/**
 * Decorative dashed divider line with faded edges.
 *
 * Uses repeating-linear-gradient for the dash pattern and
 * mask-image for the edge fade. Inherits text color for theming.
 */
export function DashedLine({
  orientation = "horizontal",
  className,
}: DashedLineProps) {
  const isHorizontal = orientation === "horizontal";

  return (
    <div
      role="separator"
      className={cn(
        "text-border",
        isHorizontal ? "h-px w-full" : "h-full w-px",
        className,
      )}
      style={{
        backgroundImage: isHorizontal
          ? "repeating-linear-gradient(to right, currentColor 0, currentColor 4px, transparent 4px, transparent 10px)"
          : "repeating-linear-gradient(to bottom, currentColor 0, currentColor 4px, transparent 4px, transparent 10px)",
        maskImage: isHorizontal
          ? "linear-gradient(to right, transparent, black 10%, black 90%, transparent)"
          : "linear-gradient(to bottom, transparent, black 10%, black 90%, transparent)",
      }}
    />
  );
}
