import { buildGitHubIssueUrl } from "@/lib/buildGitHubIssueUrl";

/** Build the GitHub issue URL from the global error boundary context. */
export function buildGlobalErrorIssueUrl(error: Error & { digest?: string }) {
  const route = typeof window !== "undefined" ? window.location.pathname : "unknown";
  const userAgent = typeof navigator !== "undefined" ? navigator.userAgent : undefined;

  return buildGitHubIssueUrl({
    message: error.message,
    stack: error.stack,
    route,
    userAgent,
    version: process.env.NEXT_PUBLIC_APP_VERSION,
    digest: error.digest,
  });
}
