import { IconBadge } from "@bnto/ui";
import { ICON_COMPONENTS } from "../../../adapters/nodeIcons";
import type { CompartmentVariant } from "../../../adapters/types";

/**
 * NodeIcon — renders the node's Lucide icon in a colored badge circle.
 *
 * When `onSurface` is true (e.g. failed node on a destructive card),
 * the badge uses a semi-transparent white circle + inherited white text
 * instead of the variant's colored badge — matching how buttons show
 * icons on colored backgrounds.
 */

interface NodeIconProps {
  icon?: string;
  variant?: CompartmentVariant;
  /** On a colored surface — use white-on-white-alpha instead of variant colors. */
  onSurface?: boolean;
}

function NodeIcon({ icon, variant = "muted", onSurface }: NodeIconProps) {
  const Icon = icon ? ICON_COMPONENTS[icon] : undefined;
  if (!Icon) return null;

  if (onSurface) {
    return (
      <div className="flex size-12 shrink-0 items-center justify-center rounded-full bg-white/20 text-inherit">
        <Icon className="size-6" />
      </div>
    );
  }

  return (
    <IconBadge variant={variant} size="lg" className="size-12">
      <Icon className="size-6" />
    </IconBadge>
  );
}

export { NodeIcon };
