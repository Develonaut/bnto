import { FadeIn, Heading, SlideUp, Stack, Text } from "@bnto/ui";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

import { Marquee } from "./Marquee";
import { ShowcaseCard } from "./ShowcaseCard";

const firstRow = BNTO_REGISTRY.slice(0, Math.ceil(BNTO_REGISTRY.length / 2));
const secondRow = BNTO_REGISTRY.slice(Math.ceil(BNTO_REGISTRY.length / 2));

export function RecipeShowcase() {
  return (
    <Stack gap="xl">
      {/* Header — mascot left, body text right */}
      <div className="flex flex-col items-center justify-center gap-6 lg:flex-row lg:items-center">
        <FadeIn>
          {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
          <img
            src="/mascots/sushi-friends.svg"
            alt=""
            width={280}
            height={280}
            className="shrink-0"
            aria-hidden
          />
        </FadeIn>
        <SlideUp>
          <Stack gap="md">
            <Text size="sm" mono color="muted" className="uppercase tracking-wider">
              House specials
            </Text>
            <Heading level={2} size="xl" className="whitespace-pre-line">
              {"15 recipes, ready to run.\nPick one or build your own."}
            </Heading>
            <Text color="muted" leading="snug">
              Every recipe runs locally. Your files never leave your machine.
            </Text>
          </Stack>
        </SlideUp>
      </div>

      {/* Horizontal marquee — two rows, opposite directions */}
      <div className="relative flex w-full flex-col items-center justify-center overflow-hidden">
        <Marquee pauseOnHover className="[--duration:35s] [--gap:1.5rem] [&_a]:h-[240px]">
          {firstRow.map((entry) => (
            <ShowcaseCard key={entry.slug} entry={entry} />
          ))}
        </Marquee>
        <Marquee reverse pauseOnHover className="[--duration:35s] [--gap:1.5rem] [&_a]:h-[240px]">
          {secondRow.map((entry) => (
            <ShowcaseCard key={entry.slug} entry={entry} />
          ))}
        </Marquee>
        <div className="from-background pointer-events-none absolute inset-y-0 left-0 w-1/6 bg-gradient-to-r" />
        <div className="from-background pointer-events-none absolute inset-y-0 right-0 w-1/6 bg-gradient-to-l" />
      </div>
    </Stack>
  );
}
