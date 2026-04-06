import { Heading, InView, Stack, Text } from "@bnto/ui";
import { BragLayout } from "./BragLayout";

/** "How it works" section — engine architecture pitch + brag cards. */
export function HowItWorksSection() {
  return (
    <InView>
      <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <Stack gap="md">
          <Text size="sm" mono color="muted" className="uppercase tracking-wider">
            How it works
          </Text>
          <Heading level={2} size="xl" className="whitespace-pre-line">
            {"Nodes are compartments.\nRecipes are the box."}
          </Heading>
          <Text color="muted" leading="snug">
            Each node encapsulates one capability: compress an image, rename a file, clean a CSV,
            download a video. Chain nodes into recipes that automate your workflow. Recipes are
            portable .bnto.json files that run everywhere: CLI, browser, desktop.
          </Text>
        </Stack>
        <BragLayout />
      </div>
    </InView>
  );
}
