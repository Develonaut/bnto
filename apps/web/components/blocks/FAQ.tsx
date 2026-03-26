import { Heading, Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";

import { FAQAccordion } from "./FAQAccordion";

export function FAQ({ headerTag = "h2" }: { headerTag?: "h1" | "h2" }) {
  return (
    <div className="grid gap-12 lg:grid-cols-2 lg:gap-16">
      <Stack gap="md">
        <Heading level={headerTag === "h1" ? 1 : 2}>Frequently asked questions</Heading>
        <Text color="muted" leading="snug" className="max-w-md">
          Can&apos;t find what you need?{" "}
          <a
            href={GITHUB_URL}
            target="_blank"
            rel="noopener noreferrer"
            className="underline underline-offset-4 transition-colors hover:text-foreground"
          >
            Open an issue on GitHub
          </a>
          .
        </Text>
      </Stack>

      <FAQAccordion />
    </div>
  );
}
