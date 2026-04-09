"use client";

import { Grid, GridItem, Heading, SlideUp, Stack, Text } from "@bnto/ui";

import { StepCard } from "./StepCard";

const STEPS = [
  {
    step: 1,
    mascot: "/mascots/sushi-friends.svg",
    mascotHeight: 180,
    variant: "primary" as const,
    title: "Pick a recipe",
    description: "Browse the house specials, or compose your own from any node.",
  },
  {
    step: 2,
    mascot: "/mascots/penguin-chef.svg",
    variant: "secondary" as const,
    title: "Pack your box",
    description: "Each node does one job. Chain them into a recipe that fits your workflow.",
  },
  {
    step: 3,
    mascot: "/mascots/sushi-motorbike.svg",
    variant: "accent" as const,
    title: "Run it anywhere",
    description: "One command: bnto run. CLI, browser, or desktop. Files stay on your machine.",
  },
];

export function StepFlow() {
  return (
    <Stack gap="lg">
      <SlideUp>
        <Stack gap="md" className="items-center text-center">
          <Text size="sm" mono color="muted" className="uppercase tracking-wider">
            How it works
          </Text>
          <Heading level={2} size="xl" className="whitespace-pre-line">
            {"Nodes are compartments.\nRecipes are the box."}
          </Heading>
          <Text color="muted" leading="snug">
            Pick your fillings. The recipe packs the box.
          </Text>
        </Stack>
      </SlideUp>
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
