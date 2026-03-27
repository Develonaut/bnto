/**
 * Build a pre-filled GitHub issue URL for error reporting.
 *
 * Pure function — no React dependency, testable in isolation.
 * Truncates body to stay under GitHub's ~8,000 char URL limit.
 */

import { GITHUB_URL } from "./links";

/** Maximum body length after encoding to stay under GitHub's URL limit. */
const MAX_BODY_CHARS = 4000;

/** Maximum stack trace frames to include. */
const MAX_STACK_FRAMES = 8;

/** Maximum error message length in the title. */
const MAX_TITLE_MESSAGE = 80;

/** Maximum error message length in the body. */
const MAX_BODY_MESSAGE = 200;

interface IssueContext {
  message: string;
  stack?: string;
  route: string;
  userAgent?: string;
  version?: string;
  digest?: string;
}

export function buildGitHubIssueUrl(context: IssueContext): string {
  const title = `[Bug] ${context.message.slice(0, MAX_TITLE_MESSAGE)}`;

  const sections = [
    `## Error\n\`${context.message.slice(0, MAX_BODY_MESSAGE)}\``,
    `## Route\n\`${context.route}\``,
    buildEnvironmentSection(context),
    buildStackSection(context.stack),
  ].filter(Boolean);

  const body = sections.join("\n\n").slice(0, MAX_BODY_CHARS);

  const params = new URLSearchParams({
    title,
    body,
    labels: "bug",
  });

  return `${GITHUB_URL}/issues/new?${params}`;
}

function buildEnvironmentSection(context: IssueContext): string {
  const lines = ["## Environment"];

  if (context.userAgent) {
    lines.push(`- **Browser:** \`${context.userAgent}\``);
  }

  lines.push(`- **Timestamp:** ${new Date().toISOString()}`);
  lines.push(`- **Version:** \`${context.version ?? "unknown"}\``);

  if (context.digest) {
    lines.push(`- **Digest:** \`${context.digest}\``);
  }

  return lines.join("\n");
}

function buildStackSection(stack: string | undefined): string {
  if (!stack) return "";

  const frames = stack.split("\n").slice(0, MAX_STACK_FRAMES).join("\n");

  return `<details><summary>Stack trace</summary>\n\n\`\`\`\n${frames}\n\`\`\`\n</details>`;
}
