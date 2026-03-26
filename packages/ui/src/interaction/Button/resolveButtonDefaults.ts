import type { ElementType, ReactNode } from "react";

import type { SpringMode } from "../../surface/Pressable";
import type { ElevationOverride } from "../resolveElevationClass";
import type { ButtonVariant } from "./buttonVariants";

interface ButtonInputProps {
  variant?: ButtonVariant;
  size?: "sm" | "icon";
  icon?: ReactNode;
  as?: ElementType;
  asChild?: boolean;
  elevation?: ElevationOverride;
  spring?: SpringMode;
  fullWidth?: boolean;
  muted?: boolean;
  hovered?: boolean;
  pressed?: boolean;
  dormant?: boolean;
  toggle?: boolean;
  disabled?: boolean;
  children?: ReactNode;
}

export interface ResolvedButtonDefaults {
  elevation: ElevationOverride;
  spring: SpringMode;
  fullWidth: boolean;
  muted: boolean;
  hovered: boolean;
  pressed: boolean;
  dormant: boolean;
  toggle: boolean;
  asChild: boolean;
  isIcon: boolean;
  resolvedSize: string;
  content: ReactNode;
}

const BOOLEAN_FIELDS = [
  "fullWidth",
  "muted",
  "hovered",
  "pressed",
  "dormant",
  "toggle",
  "asChild",
] as const;

/** Apply defaults and resolve derived values from button props. */
export function resolveButtonDefaults(props: ButtonInputProps): ResolvedButtonDefaults {
  const booleans = resolveBooleanDefaults(props);
  const isIcon = props.icon !== undefined || props.size === "icon";

  return {
    elevation: props.elevation ?? true,
    spring: props.spring ?? "bounciest",
    ...booleans,
    isIcon,
    resolvedSize: props.size === "icon" ? "md" : (props.size ?? "md"),
    content: isIcon ? (props.icon ?? props.children) : props.children,
  };
}

/** Resolve all boolean props, defaulting to false. */
function resolveBooleanDefaults(props: ButtonInputProps) {
  const result: Record<string, boolean> = {};
  for (const key of BOOLEAN_FIELDS) {
    result[key] = props[key] ?? false;
  }
  return result as Pick<ResolvedButtonDefaults, (typeof BOOLEAN_FIELDS)[number]>;
}
