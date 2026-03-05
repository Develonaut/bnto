import { Row, Stack, XCircleIcon } from "@bnto/ui";

interface ErrorCardProps {
  error: string;
}

/** Error card for client-side failures (browser or cloud). */
export function ErrorCard({ error }: ErrorCardProps) {
  return (
    <div
      className="rounded-lg border border-destructive/50 bg-destructive/5 p-4 text-left"
      data-testid="client-error"
    >
      <Row align="start" className="gap-3">
        <XCircleIcon className="mt-0.5 size-5 shrink-0 text-destructive" />
        <Stack className="gap-1">
          <p className="text-sm font-medium text-destructive">
            Something went wrong
          </p>
          <p className="text-sm text-muted-foreground">{error}</p>
        </Stack>
      </Row>
    </div>
  );
}
