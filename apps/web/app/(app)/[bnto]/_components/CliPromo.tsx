import { Card, Badge, Heading, Text, Stack, Row } from "@bnto/ui";

export function CliPromo({ slug }: { slug: string }) {
  return (
    <Card className="mx-auto max-w-lg p-6 text-left">
      <Stack className="gap-4">
        <Row className="gap-2">
          <Badge size="sm">CLI Only</Badge>
        </Row>
        <Heading level={3} size="sm">
          Available via the bnto CLI
        </Heading>
        <Text size="sm" color="muted">
          This recipe requires system tools that can&apos;t run in a browser. Install the CLI to run
          it locally.
        </Text>

        <Stack className="gap-2 rounded-lg bg-muted p-4">
          <Text size="xs" mono color="muted" className="uppercase tracking-wider">
            Install
          </Text>
          <code className="font-mono text-sm">cargo install bnto</code>
        </Stack>

        <Stack className="gap-2 rounded-lg bg-muted p-4">
          <Text size="xs" mono color="muted" className="uppercase tracking-wider">
            Run
          </Text>
          <code className="font-mono text-sm">bnto run {slug} &lt;url&gt;</code>
        </Stack>

        <a
          href="https://github.com/bntoio/bnto"
          target="_blank"
          rel="noopener noreferrer"
          className="text-primary text-sm font-medium hover:underline"
        >
          View on GitHub
        </a>
      </Stack>
    </Card>
  );
}
