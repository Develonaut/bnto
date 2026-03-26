import { Heading, Stack, Text } from "@bnto/ui";
import { NotFoundActions } from "./NotFoundActions";

/** Inner content for the 404 page — heading, description, and action buttons. */
export function NotFoundContent() {
  return (
    <Stack align="center" justify="center" className="min-h-[60vh] text-center">
      <Stack gap="md" align="center" className="max-w-lg">
        <Text size="sm" mono color="muted" className="uppercase tracking-wider">
          404
        </Text>
        <Heading level={1} data-testid="not-found-heading">
          Page Not Found
        </Heading>
        <Text color="muted" size="lg" balance>
          Sorry, we couldn&apos;t find the page you&apos;re looking for. The page might have been
          removed or the URL might be incorrect.
        </Text>
      </Stack>
      <NotFoundActions />
    </Stack>
  );
}
