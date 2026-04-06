import { Heading, InView, Stack, Text } from "@bnto/ui";
import { BragLayout } from "./BragLayout";

/** "How it works" section — engine architecture pitch + brag cards. */
export function HowItWorksSection() {
  return (
    <InView>
      <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <Stack gap="md">
          <Text size="sm" mono color="muted" className="uppercase tracking-wider">
            What&apos;s in the box
          </Text>
          <Heading level={2} size="xl" className="whitespace-pre-line">
            {"Pick your fillings.\nThe recipe packs the box."}
          </Heading>
          <Text color="muted" leading="snug">
            One node, one job. Chain them together, run them anywhere.
          </Text>
        </Stack>
        <BragLayout />
      </div>
    </InView>
  );
}
