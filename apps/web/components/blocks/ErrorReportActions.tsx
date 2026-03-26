import { ArrowLeftIcon, Button, RefreshCwIcon, Row } from "@bnto/ui";

interface ErrorReportActionsProps {
  issueUrl: string;
  onReset: () => void;
}

export function ErrorReportActions({ issueUrl, onReset }: ErrorReportActionsProps) {
  return (
    <Row gap="sm" className="w-full justify-center pt-2">
      <Button onClick={onReset} data-testid="error-try-again" className="gap-2">
        <RefreshCwIcon className="size-4" />
        Try Again
      </Button>
      <Button
        variant="outline"
        href={issueUrl}
        target="_blank"
        rel="noopener noreferrer"
        data-testid="error-report-issue"
      >
        Report Issue
      </Button>
      <Button variant="ghost" href="/" data-testid="error-back-home" className="gap-2">
        <ArrowLeftIcon className="size-4" />
        Back to Home
      </Button>
    </Row>
  );
}
