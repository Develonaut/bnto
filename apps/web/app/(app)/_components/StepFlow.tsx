"use client";

import {
  BoxIcon,
  FadeIn,
  Grid,
  GridItem,
  Heading,
  ListChecksIcon,
  SlideUp,
  Stack,
  TerminalIcon,
  Text,
} from "@bnto/ui";

import { StepCard } from "./StepCard";

const STEPS = [
  {
    step: 1,
    icon: ListChecksIcon,
    variant: "primary" as const,
    title: "Pick a recipe",
    description: "15 house specials included, or compose your own from any node.",
  },
  {
    step: 2,
    icon: BoxIcon,
    variant: "secondary" as const,
    title: "Pack your box",
    description: "Each node does one job. Chain them into a recipe that fits your workflow.",
  },
  {
    step: 3,
    icon: TerminalIcon,
    variant: "accent" as const,
    title: "Run it anywhere",
    description: "One command: bnto run. CLI, browser, or desktop. Files stay on your machine.",
  },
];

export function StepFlow() {
  return (
    <Stack gap="lg">
      <div className="flex flex-col items-center justify-center gap-6 lg:flex-row lg:items-center">
        <FadeIn>
          {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
          <img
            src="/mascots/bento-sushi.svg"
            alt=""
            width={280}
            height={280}
            className="shrink-0"
            aria-hidden
          />
        </FadeIn>
        <SlideUp>
          <Stack gap="md" style={{ paddingTop: 44, marginLeft: -28 }}>
            <Text size="sm" mono color="muted" className="uppercase tracking-wider">
              What&apos;s in the box
            </Text>
            <Heading level={2} size="xl" className="whitespace-pre-line">
              {"Nodes are compartments.\nRecipes are the box."}
            </Heading>
            <Text color="muted" leading="snug">
              Pick your fillings. The recipe packs the box.
            </Text>
          </Stack>
        </SlideUp>
      </div>
      <div
        className="rounded-lg border bg-muted p-3 lg:p-4"
        style={{ borderColor: "var(--surface-muted-wall)" }}
      >
        <Grid cols={{ mobile: 1, desktop: 3 }} gap="md">
          <GridItem colSpan={1}>
            <StepCard {...STEPS[0]} delay={200} className="h-full" />
          </GridItem>
          <GridItem colSpan={1}>
            <StepCard {...STEPS[1]} delay={300} className="h-full" />
          </GridItem>
          <GridItem colSpan={1}>
            <StepCard {...STEPS[2]} delay={400} className="h-full" />
          </GridItem>
        </Grid>
      </div>
    </Stack>
  );
}
