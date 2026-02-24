import { Heading } from "#components/ui/Heading";
import { Text } from "#components/ui/Text";
import { Stack } from "#components/ui/Stack";

export function ShowcaseSection({
  id,
  title,
  description,
  children,
}: {
  id: string;
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <Stack as="section" gap="md" data-testid={id}>
      <div>
        <Heading level={2} size="sm">{title}</Heading>
        {description && (
          <Text size="sm" color="muted" className="mt-1">{description}</Text>
        )}
      </div>
      {children}
    </Stack>
  );
}
