import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

import { Container } from "./container";

type ContainerSize = "sm" | "md" | "lg" | "xl" | "full";
type SectionSpacing = "sm" | "md" | "lg" | "xl";

const spacingMap: Record<SectionSpacing, string> = {
  sm: "py-12 lg:py-16",
  md: "py-20 lg:py-24",
  lg: "py-28 lg:py-32",
  xl: "py-32 lg:py-40",
};

type SectionProps = ComponentProps<"section"> & {
  /** Vertical padding. Default `"lg"`. */
  spacing?: SectionSpacing;
  /** Container max-width. Pass `false` to skip the Container wrapper. Default `"full"`. */
  container?: ContainerSize | false;
};

export function Section({
  spacing = "lg",
  container = "full",
  className,
  children,
  ...props
}: SectionProps) {
  return (
    <section className={cn(spacingMap[spacing], className)} {...props}>
      {container !== false ? (
        <Container size={container}>{children}</Container>
      ) : (
        children
      )}
    </section>
  );
}
