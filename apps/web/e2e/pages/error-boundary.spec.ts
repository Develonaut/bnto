import { test, expect } from "@playwright/test";

/**
 * Error boundary journey — verifies that unhandled React errors are caught
 * and display the ErrorReport dialog instead of a white screen.
 *
 * Uses a dev-only test page at /test-error that intentionally throws
 * during render, triggering the (dev) error boundary.
 *
 * NOTE: Imports from @playwright/test (not ../fixtures) because the shared
 * fixture auto-fails when the Next.js dev error overlay detects errors.
 * This test INTENTIONALLY triggers errors — the overlay is expected.
 */
test.describe("Error boundary @browser", () => {
  test("shows error dialog when a component throws", async ({ page }) => {
    // Navigate to the intentional-error page — it throws during render,
    // which the (dev)/error.tsx boundary catches and renders ErrorReport
    await page.goto("/test-error");

    // ErrorReport should render with the error message
    await expect(page.getByTestId("error-heading")).toBeVisible();

    // Verify the three action buttons exist
    await expect(page.getByTestId("error-try-again")).toBeVisible();
    await expect(page.getByTestId("error-report-issue")).toBeVisible();
    await expect(page.getByTestId("error-back-home")).toBeVisible();

    // Verify the error message is displayed
    await expect(page.getByTestId("error-message")).toBeVisible();

    // Verify the Report Issue link points to GitHub with pre-filled data
    const reportLink = page.getByTestId("error-report-issue");
    const href = await reportLink.getAttribute("href");
    expect(href).toContain("github.com/Develonaut/bnto/issues/new");
    expect(href).toContain("labels=bug");
  });

  test("Report Issue link includes error context", async ({ page }) => {
    await page.goto("/test-error");

    await expect(page.getByTestId("error-heading")).toBeVisible();

    const reportLink = page.getByTestId("error-report-issue");
    const href = await reportLink.getAttribute("href");

    // URL should contain pre-filled issue fields
    const url = new URL(href!);
    const title = url.searchParams.get("title");
    const body = url.searchParams.get("body");

    expect(title).toContain("[Bug]");
    expect(title).toContain("Intentional test error");
    expect(body).toContain("test-error");
    expect(body).toContain("Environment");
  });

  test("Try Again button resets the error boundary", async ({ page }) => {
    await page.goto("/test-error");

    await expect(page.getByTestId("error-heading")).toBeVisible();

    // Click Try Again — it calls reset() which re-renders the erroring component.
    // Since the test page always throws, the error boundary will catch again.
    await page.getByTestId("error-try-again").click();

    // The error boundary should still be visible (component throws again)
    await expect(page.getByTestId("error-heading")).toBeVisible();
  });

  test("Back to Home link navigates to root", async ({ page }) => {
    await page.goto("/test-error");

    await expect(page.getByTestId("error-heading")).toBeVisible();

    const homeLink = page.getByTestId("error-back-home");
    expect(await homeLink.getAttribute("href")).toBe("/");
  });
});
