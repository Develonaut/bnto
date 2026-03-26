import { CircleAlertIcon, Heading, Stack, Text } from "@bnto/ui";

interface ErrorReportContentProps {
  message: string;
}

export function ErrorReportContent({ message }: ErrorReportContentProps) {
  return (
    <Stack gap="lg" align="center">
      <CircleAlertIcon className="size-12 text-destructive" />
      <Stack gap="md" align="center">
        <Heading level={2} data-testid="error-heading">
          Something went wrong
        </Heading>
        <Text color="muted" balance>
          An unexpected error occurred. You can try again or report this issue on GitHub so we can
          fix it.
        </Text>
      </Stack>
      <Text
        size="sm"
        mono
        color="muted"
        className="max-w-full truncate px-4"
        data-testid="error-message"
      >
        {message}
      </Text>
    </Stack>
  );
}
