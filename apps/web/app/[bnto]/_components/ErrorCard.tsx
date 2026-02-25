import { XCircle } from "lucide-react";

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
      <div className="flex items-start gap-3">
        <XCircle className="mt-0.5 size-5 shrink-0 text-destructive" />
        <div className="space-y-1">
          <p className="text-sm font-medium text-destructive">
            Something went wrong
          </p>
          <p className="text-sm text-muted-foreground">{error}</p>
        </div>
      </div>
    </div>
  );
}
