import { describe, it, expect, vi, beforeEach } from "vitest";
import { buildGitHubIssueUrl } from "../buildGitHubIssueUrl";

/** Extract the decoded body param from a GitHub issue URL. */
function getBody(url: string): string {
  const params = new URLSearchParams(url.split("?")[1]);
  return params.get("body") ?? "";
}

beforeEach(() => {
  vi.useFakeTimers();
  vi.setSystemTime(new Date("2026-03-14T12:00:00Z"));
});

describe("buildGitHubIssueUrl", () => {
  it("builds a valid GitHub issue URL", () => {
    const url = buildGitHubIssueUrl({
      message: "Cannot read properties of undefined",
      route: "/compress-images",
    });

    expect(url).toContain("https://github.com/Develonaut/bnto/issues/new?");
    expect(url).toContain("labels=bug");

    const params = new URLSearchParams(url.split("?")[1]);
    expect(params.get("title")).toContain("Cannot read properties of undefined");
    expect(getBody(url)).toContain("compress-images");
  });

  it("includes environment info in body", () => {
    const body = getBody(
      buildGitHubIssueUrl({
        message: "Test error",
        route: "/",
        userAgent: "Mozilla/5.0 Test",
        version: "1.2.3",
      }),
    );

    expect(body).toContain("Mozilla/5.0 Test");
    expect(body).toContain("1.2.3");
    expect(body).toContain("2026-03-14");
  });

  it("includes digest when provided", () => {
    const body = getBody(
      buildGitHubIssueUrl({
        message: "Test error",
        route: "/",
        digest: "abc123",
      }),
    );

    expect(body).toContain("abc123");
  });

  it("truncates long error messages in title", () => {
    const longMessage = "A".repeat(200);
    const url = buildGitHubIssueUrl({
      message: longMessage,
      route: "/",
    });

    const params = new URLSearchParams(url.split("?")[1]);
    const title = params.get("title") ?? "";
    // [Bug] prefix (6 chars) + 80 chars max = 86
    expect(title.length).toBeLessThanOrEqual(86);
  });

  it("truncates long error messages in body", () => {
    const longMessage = "B".repeat(500);
    const body = getBody(
      buildGitHubIssueUrl({
        message: longMessage,
        route: "/",
      }),
    );

    // Body message capped at 200 chars
    expect(body).not.toContain("B".repeat(201));
  });

  it("includes stack trace in collapsed details block", () => {
    const stack = Array.from({ length: 20 }, (_, i) => `  at fn${i} (file.js:${i}:0)`).join("\n");
    const body = getBody(
      buildGitHubIssueUrl({
        message: "Test error",
        route: "/",
        stack,
      }),
    );

    expect(body).toContain("<details>");
    expect(body).toContain("Stack trace");
    // Only first 8 frames included
    expect(body).toContain("fn7");
    expect(body).not.toContain("fn8");
  });

  it("handles missing stack gracefully", () => {
    const body = getBody(
      buildGitHubIssueUrl({
        message: "Test error",
        route: "/",
      }),
    );

    expect(body).not.toContain("<details>");
  });

  it("keeps total body under 4000 chars", () => {
    const longStack = Array.from(
      { length: 100 },
      (_, i) =>
        `  at veryLongFunctionName${i} (very/long/path/to/file/that/makes/lines/really/long.tsx:${i}:0)`,
    ).join("\n");
    const body = getBody(
      buildGitHubIssueUrl({
        message: "C".repeat(200),
        route: "/some/deep/nested/route",
        userAgent: "D".repeat(200),
        stack: longStack,
      }),
    );

    expect(body.length).toBeLessThanOrEqual(4000);
  });

  it("produces a URL under 8000 chars total", () => {
    const longStack = Array.from({ length: 50 }, (_, i) => `  at fn${i} (path.tsx:${i}:0)`).join(
      "\n",
    );
    const url = buildGitHubIssueUrl({
      message: "E".repeat(200),
      route: "/long/route/path",
      userAgent: "F".repeat(200),
      version: "1.0.0-beta.123",
      stack: longStack,
    });

    expect(url.length).toBeLessThan(8000);
  });
});
