import { test, expect } from "@playwright/test";

test.use({ reducedMotion: "reduce" });

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
    await expect(page.getByRole("heading", { name: "Something went wrong" })).toBeVisible();

    // Verify the three action buttons exist
    await expect(page.getByRole("button", { name: "Try Again" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Report Issue" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Back to Home" })).toBeVisible();

    // Verify the error message is displayed
    await expect(page.getByText("Intentional test error")).toBeVisible();

    // Verify the Report Issue link points to GitHub with pre-filled data
    const reportLink = page.getByRole("link", { name: "Report Issue" });
    const href = await reportLink.getAttribute("href");
    expect(href).toContain("github.com/Develonaut/bnto/issues/new");
    expect(href).toContain("labels=bug");
  });

  test("Report Issue link includes error context", async ({ page }) => {
    await page.goto("/test-error");

    await expect(page.getByRole("heading", { name: "Something went wrong" })).toBeVisible();

    const reportLink = page.getByRole("link", { name: "Report Issue" });
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

    await expect(page.getByRole("heading", { name: "Something went wrong" })).toBeVisible();

    // Click Try Again — it calls reset() which re-renders the erroring component.
    // Since the test page always throws, the error boundary will catch again.
    await page.getByRole("button", { name: "Try Again" }).click();

    // The error boundary should still be visible (component throws again)
    await expect(page.getByRole("heading", { name: "Something went wrong" })).toBeVisible();
  });

  test("Back to Home link navigates to root", async ({ page }) => {
    await page.goto("/test-error");

    await expect(page.getByRole("heading", { name: "Something went wrong" })).toBeVisible();

    const homeLink = page.getByRole("link", { name: "Back to Home" });
    expect(await homeLink.getAttribute("href")).toBe("/");
  });
});
