"use client";

import { Card, Heading, Skeleton, Stack, Text } from "@bnto/ui";

import { useStepInView } from "./useStepInView";

interface StepCardProps {
  step: number;
  title: string;
  description: string;
  mascot: string;
  /** Max height of the mascot image in px. Defaults to 200. */
  mascotHeight?: number;
  variant: "primary" | "secondary" | "accent";
  className?: string;
  /** Stagger delay in ms before the card springs in. */
  delay?: number;
}

export function StepCard({
  step,
  title,
  description,
  mascot,
  mascotHeight = 200,
  className,
  delay = 0,
}: StepCardProps) {
  const [inView, ref] = useStepInView(0.4, delay);

  return (
    <Card ref={ref} dormant={!inView} elevation="lg" className={`p-6 ${className ?? ""}`}>
      <Stack gap="sm" className="h-[310px] items-center text-center">
        {inView ? (
          <>
            <div className="flex h-[200px] w-full items-center justify-center">
              {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
              <img
                src={mascot}
                alt=""
                className="w-auto shrink-0"
                style={{ maxHeight: mascotHeight }}
                aria-hidden
              />
            </div>
            <Text size="xs" mono color="muted" className="uppercase tracking-wider">
              Step {step}
            </Text>
            <Heading level={3} size="sm">
              {title}
            </Heading>
            <Text color="muted" size="sm" leading="snug">
              {description}
            </Text>
          </>
        ) : (
          <>
            <Skeleton className="h-[200px] w-[200px] rounded-full" />
            <Skeleton className="h-4 w-12 rounded" />
            <Skeleton className="h-6 w-2/3 rounded" />
            <Skeleton className="h-5 w-full rounded" />
            <Skeleton className="h-5 w-4/5 rounded" />
          </>
        )}
      </Stack>
    </Card>
  );
}
