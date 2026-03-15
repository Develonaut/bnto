"use client";

import { useEffect } from "react";
import { buildGitHubIssueUrl } from "@/lib/buildGitHubIssueUrl";

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
  const route = typeof window !== "undefined" ? window.location.pathname : "unknown";
  const userAgent = typeof navigator !== "undefined" ? navigator.userAgent : undefined;

  const issueUrl = buildGitHubIssueUrl({
    message: error.message,
    stack: error.stack,
    route,
    userAgent,
    version: process.env.NEXT_PUBLIC_APP_VERSION,
    digest: error.digest,
  });

  useEffect(() => {
    console.error("[bnto] Global error boundary caught:", error);
  }, [error]);

  return (
    <html lang="en">
      <body style={bodyStyle}>
        <div style={containerStyle}>
          <div style={cardStyle}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="oklch(0.6356 0.2082 25.38)"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="12" cy="12" r="10" />
              <line x1="12" x2="12" y1="8" y2="12" />
              <line x1="12" x2="12.01" y1="16" y2="16" />
            </svg>

            <h1 style={headingStyle}>Something went wrong</h1>
            <p style={textStyle}>
              An unexpected error occurred. You can try again or report this issue so we can fix it.
            </p>
            <p style={errorMessageStyle}>{error.message}</p>

            <div style={buttonRowStyle}>
              <button type="button" onClick={reset} style={primaryButtonStyle}>
                Try Again
              </button>
              <a
                href={issueUrl}
                target="_blank"
                rel="noopener noreferrer"
                style={outlineButtonStyle}
              >
                Report Issue
              </a>
              {/* eslint-disable-next-line @next/next/no-html-link-for-pages -- global-error replaces the document, next/link router context unavailable */}
              <a href="/" style={ghostButtonStyle}>
                Back to Home
              </a>
            </div>
          </div>
        </div>
      </body>
    </html>
  );
}

/* Minimal inline styles matching bnto theme tokens — no CSS file available. */

const bodyStyle: React.CSSProperties = {
  margin: 0,
  fontFamily: "Inter, system-ui, -apple-system, sans-serif",
  backgroundColor: "oklch(0.9899 0.0164 95.22)",
  color: "oklch(0.2628 0.0204 31.40)",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  minHeight: "100vh",
};

const containerStyle: React.CSSProperties = {
  padding: "1.5rem",
  width: "100%",
  maxWidth: "32rem",
};

const cardStyle: React.CSSProperties = {
  backgroundColor: "oklch(1.0000 0 0)",
  borderRadius: "1.25rem",
  padding: "2.5rem 2rem",
  textAlign: "center",
  boxShadow: "0 4px 6px -1px hsla(10, 20%, 15%, 0.08), 0 2px 4px -2px hsla(10, 20%, 15%, 0.06)",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: "1rem",
};

const headingStyle: React.CSSProperties = {
  fontSize: "1.5rem",
  fontWeight: 600,
  fontFamily: "Geist, system-ui, sans-serif",
  margin: 0,
  letterSpacing: "0.02em",
};

const textStyle: React.CSSProperties = {
  color: "oklch(0.5452 0.0251 31.20)",
  fontSize: "1rem",
  lineHeight: 1.6,
  margin: 0,
  textWrap: "balance",
};

const errorMessageStyle: React.CSSProperties = {
  fontFamily: "Geist Mono, monospace",
  fontSize: "0.875rem",
  color: "oklch(0.5452 0.0251 31.20)",
  maxWidth: "100%",
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  padding: "0 1rem",
};

const buttonRowStyle: React.CSSProperties = {
  display: "flex",
  gap: "0.5rem",
  paddingTop: "0.5rem",
  flexWrap: "wrap",
  justifyContent: "center",
};

const sharedButtonStyle: React.CSSProperties = {
  padding: "0.5rem 1rem",
  borderRadius: "calc(1.25rem - 2px)",
  fontSize: "0.875rem",
  fontWeight: 500,
  fontFamily: "Inter, system-ui, sans-serif",
  cursor: "pointer",
  textDecoration: "none",
  display: "inline-flex",
  alignItems: "center",
  gap: "0.5rem",
  border: "none",
  transition: "opacity 0.15s ease-out",
};

const primaryButtonStyle: React.CSSProperties = {
  ...sharedButtonStyle,
  backgroundColor: "oklch(0.6751 0.1788 35.19)",
  color: "white",
};

const outlineButtonStyle: React.CSSProperties = {
  ...sharedButtonStyle,
  backgroundColor: "transparent",
  color: "oklch(0.2628 0.0204 31.40)",
  border: "1px solid oklch(0.8976 0.0168 95.25)",
};

const ghostButtonStyle: React.CSSProperties = {
  ...sharedButtonStyle,
  backgroundColor: "transparent",
  color: "oklch(0.5452 0.0251 31.20)",
};
