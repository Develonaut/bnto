/** Button rendering path for asChild / as — single-element (no pushable wrapper). */

import type { ElementType } from "react";

import { cn } from "../../utils/cn";
import { resolveElevationClass, type ElevationOverride } from "../resolveElevationClass";
import { stripSizeElevation } from "../stripSizeElevation";
import { type ButtonVariant, VARIANT_CLASSES, textCn, iconCn } from "./buttonVariants";

interface AsChildRenderProps {
  Comp: ElementType;
  sharedProps: Record<string, unknown>;
  isIcon: boolean;
  resolvedSize: string;
  resolvedVariant: ButtonVariant | undefined;
  resolvedElevation: ElevationOverride;
  fullWidth: boolean;
  size: string | undefined;
  asChild: boolean;
  as: ElementType | undefined;
  className?: string;
  content: React.ReactNode;
}

export function ButtonAsChild({
  Comp,
  sharedProps,
  isIcon,
  resolvedSize,
  resolvedVariant,
  resolvedElevation,
  fullWidth,
  size,
  asChild,
  as,
  className,
  content,
}: AsChildRenderProps) {
  const elevationClass = resolveElevationClass(resolvedElevation);
  const variantClass = resolvedVariant ? VARIANT_CLASSES[resolvedVariant] : undefined;

  const behaviorCn = cn(
    "pressable outline-none surface",
    variantClass,
    elevationClass,
    fullWidth && "flex w-full",
  );
  const applySize = size !== undefined || (!asChild && !as);
  const sizeClasses = applySize
    ? isIcon
      ? iconCn({ variant: resolvedVariant, size: resolvedSize })
      : textCn({ variant: resolvedVariant, size: resolvedSize })
    : "";
  const resolvedSizeClasses = behaviorCn.includes("elevation-")
    ? stripSizeElevation(sizeClasses)
    : sizeClasses;

  return (
    <Comp {...sharedProps} className={cn(behaviorCn, resolvedSizeClasses, className)}>
      {content}
    </Comp>
  );
}
