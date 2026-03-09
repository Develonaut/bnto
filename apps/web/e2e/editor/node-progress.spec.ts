import { test, expect } from "../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Per-node progress visualization — data attribute verification.
 *
 * Uses DevTab per-node controls to force status and progress values
 * on individual nodes, then asserts `data-state` and `data-progress`
 * attributes on node cards.
 *
 * @browser — no Convex backend needed.
 */

/** Navigate to editor, load a recipe, and open the Dev tab. */
async function setupEditorWithDevTab(page: import("@playwright/test").Page) {
  await page.goto("/");
  await page.evaluate(() => localStorage.setItem("bnto:flags", JSON.stringify({ editor: true })));
  await page.goto("/editor");
  await page.waitForSelector('[data-testid="recipe-editor"]', { timeout: 10000 });

  // Load compress-images recipe via DevTab
  // First open the run panel
  const runButton = page
    .locator('[data-testid="editor-toolbar"]')
    .getByRole("button", { name: /run/i });
  await runButton.click();

  // Click the Dev tab
  await page.getByRole("tab", { name: "Dev" }).click();

  // Load a recipe so we have processing nodes
  const recipeSelect = page.locator("text=Select recipe...").first();
  await recipeSelect.click();
  await page
    .getByRole("option", { name: /compress/i })
    .first()
    .click();

  // Wait for nodes to appear
  await page.waitForSelector('[data-testid="node-card"]', { timeout: 5000 });
}

test.describe("node progress visualization @browser", () => {
  test.beforeEach(async ({ page }) => {
    await setupEditorWithDevTab(page);
  });

  test("node cards have data-state attribute reflecting idle by default", async ({ page }) => {
    const nodeCards = page.locator('[data-testid="node-card"][data-state]');
    const count = await nodeCards.count();
    expect(count).toBeGreaterThan(0);

    // All processing nodes should start as idle
    for (let i = 0; i < count; i++) {
      const state = await nodeCards.nth(i).getAttribute("data-state");
      expect(state).toBe("idle");
    }
  });

  test("forcing node status via DevTab updates data-state", async ({ page }) => {
    // Find the per-node controls section
    const perNodeSection = page.locator("text=Per-Node Controls").first();
    await expect(perNodeSection).toBeVisible();

    // Click the "pending" button on the first node
    const firstNodeControl = page
      .locator("text=Per-Node Controls")
      .locator("..")
      .locator("..")
      .locator("[class*=border-border]")
      .first();
    await firstNodeControl.getByRole("button", { name: "pending" }).click();

    // Verify a node card now has data-state="pending"
    const pendingNode = page.locator('[data-testid="node-card"][data-state="pending"]');
    await expect(pendingNode.first()).toBeVisible();

    // Step to active
    await firstNodeControl.getByRole("button", { name: "active" }).click();
    const activeNode = page.locator('[data-testid="node-card"][data-state="active"]');
    await expect(activeNode.first()).toBeVisible();

    // Step to completed
    await firstNodeControl.getByRole("button", { name: "completed" }).click();
    const completedNode = page.locator('[data-testid="node-card"][data-state="completed"]');
    await expect(completedNode.first()).toBeVisible();
  });

  test("forcing node progress via DevTab updates data-progress", async ({ page }) => {
    const perNodeSection = page.locator("text=Per-Node Controls").first();
    await expect(perNodeSection).toBeVisible();

    // Find the first node's progress slider and drag it
    const firstNodeControl = page
      .locator("text=Per-Node Controls")
      .locator("..")
      .locator("..")
      .locator("[class*=border-border]")
      .first();

    // Set status to active first so progress fill is visible
    await firstNodeControl.getByRole("button", { name: "active" }).click();

    // Use the slider — click at ~50% position
    const slider = firstNodeControl.getByRole("slider");
    await slider.click();

    // Verify data-progress is set on a node card
    const nodeWithProgress = page.locator('[data-testid="node-card"][data-progress]');
    await expect(nodeWithProgress.first()).toBeVisible();
    const progressValue = await nodeWithProgress.first().getAttribute("data-progress");
    expect(Number(progressValue)).toBeGreaterThanOrEqual(0);
  });

  test("full progression flow: pending → active → completed", async ({ page }) => {
    // Use the overall phase controls to force "running" state
    await page.getByRole("button", { name: "Running" }).click();

    // After forcing running, processing nodes should have execution state
    // Now use per-node controls to step through
    const perNodeSection = page.locator("text=Per-Node Controls").first();
    await expect(perNodeSection).toBeVisible();

    const firstNodeControl = page
      .locator("text=Per-Node Controls")
      .locator("..")
      .locator("..")
      .locator("[class*=border-border]")
      .first();

    // Step: pending
    await firstNodeControl.getByRole("button", { name: "pending" }).click();
    await expect(
      page.locator('[data-testid="node-card"][data-state="pending"]').first(),
    ).toBeVisible();

    // Step: active
    await firstNodeControl.getByRole("button", { name: "active" }).click();
    await expect(
      page.locator('[data-testid="node-card"][data-state="active"]').first(),
    ).toBeVisible();

    // Step: completed
    await firstNodeControl.getByRole("button", { name: "completed" }).click();
    await expect(
      page.locator('[data-testid="node-card"][data-state="completed"]').first(),
    ).toBeVisible();
  });
});
