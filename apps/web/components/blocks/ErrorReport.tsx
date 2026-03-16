"use client";

import { useEffect } from "react";
import {
  ArrowLeftIcon,
  Button,
  Card,
  CardContent,
  CardFooter,
  CircleAlertIcon,
  Heading,
  RefreshCwIcon,
  Row,
  Stack,
  Text,
} from "@bnto/ui";
import { core } from "@bnto/core";
import { buildGitHubIssueUrl } from "@/lib/buildGitHubIssueUrl";

interface ErrorReportProps {
  error: Error & { digest?: string };
  reset: () => void;
}

export function ErrorReport({ error, reset }: ErrorReportProps) {
  const route = typeof window !== "undefined" ? window.location.pathname : "unknown";
  const userAgent = typeof navigator !== "undefined" ? navigator.userAgent : undefined;
  const version = process.env.NEXT_PUBLIC_APP_VERSION;

  const issueUrl = buildGitHubIssueUrl({
    message: error.message,
    stack: error.stack,
    route,
    userAgent,
    version,
    digest: error.digest,
  });

  useEffect(() => {
    core.telemetry.capture("app_error", {
      message: error.message,
      route,
      digest: error.digest,
    });
  }, [error.message, error.digest, route]);

  return (
    <Stack align="center" justify="center" className="min-h-[60vh] text-center">
      <Card className="max-w-lg w-full px-8 py-10">
        <CardContent>
          <Stack gap="lg" align="center">
            <CircleAlertIcon className="size-12 text-destructive" />
            <Stack gap="md" align="center">
              <Heading level={2} data-testid="error-heading">
                Something went wrong
              </Heading>
              <Text color="muted" balance>
                An unexpected error occurred. You can try again or report this issue on GitHub so we
                can fix it.
              </Text>
            </Stack>
            <Text
              size="sm"
              mono
              color="muted"
              className="max-w-full truncate px-4"
              data-testid="error-message"
            >
              {error.message}
            </Text>
          </Stack>
        </CardContent>
        <CardFooter>
          <Row gap="sm" className="w-full justify-center pt-2">
            <Button onClick={reset} data-testid="error-try-again" className="gap-2">
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
        </CardFooter>
      </Card>
    </Stack>
  );
}
