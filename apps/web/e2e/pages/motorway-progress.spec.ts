import { test, expect } from "../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Motorway Progress showcase — interactive simulation + visual regression.
 *
 * Tests the Progress tab on /motorway which renders:
 * 1. LinearProgress primitive permutations (value range, icons, labels)
 * 2. Interactive execution scenarios — click Run, watch progress fill,
 *    verify data attributes at completion
 *
 * The execution scenarios drive the real BrowserExecutionProgress and
 * ToolbarProgress components through a timed simulation, proving the
 * progress bar behaves correctly end-to-end.
 */

const MOTORWAY_URL = "/motorway";

/** Navigate to the Progress tab and wait for content to render. */
async function gotoProgressTab(page: import("@playwright/test").Page) {
  await page.goto(MOTORWAY_URL);
  await page.getByRole("tab", { name: "Progress" }).click();
  await expect(page.locator('[data-testid="execution-progress"]')).toBeVisible();
}

/** Run a scenario and wait for it to complete. */
async function runScenario(page: import("@playwright/test").Page, id: string) {
  const scenario = page.locator(`[data-testid="scenario-${id}"]`);
  await scenario.locator(`[data-testid="run-${id}"]`).click();
  await expect(scenario).toHaveAttribute("data-scenario-status", "completed", {
    timeout: 15_000,
  });
}

/* ── Screenshots: LinearProgress Primitives ────────────────────── */

test.describe("Motorway Progress — progress indicators", () => {
  test("LinearProgress permutations screenshot", async ({ page }) => {
    await gotoProgressTab(page);
    const section = page.locator('[data-testid="progress-indicators"]');
    await expect(section).toBeVisible();
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(section).toHaveScreenshot("progress-indicators.png");
  });
});

/* ── Interactive Execution Scenarios ───────────────────────────── */

test.describe("Motorway Progress — execution simulations", () => {
  test("all scenarios start in idle state", async ({ page }) => {
    await gotoProgressTab(page);

    for (const id of ["single-file", "three-file-batch", "five-file-batch"]) {
      const scenario = page.locator(`[data-testid="scenario-${id}"]`);
      await expect(scenario).toHaveAttribute("data-scenario-status", "idle");
      // Run button visible in idle
      await expect(scenario.locator(`[data-testid="run-${id}"]`)).toBeVisible();
    }
  });

  test("single file: run to completion", async ({ page }) => {
    await gotoProgressTab(page);
    await runScenario(page, "single-file");

    const scenario = page.locator('[data-testid="scenario-single-file"]');

    // Card view shows completed status
    const card = scenario.locator('[data-testid="browser-execution-progress"]');
    await expect(card).toHaveAttribute("data-status", "completed");

    // Toolbar shows completed with 1 file
    const toolbar = scenario.locator('[data-testid="toolbar-progress"]');
    await expect(toolbar).toHaveAttribute("data-status", "completed");
    await expect(toolbar).toHaveAttribute("data-files-count", "1");

    // Reset button appears
    await expect(scenario.locator('[data-testid="reset-single-file"]')).toBeVisible();
  });

  test("three-file batch: run to completion and verify data attributes", async ({ page }) => {
    await gotoProgressTab(page);
    await runScenario(page, "three-file-batch");

    const scenario = page.locator('[data-testid="scenario-three-file-batch"]');

    // Card completed
    const card = scenario.locator('[data-testid="browser-execution-progress"]');
    await expect(card).toHaveAttribute("data-status", "completed");

    // Toolbar completed with 3 files and savings
    const toolbar = scenario.locator('[data-testid="toolbar-progress"]');
    await expect(toolbar).toHaveAttribute("data-status", "completed");
    await expect(toolbar).toHaveAttribute("data-files-count", "3");

    // Screenshot the completed state
    await expect(scenario).toHaveScreenshot("execution-three-file-completed.png", {
      maxDiffPixelRatio: 0.02,
    });
  });

  test("five-file batch: run to completion", async ({ page }) => {
    await gotoProgressTab(page);
    await runScenario(page, "five-file-batch");

    const scenario = page.locator('[data-testid="scenario-five-file-batch"]');

    const toolbar = scenario.locator('[data-testid="toolbar-progress"]');
    await expect(toolbar).toHaveAttribute("data-status", "completed");
    await expect(toolbar).toHaveAttribute("data-files-count", "5");
  });

  test("reset returns to idle", async ({ page }) => {
    await gotoProgressTab(page);

    // Run then reset
    await runScenario(page, "single-file");
    const scenario = page.locator('[data-testid="scenario-single-file"]');
    await scenario.locator('[data-testid="reset-single-file"]').click();
    await expect(scenario).toHaveAttribute("data-scenario-status", "idle");

    // Run button reappears
    await expect(scenario.locator('[data-testid="run-single-file"]')).toBeVisible();
  });

  test("progress bar advances during processing", async ({ page }) => {
    await gotoProgressTab(page);

    const scenario = page.locator('[data-testid="scenario-three-file-batch"]');
    await scenario.locator('[data-testid="run-three-file-batch"]').click();

    // Wait for processing to start — card should show processing status
    const card = scenario.locator('[data-testid="browser-execution-progress"]');
    await expect(card).toHaveAttribute("data-status", "processing");

    // Overall percent should be a number > 0 at some point during processing
    // Poll until we see a non-zero percent or it completes
    await expect(async () => {
      const status = await scenario.getAttribute("data-scenario-status");
      if (status === "completed") return; // finished fast, still valid
      const percent = await card.getAttribute("data-overall-percent");
      expect(Number(percent)).toBeGreaterThan(0);
    }).toPass({ timeout: 10_000 });

    // Wait for completion
    await expect(scenario).toHaveAttribute("data-scenario-status", "completed", {
      timeout: 15_000,
    });
  });
});

/* ── ARIA ──────────────────────────────────────────────────────── */

test.describe("Motorway Progress — ARIA", () => {
  test("completed toolbar has progressbar at 100%", async ({ page }) => {
    await gotoProgressTab(page);
    await runScenario(page, "single-file");

    const toolbar = page
      .locator('[data-testid="scenario-single-file"]')
      .locator('[data-testid="toolbar-progress"]')
      .locator('[role="progressbar"]');

    await expect(toolbar).toHaveAttribute("aria-valuenow", "100");
  });
});
