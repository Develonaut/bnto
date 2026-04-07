import { ArrowRight } from "lucide-react";
import { Button, Heading, Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";

/** "No catch" copy block — heading, description, and GitHub CTA. */
export function NoCatchCopy() {
  return (
    <Stack gap="md">
      <Text size="sm" mono color="muted" className="uppercase tracking-wider">
        No mystery meat
      </Text>
      <Heading level={2} size="xl" className="whitespace-pre-line">
        {"No hidden ingredients.\nOpen source you can taste-test."}
      </Heading>
      <Text color="muted" leading="snug">
        No signup. No watermarks. No daily caps. Your kitchen, your rules. MIT licensed. Peek behind
        the counter anytime.
      </Text>
      <div className="pt-2">
        <Button
          variant="secondary"
          href={`${GITHUB_URL}/tree/main/engine`}
          target="_blank"
          rel="noopener noreferrer"
        >
          Browse the engine source
          <ArrowRight className="size-4" />
        </Button>
      </div>
    </Stack>
  );
}
