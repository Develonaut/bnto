"use client";

import { Stack } from "@bnto/ui";
import { buildGitHubIssueUrl } from "@/lib/buildGitHubIssueUrl";

import { ErrorReportCard } from "./ErrorReportCard";
import { useErrorTelemetry } from "./useErrorTelemetry";

interface ErrorReportProps {
  error: Error & { digest?: string };
  reset: () => void;
}

export function ErrorReport({ error, reset }: ErrorReportProps) {
  const route = typeof window !== "undefined" ? window.location.pathname : "unknown";

  const issueUrl = buildGitHubIssueUrl({
    message: error.message,
    stack: error.stack,
    route,
    userAgent: typeof navigator !== "undefined" ? navigator.userAgent : undefined,
    version: process.env.NEXT_PUBLIC_APP_VERSION,
    digest: error.digest,
  });

  useErrorTelemetry(error.message, error.digest, route);

  return (
    <Stack align="center" justify="center" className="min-h-[60vh] text-center">
      <ErrorReportCard message={error.message} issueUrl={issueUrl} onReset={reset} />
    </Stack>
  );
}
