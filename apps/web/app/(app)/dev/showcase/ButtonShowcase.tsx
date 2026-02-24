import { Button } from "#components/ui/button";
import { Text } from "#components/ui/Text";
import { Stack } from "#components/ui/Stack";
import { Row } from "#components/ui/Row";
import { ArrowRight } from "lucide-react";

function ButtonRow({
  variant,
  label,
}: {
  variant: "default" | "secondary" | "muted" | "destructive" | "success" | "warning" | "outline";
  label: string;
}) {
  return (
    <Row className="gap-6">
      <Text size="xs" color="muted" mono as="span" className="w-24 shrink-0">
        {label}
      </Text>
      <Button variant={variant} size="icon">
        <ArrowRight />
      </Button>
      <Button variant={variant} size="sm">
        Learn More
      </Button>
      <Button variant={variant} size="default">
        Learn More
      </Button>
      <Button variant={variant} size="lg" data-testid={`pressable-${label.toLowerCase()}-lg`}>
        Learn More
      </Button>
    </Row>
  );
}

export function ButtonShowcase() {
  return (
    <Stack gap="lg">
      <ButtonRow variant="default" label="Default" />
      <ButtonRow variant="secondary" label="Secondary" />
      <ButtonRow variant="outline" label="Outline" />
      <ButtonRow variant="muted" label="Muted" />
      <ButtonRow variant="destructive" label="Destructive" />
      <ButtonRow variant="success" label="Success" />
      <ButtonRow variant="warning" label="Warning" />
    </Stack>
  );
}
