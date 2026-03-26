import { Button, GithubIcon, Heading, Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";

/** "No catch" copy block — heading, description, and GitHub CTA. */
export function NoCatchCopy() {
  return (
    <Stack gap="md">
      <Text size="sm" mono color="muted" className="uppercase tracking-wider">
        No catch
      </Text>
      <Heading level={2} size="xl" className="whitespace-pre-line">
        {"Free tools that stay free.\nOpen source you can verify."}
      </Heading>
      <Text color="muted" leading="snug">
        No signup. No watermarks. No daily caps. No &apos;20 free compressions per month.&apos; Your
        browser does all the processing, so it costs us nothing to run. We&apos;ll never put a meter
        on it. The engine is open source and MIT licensed. You can read every line.
      </Text>
      <div className="pt-2">
        <Button
          variant="outline"
          href={GITHUB_URL}
          target="_blank"
          rel="noopener noreferrer"
          elevation="sm"
        >
          <GithubIcon className="size-4" />
          View on GitHub
        </Button>
      </div>
    </Stack>
  );
}
