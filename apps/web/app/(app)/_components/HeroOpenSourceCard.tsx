import { Card, GithubIcon, Heading, IconBadge, Stack, Text } from "@bnto/ui";
import Link from "next/link";
import { GITHUB_URL } from "@/lib/copy";

export function HeroOpenSourceCard() {
  return (
    <Card
      asChild
      color="muted"
      className="pressable flex h-full flex-col justify-between gap-4 p-6"
    >
      <Link href={GITHUB_URL} target="_blank" rel="noopener noreferrer">
        <IconBadge variant="primary" size="lg" shape="square">
          <GithubIcon className="size-5" />
        </IconBadge>
        <Stack className="gap-1">
          <Heading level={3} as="p" size="xs">
            Open source (MIT)
          </Heading>
          <Text size="sm" color="muted">
            Inspect every line. Contribute on GitHub.
          </Text>
        </Stack>
      </Link>
    </Card>
  );
}
