import { IconBadge } from "@bnto/ui";
import { ICON_COMPONENTS } from "../../../adapters/nodeIcons";
import type { CompartmentVariant } from "../../../adapters/types";

/** NodeIcon — renders the node's Lucide icon in a colored badge circle. */

function NodeIcon({ icon, variant = "muted" }: { icon?: string; variant?: CompartmentVariant }) {
  const Icon = icon ? ICON_COMPONENTS[icon] : undefined;
  if (!Icon) return null;
  return (
    <IconBadge variant={variant} size="lg">
      <Icon className="size-5" />
    </IconBadge>
  );
}

export { NodeIcon };
