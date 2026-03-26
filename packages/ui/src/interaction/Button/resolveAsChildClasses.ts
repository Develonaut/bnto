/** Resolve the merged className for asChild / as rendering path. */

import type { ElementType } from "react";

import { cn } from "../../utils/cn";
import { resolveElevationClass, type ElevationOverride } from "../resolveElevationClass";
import { stripSizeElevation } from "../stripSizeElevation";
import { type ButtonVariant, VARIANT_CLASSES, textCn, iconCn } from "./buttonVariants";

interface AsChildClassInput {
  isIcon: boolean;
  resolvedSize: string;
  resolvedVariant: ButtonVariant | undefined;
  resolvedElevation: ElevationOverride;
  fullWidth: boolean;
  size: string | undefined;
  asChild: boolean;
  as: ElementType | undefined;
  className?: string;
}

export function resolveAsChildClasses(input: AsChildClassInput) {
  const elevationClass = resolveElevationClass(input.resolvedElevation);
  const variantClass = input.resolvedVariant ? VARIANT_CLASSES[input.resolvedVariant] : undefined;

  const behaviorCn = cn(
    "pressable outline-none surface",
    variantClass,
    elevationClass,
    input.fullWidth && "flex w-full",
  );
  const applySize = input.size !== undefined || (!input.asChild && !input.as);
  const sizeClasses = applySize
    ? input.isIcon
      ? iconCn({ variant: input.resolvedVariant, size: input.resolvedSize })
      : textCn({ variant: input.resolvedVariant, size: input.resolvedSize })
    : "";
  const resolvedSizeClasses = behaviorCn.includes("elevation-")
    ? stripSizeElevation(sizeClasses)
    : sizeClasses;

  return cn(behaviorCn, resolvedSizeClasses, input.className);
}
