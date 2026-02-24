import { Button } from "#components/ui/button";
import { Container } from "#components/ui/Container";
import { Heading } from "#components/ui/Heading";
import { Text } from "#components/ui/Text";
import { Stack } from "#components/ui/Stack";

export function FooterCta() {
  return (
    <Container>
      <Stack gap="sm" className="text-center">
        <Heading level={2}>
          Start your free trial today
        </Heading>
        <Text color="muted" leading="snug" balance className="mx-auto max-w-xl">
          Mainline is the fit-for-purpose tool for planning and building modern
          software products.
        </Text>
        <div>
          <Button size="lg" className="mt-4" asChild>
            <a href="https://github.com/shadcnblocks/mainline-nextjs-template">
              Get template
            </a>
          </Button>
        </div>
      </Stack>
    </Container>
  );
}
