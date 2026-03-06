import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import { ScaleIn, Card, Pressable, Text } from "@bnto/ui";
import type { BentoNode } from "../../adapters/types";
import { ICON_COMPONENTS } from "../../adapters/nodeIcons";

/**
 * A bento box compartment — a .surface Card that acts as a "building"
 * on the Mini Motorways-style grid. Each compartment pops onto the
 * grid with a springy bounce, like buildings materializing on the map.
 *
 * Layout: large icon (32px) above the label, with category sublabel below.
 */

export type CompartmentStatus = "idle" | "pending" | "active" | "completed";

export const CompartmentNode = memo(function CompartmentNode({
  data,
  selected,
}: NodeProps<BentoNode>) {
  const w = data.width ?? 120;
  const h = data.height ?? 120;
  const status = data.status ?? "idle";
  const Icon = data.icon ? ICON_COMPONENTS[data.icon] : undefined;

  return (
    <ScaleIn from={0.7} easing="spring-bouncy">
      <Pressable asChild spring="bounciest" toggle active={selected} muted={status === "pending"}>
        <Card
          elevation={selected ? "lg" : "sm"}
          className="flex flex-col items-center justify-center gap-1 rounded-xl"
          style={{ width: w, height: h }}
          data-testid="compartment-node"
        >
          {Icon && <Icon className="size-8 text-muted-foreground" />}
          <Text size="sm" className="font-display font-semibold leading-tight text-center">
            {data.label}
          </Text>
          {data.sublabel && (
            <Text size="xs" color="muted" className="capitalize">
              {data.sublabel}
            </Text>
          )}
        </Card>
      </Pressable>
    </ScaleIn>
  );
});
