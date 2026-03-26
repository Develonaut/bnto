/** Button rendering path for standard three-span pushable DOM. */

import type { ElementType, ReactNode } from "react";

import { cn } from "../../utils/cn";
import { resolveElevationClass, type ElevationOverride } from "../resolveElevationClass";
import {
  type ButtonVariant,
  VARIANT_CLASSES,
  RADIUS_BY_SIZE,
  ICON_ELEVATION_BY_SIZE,
  TEXT_ELEVATION_BY_SIZE,
  textFaceCn,
  iconFaceCn,
} from "./buttonVariants";

interface PushableRenderProps {
  Comp: ElementType;
  sharedProps: Record<string, unknown>;
  isIcon: boolean;
  resolvedSize: string;
  resolvedVariant: ButtonVariant | undefined;
  resolvedElevation: ElevationOverride;
  fullWidth: boolean;
  className?: string;
  content: ReactNode;
}

export function ButtonPushable({
  Comp,
  sharedProps,
  isIcon,
  resolvedSize,
  resolvedVariant,
  resolvedElevation,
  fullWidth,
  className,
  content,
}: PushableRenderProps) {
  const elevationClass = resolveElevationClass(resolvedElevation);
  const variantClass = resolvedVariant ? VARIANT_CLASSES[resolvedVariant] : undefined;
  const sizeElevation = isIcon
    ? ICON_ELEVATION_BY_SIZE[resolvedSize]
    : TEXT_ELEVATION_BY_SIZE[resolvedSize];
  const containerClasses = cn(
    "pushable",
    fullWidth ? "flex w-full" : "inline-flex",
    variantClass,
    elevationClass ?? sizeElevation,
  );
  const radiusClass = RADIUS_BY_SIZE[resolvedSize] ?? "rounded-md";
  const faceClasses = isIcon
    ? iconFaceCn({ size: resolvedSize })
    : cn("flex-1 min-w-0", textFaceCn({ size: resolvedSize }));

  return (
    <Comp {...sharedProps} className={cn(containerClasses, radiusClass, className)}>
      <span className="pushable-shadow" aria-hidden="true" />
      <span className="pushable-edge" aria-hidden="true" />
      <span className={cn("pushable-face", faceClasses, className)}>{content}</span>
    </Comp>
  );
}
