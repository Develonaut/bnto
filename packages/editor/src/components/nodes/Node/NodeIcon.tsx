import { ICON_COMPONENTS } from "../../../adapters/nodeIcons";

/** NodeIcon — renders the node's Lucide icon inside a card. */

function NodeIcon({ icon }: { icon?: string }) {
  const Icon = icon ? ICON_COMPONENTS[icon] : undefined;
  if (!Icon) return null;
  return <Icon className="size-8 text-muted-foreground" />;
}

export { NodeIcon };
