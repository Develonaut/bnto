"use client";

import { useEffect } from "react";
import { buildGlobalErrorIssueUrl } from "./buildGlobalErrorIssueUrl";
import { GlobalErrorCard } from "./GlobalErrorCard";
import { bodyStyle, containerStyle } from "./globalErrorStyles";

/**
 * Root-level error boundary — last-resort catch-all.
 *
 * This replaces the entire <html> document when the root layout itself throws.
 * Cannot use the design system (AppShell, Card, etc.) since those live inside
 * the layout that just crashed. Uses minimal inline styles matching theme tokens.
 *
 * PostHog telemetry is NOT available here — providers are down. We rely on the
 * GitHub issue link for error reporting.
 */
export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  const issueUrl = buildGlobalErrorIssueUrl(error);

  useEffect(() => {
    console.error("[bnto] Global error boundary caught:", error);
  }, [error]);

  return (
    <html lang="en">
      <body style={bodyStyle}>
        <div style={containerStyle}>
          <GlobalErrorCard message={error.message} issueUrl={issueUrl} onReset={reset} />
        </div>
      </body>
    </html>
  );
}
