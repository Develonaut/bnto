import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import { Animate } from "@/components/ui/Animate";
import { Card } from "@/components/ui/Card";
import { Pressable } from "@/components/ui/Pressable";
import { Text } from "@/components/ui/Text";
import type { CompartmentVariant, BentoNode } from "@/editor/adapters/types";

/**
 * A bento box compartment — a .surface Card that acts as a "building"
 * on the Mini Motorways-style grid. Each compartment pops onto the
 * grid with a springy bounce, like buildings materializing on the map.
 *
 * Compartments have variable sizes (width/height via data props) to
 * create the varied-compartment look of a real bento box — some are
 * small squares, others are wide rectangles, others tall.
 *
 * Domain data (nodeType, name, parameters) lives in the configs store,
 * NOT in node.data. Only visual fields (label, variant, etc.) are in
 * node.data — this prevents parameter changes from re-rendering nodes.
 */

export type CompartmentStatus = "idle" | "pending" | "active" | "completed";

const SURFACE_CLASS: Record<CompartmentVariant, string> = {
  primary: "surface-primary",
  secondary: "surface-secondary",
  accent: "surface-accent",
  muted: "surface-muted",
  success: "surface-success",
  warning: "surface-warning",
};

export const CompartmentNode = memo(function CompartmentNode({
  data,
  selected,
}: NodeProps<BentoNode>) {
  const w = data.width ?? 120;
  const h = data.height ?? 120;
  const status = data.status ?? "idle";
  return (
    <Animate.ScaleIn from={0.7} easing="spring-bouncy">
      <Pressable asChild spring="bounciest" toggle active={selected} muted={status === "pending"}>
        <Card
          elevation="lg"
          className={`${SURFACE_CLASS[data.variant ?? "primary"]} flex flex-col items-center justify-center rounded-xl`}
          style={{ width: w, height: h }}
        >
          <Text size="sm" className="font-display font-semibold leading-tight">
            {data.label}
          </Text>
          {data.sublabel && (
            <Text size="xs" color="muted" className="mt-0.5">
              {data.sublabel}
            </Text>
          )}
        </Card>
      </Pressable>
    </Animate.ScaleIn>
  );
});
