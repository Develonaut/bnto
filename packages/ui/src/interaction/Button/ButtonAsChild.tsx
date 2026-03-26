/** Button rendering path for asChild / as — single-element (no pushable wrapper). */

import type { ElementType } from "react";

import type { ElevationOverride } from "../resolveElevationClass";
import type { ButtonVariant } from "./buttonVariants";
import { resolveAsChildClasses } from "./resolveAsChildClasses";

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

export function ButtonAsChild(props: AsChildRenderProps) {
  const { Comp, sharedProps, content } = props;
  const mergedClassName = resolveAsChildClasses(props);

  return (
    <Comp {...sharedProps} className={mergedClassName}>
      {content}
    </Comp>
  );
}
