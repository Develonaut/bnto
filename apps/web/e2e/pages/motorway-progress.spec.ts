import { test, expect } from "../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Motorway Progress showcase — visual regression + programmatic assertions.
 *
 * Tests the Progress tab on /motorway which renders:
 * 1. LinearProgress primitive permutations (value range, icons, labels)
 * 2. Six execution scenarios with BrowserExecutionProgress + ToolbarProgress
 *
 * Data attributes on progress components enable bulletproof programmatic
 * assertions alongside screenshot baselines.
 */

const MOTORWAY_PROGRESS_URL = "/motorway";

/** Navigate to the Progress tab and wait for content to render. */
async function gotoProgressTab(page: import("@playwright/test").Page) {
  await page.goto(MOTORWAY_PROGRESS_URL);
  await page.getByRole("tab", { name: "Progress" }).click();
  // Wait for the execution progress section to be visible
  await expect(page.locator('[data-testid="execution-progress"]')).toBeVisible();
}

/* ── Screenshot Tests ──────────────────────────────────────────── */

test.describe("Motorway Progress — screenshots", () => {
  test("progress indicators section", async ({ page }) => {
    await gotoProgressTab(page);
    const section = page.locator('[data-testid="progress-indicators"]');
    await expect(section).toBeVisible();
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(section).toHaveScreenshot("progress-indicators.png");
  });

  test("execution progress section", async ({ page }) => {
    await gotoProgressTab(page);
    const section = page.locator('[data-testid="execution-progress"]');
    await expect(section).toBeVisible();
    await expect(section).toHaveScreenshot("execution-progress.png", {
      maxDiffPixelRatio: 0.02,
    });
  });
});

/* ── Programmatic Data Attribute Assertions ────────────────────── */

test.describe("Motorway Progress — data attributes", () => {
  test("all 6 scenario cards are present", async ({ page }) => {
    await gotoProgressTab(page);

    const scenarioIds = [
      "initializing",
      "single-file-50",
      "multi-file-early",
      "multi-file-mid",
      "multi-file-near",
      "completed",
    ];

    for (const id of scenarioIds) {
      await expect(page.locator(`[data-testid="scenario-${id}"]`)).toBeVisible();
    }
  });

  test("BrowserExecutionProgress: initializing state", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-initializing"]');
    const card = scenario.locator('[data-testid="browser-execution-progress"]');

    await expect(card).toHaveAttribute("data-status", "processing");
    // No file progress — data attributes should be absent (undefined renders as no attribute)
    await expect(card).not.toHaveAttribute("data-file-index");
  });

  test("BrowserExecutionProgress: single file at 50%", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-single-file-50"]');
    const card = scenario.locator('[data-testid="browser-execution-progress"]');

    await expect(card).toHaveAttribute("data-status", "processing");
    await expect(card).toHaveAttribute("data-file-index", "0");
    await expect(card).toHaveAttribute("data-total-files", "1");
    await expect(card).toHaveAttribute("data-overall-percent", "50");
  });

  test("BrowserExecutionProgress: multi-file mid scenario", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-multi-file-mid"]');
    const card = scenario.locator('[data-testid="browser-execution-progress"]');

    await expect(card).toHaveAttribute("data-status", "processing");
    await expect(card).toHaveAttribute("data-file-index", "1");
    await expect(card).toHaveAttribute("data-total-files", "3");
    await expect(card).toHaveAttribute("data-overall-percent", "60");
  });

  test("BrowserExecutionProgress: multi-file near complete", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-multi-file-near"]');
    const card = scenario.locator('[data-testid="browser-execution-progress"]');

    await expect(card).toHaveAttribute("data-status", "processing");
    await expect(card).toHaveAttribute("data-file-index", "2");
    await expect(card).toHaveAttribute("data-total-files", "3");
    await expect(card).toHaveAttribute("data-overall-percent", "97");
  });

  test("BrowserExecutionProgress: completed state", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-completed"]');
    const card = scenario.locator('[data-testid="browser-execution-progress"]');

    await expect(card).toHaveAttribute("data-status", "completed");
  });

  test("ToolbarProgress: initializing has no file progress", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-initializing"]');
    const toolbar = scenario.locator('[data-testid="toolbar-progress"]');

    await expect(toolbar).toHaveAttribute("data-status", "processing");
    // No file-level attributes for initializing state
    await expect(toolbar).not.toHaveAttribute("data-file-index");
  });

  test("ToolbarProgress: processing scenarios have overall percent", async ({ page }) => {
    await gotoProgressTab(page);

    const cases = [
      { id: "single-file-50", percent: "50" },
      { id: "multi-file-early", percent: "20" },
      { id: "multi-file-mid", percent: "60" },
      { id: "multi-file-near", percent: "97" },
    ];

    for (const { id, percent } of cases) {
      const toolbar = page
        .locator(`[data-testid="scenario-${id}"]`)
        .locator('[data-testid="toolbar-progress"]');

      await expect(toolbar).toHaveAttribute("data-status", "processing");
      await expect(toolbar).toHaveAttribute("data-overall-percent", percent);
    }
  });

  test("ToolbarProgress: completed shows files count", async ({ page }) => {
    await gotoProgressTab(page);

    const toolbar = page
      .locator('[data-testid="scenario-completed"]')
      .locator('[data-testid="toolbar-progress"]');

    await expect(toolbar).toHaveAttribute("data-status", "completed");
    await expect(toolbar).toHaveAttribute("data-files-count", "3");
  });
});

/* ── ARIA Assertions ───────────────────────────────────────────── */

test.describe("Motorway Progress — ARIA", () => {
  test("LinearProgress elements have correct aria-valuenow", async ({ page }) => {
    await gotoProgressTab(page);

    // Check progressbar roles exist in the execution progress section.
    // Only ToolbarProgress uses LinearProgress (with role="progressbar").
    // BrowserExecutionProgress uses a custom progress bar without the ARIA role.
    // 6 scenarios × 1 toolbar view = 6 progressbar elements.
    const section = page.locator('[data-testid="execution-progress"]');
    const progressBars = section.locator('[role="progressbar"]');

    const count = await progressBars.count();
    expect(count).toBeGreaterThanOrEqual(6);

    // Spot-check: completed scenario toolbar should have aria-valuenow=100
    const completedToolbar = page
      .locator('[data-testid="scenario-completed"]')
      .locator('[data-testid="toolbar-progress"]')
      .locator('[role="progressbar"]');
    await expect(completedToolbar).toHaveAttribute("aria-valuenow", "100");

    // Spot-check: single-file-50 toolbar should have aria-valuenow=50
    const singleFileToolbar = page
      .locator('[data-testid="scenario-single-file-50"]')
      .locator('[data-testid="toolbar-progress"]')
      .locator('[role="progressbar"]');
    await expect(singleFileToolbar).toHaveAttribute("aria-valuenow", "50");
  });
});
