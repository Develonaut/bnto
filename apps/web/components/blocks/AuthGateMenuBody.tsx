import { Button, Row, Stack, Text } from "@bnto/ui";

interface AuthGateMenuBodyProps {
  title: string;
  description: string;
}

export function AuthGateMenuBody({ title, description }: AuthGateMenuBodyProps) {
  return (
    <Stack className="gap-4">
      <Stack className="gap-1.5">
        <Text size="base" weight="medium">
          {title}
        </Text>
        <Text size="sm" color="muted">
          {description}
        </Text>
      </Stack>
      <Row className="gap-2 justify-center">
        <Button href="/signin" variant="primary" elevation="sm" className="h-8 px-4 text-sm">
          Sign up free
        </Button>
        <Button href="/signin" variant="ghost" className="h-8 px-4 text-sm">
          Sign in
        </Button>
      </Row>
    </Stack>
  );
}
