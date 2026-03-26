/** Button component -- three-span pushable with variant and size resolution. */

import type { ComponentProps, ReactNode, Ref, ElementType } from "react";

import type { SpringMode } from "../../surface/Pressable";
import type { ElevationOverride } from "../resolveElevationClass";
import type { ButtonVariant } from "./buttonVariants";

import { resolveComponent } from "./resolveComponent";
import { resolveButtonVariant, resolveButtonElevation } from "./resolveButtonProps";
import { resolveButtonDefaults } from "./resolveButtonDefaults";
import { buildSharedProps } from "./buildSharedProps";
import { ButtonAsChild } from "./ButtonAsChild";
import { ButtonPushable } from "./ButtonPushable";

/* -- Types -------------------------------------------------------- */

type ButtonSize = "sm" | "icon";

type ButtonProps = Omit<ComponentProps<"button">, "ref"> &
  Omit<ComponentProps<"a">, "ref"> & {
    variant?: ButtonVariant;
    size?: ButtonSize;
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
    href?: string;
    ref?: Ref<HTMLElement>;
  };

/* -- Component ---------------------------------------------------- */

function Button({ className, variant, href, style, ref, disabled, as, ...rest }: ButtonProps) {
  const d = resolveButtonDefaults(rest);
  const Comp = resolveComponent(as, d.asChild, href, rest.target as string | undefined);
  const sharedProps = buildSharedProps({
    ref,
    disabled,
    href,
    spring: d.spring,
    style,
    muted: d.muted,
    hovered: d.hovered,
    pressed: d.pressed,
    dormant: d.dormant,
    toggle: d.toggle,
    rest,
  });

  if (d.asChild || as) {
    return (
      <ButtonAsChild
        Comp={Comp}
        sharedProps={sharedProps}
        isIcon={d.isIcon}
        resolvedSize={d.resolvedSize}
        resolvedVariant={resolveButtonVariant(variant, disabled)}
        resolvedElevation={resolveButtonElevation(d.elevation, disabled)}
        fullWidth={d.fullWidth}
        size={rest.size}
        asChild={d.asChild}
        as={as}
        className={className}
        content={d.content}
      />
    );
  }

  const pushable = (
    <ButtonPushable
      Comp={Comp}
      sharedProps={sharedProps}
      isIcon={d.isIcon}
      resolvedSize={d.resolvedSize}
      resolvedVariant={resolveButtonVariant(variant, disabled)}
      resolvedElevation={resolveButtonElevation(d.elevation, disabled)}
      fullWidth={d.fullWidth}
      className={className}
      content={d.content}
    />
  );
  if (!d.dormant) return pushable;
  return <DormantWrapper disabled={disabled}>{pushable}</DormantWrapper>;
}

/** Wrap a dormant button with hover-zone padding. */
function DormantWrapper({ disabled, children }: { disabled?: boolean; children: ReactNode }) {
  if (disabled)
    return <span className="inline-flex opacity-50 pointer-events-none">{children}</span>;
  return <span className="group inline-flex p-4 -m-4">{children}</span>;
}

export { Button };
export type { ButtonVariant, SpringMode };
