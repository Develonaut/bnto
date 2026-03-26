/** Resolve the merged classNames for the pushable three-span Button path. */

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

export function resolvePushableClasses(
  isIcon: boolean,
  resolvedSize: string,
  resolvedVariant: ButtonVariant | undefined,
  resolvedElevation: ElevationOverride,
  fullWidth: boolean,
  className?: string,
) {
  const elevationClass = resolveElevationClass(resolvedElevation);
  const variantClass = resolvedVariant ? VARIANT_CLASSES[resolvedVariant] : undefined;
  const sizeElevation = isIcon
    ? ICON_ELEVATION_BY_SIZE[resolvedSize]
    : TEXT_ELEVATION_BY_SIZE[resolvedSize];

  const container = cn(
    "pushable",
    fullWidth ? "flex w-full" : "inline-flex",
    variantClass,
    elevationClass ?? sizeElevation,
  );
  const radius = RADIUS_BY_SIZE[resolvedSize] ?? "rounded-md";
  const face = isIcon
    ? iconFaceCn({ size: resolvedSize })
    : cn("flex-1 min-w-0", textFaceCn({ size: resolvedSize }));

  return {
    outer: cn(container, radius, className),
    face: cn("pushable-face", face, className),
  };
}
