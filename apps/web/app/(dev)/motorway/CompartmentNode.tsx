import { type NodeProps, type Node } from "@xyflow/react";
import { Animate } from "@/components/ui/Animate";
import { Card } from "@/components/ui/Card";
import { Pressable } from "@/components/ui/Pressable";
import { Text } from "@/components/ui/Text";

/**
 * A bento box compartment — a .surface Card that acts as a "building"
 * on the Mini Motorways-style grid. Each compartment pops onto the
 * grid with a springy bounce, like buildings materializing on the map.
 *
 * Compartments have variable sizes (width/height via data props) to
 * create the varied-compartment look of a real bento box — some are
 * small squares, others are wide rectangles, others tall.
 */

export type CompartmentStatus = "idle" | "pending" | "active" | "completed";

export type CompartmentData = {
  label: string;
  sublabel?: string;
  variant?:
    | "primary"
    | "secondary"
    | "accent"
    | "muted"
    | "success"
    | "warning";
  /** Width in px. Default: 120 */
  width?: number;
  /** Height in px. Default: 120 */
  height?: number;
  /** Simulation status — drives elevation and opacity. Default: "idle" */
  status?: CompartmentStatus;
};

export type CompartmentNodeType = Node<CompartmentData, "compartment">;

const SURFACE_CLASS: Record<NonNullable<CompartmentData["variant"]>, string> = {
  primary: "surface-primary",
  secondary: "surface-secondary",
  accent: "surface-accent",
  muted: "surface-muted",
  success: "surface-success",
  warning: "surface-warning",
};

const ELEVATION_BY_STATUS: Record<CompartmentStatus, "none" | "sm" | "md" | "lg"> = {
  idle: "lg",
  pending: "sm",
  active: "lg",
  completed: "md",
};

export function CompartmentNode({ data }: NodeProps<CompartmentNodeType>) {
  const w = data.width ?? 120;
  const h = data.height ?? 120;
  const status = data.status ?? "idle";
  const elevation = ELEVATION_BY_STATUS[status];

  return (
    <Animate.ScaleIn from={0.7} easing="spring-bouncy">
      <Pressable asChild spring="lg" muted={status === "pending"}>
        <Card
          elevation={elevation}
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
}
