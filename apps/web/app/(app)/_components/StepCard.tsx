"use client";

import type { ComponentType } from "react";

import { Card, Heading, IconBadge, Skeleton, Stack, Text } from "@bnto/ui";

import { useStepInView } from "./useStepInView";

type IconBadgeVariant = "primary" | "secondary" | "accent";

interface StepCardProps {
  step: number;
  title: string;
  description: string;
  icon: ComponentType<{ className?: string }>;
  variant: IconBadgeVariant;
  className?: string;
  /** Stagger delay in ms before the card springs in. */
  delay?: number;
}

export function StepCard({
  step,
  title,
  description,
  icon: Icon,
  variant,
  className,
  delay = 0,
}: StepCardProps) {
  const [inView, ref] = useStepInView(0.4, delay);

  return (
    <Card ref={ref} dormant={!inView} elevation="lg" className={`p-6 ${className ?? ""}`}>
      {inView ? (
        <Stack gap="sm">
          <IconBadge variant={variant} size="md" shape="square">
            <Icon className="size-4" />
          </IconBadge>
          <Text size="xs" mono color="muted" className="uppercase tracking-wider">
            Step {step}
          </Text>
          <Heading level={3} size="sm">
            {title}
          </Heading>
          <Text color="muted" size="sm" leading="snug">
            {description}
          </Text>
        </Stack>
      ) : (
        <Stack gap="sm">
          <Skeleton className="size-9 rounded-lg" />
          <Skeleton className="h-3 w-12 rounded" />
          <Skeleton className="h-5 w-2/3 rounded" />
          <Skeleton className="h-4 w-full rounded" />
          <Skeleton className="h-4 w-4/5 rounded" />
        </Stack>
      )}
    </Card>
  );
}
