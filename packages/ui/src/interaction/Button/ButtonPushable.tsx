/** Button rendering path for standard three-span pushable DOM. */

import type { ElementType, ReactNode } from "react";

import type { ElevationOverride } from "../resolveElevationClass";
import type { ButtonVariant } from "./buttonVariants";
import { resolvePushableClasses } from "./resolvePushableClasses";

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
  const cls = resolvePushableClasses(
    isIcon,
    resolvedSize,
    resolvedVariant,
    resolvedElevation,
    fullWidth,
    className,
  );
  return (
    <Comp {...sharedProps} className={cls.outer}>
      <span className="pushable-shadow" aria-hidden="true" />
      <span className="pushable-edge" aria-hidden="true" />
      <span className={cls.face}>{content}</span>
    </Comp>
  );
}
