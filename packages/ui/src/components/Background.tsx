import { cn } from "../lib/utils";

interface BackgroundProps {
  variant?: "top" | "bottom";
  children: React.ReactNode;
  className?: string;
}

/**
 * Gradient background wrapper for hero and CTA sections.
 *
 * Uses the primary color at low opacity for a warm tinted gradient.
 * Outer margin matches the Mainline template spacing.
 */
export function Background({
  variant = "top",
  children,
  className,
}: BackgroundProps) {
  return (
    <div
      className={cn(
        "mx-2.5 lg:mx-4",
        variant === "top" && "mt-2.5 rounded-t-[2rem] bg-linear-to-b from-primary/20 via-background to-background",
        variant === "bottom" && "rounded-b-[2rem] bg-linear-to-b from-background via-background to-primary/20",
        className,
      )}
    >
      {children}
    </div>
  );
}
