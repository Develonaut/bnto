"use client";

import { Heading, SlideUp, Stack, Text } from "@bnto/ui";
import { StepFlowLayout } from "@/components/sections";
import type { StepFlowStep } from "@/components/sections";

const STEPS: StepFlowStep[] = [
  {
    step: 1,
    mascot: "/mascots/sushi-friends.svg",
    mascotHeight: 180,
    variant: "primary",
    title: "Pick a recipe",
    description: "Browse the house specials, or compose your own from any node.",
  },
  {
    step: 2,
    mascot: "/mascots/penguin-chef.svg",
    variant: "secondary",
    title: "Pack your box",
    description: "Each node does one job. Chain them into a recipe that fits your workflow.",
  },
  {
    step: 3,
    mascot: "/mascots/sushi-motorbike.svg",
    variant: "accent",
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
      <StepFlowLayout steps={STEPS} />
    </Stack>
  );
}
