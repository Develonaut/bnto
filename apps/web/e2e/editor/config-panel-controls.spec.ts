import { test, expect } from "../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Editor config panel — schema-driven form controls.
 *
 * Verifies that SchemaForm renders the correct UI control for each
 * parameter type via the CONTROL_REGISTRY lookup (select, switch,
 * slider, number, text).
 *
 * @browser — no Convex backend needed.
 */

/** Add a node by type label via the palette menu, then click it on canvas. */
async function addAndSelectNode(page: import("@playwright/test").Page, nodeLabel: string) {
  // Open palette — target the toolbar button specifically to avoid
  // ambiguity with the PlaceholderNode's "Add node" button.
  const toolbarButton = page
    .locator('[data-testid="editor-toolbar"]')
    .getByRole("button", { name: "Add node" });
  await toolbarButton.click();
  // Click the menu item button (not the category label)
  await page.getByRole("button", { name: new RegExp(`^${nodeLabel}\\s`) }).click();
  // Wait for the node to appear on canvas
  const node = page.locator('[data-testid="compartment-node"]').filter({ hasText: nodeLabel });
  await node.waitFor({ timeout: 5000 });
  // Click the node to select it and open config panel
  await node.click({ force: true });
}

test.describe("config panel controls @browser", () => {
  test.beforeEach(async ({ page }) => {
    // Enable the editor feature flag before navigating (default is off)
    await page.goto("/");
    await page.evaluate(() => localStorage.setItem("bnto:flags", JSON.stringify({ editor: true })));
    await page.goto("/editor");
    await page.waitForSelector('[data-testid="recipe-editor"]', {
      timeout: 10000,
    });
  });

  test("image node: select for operation, slider for quality", async ({ page }) => {
    await addAndSelectNode(page, "Image");

    // Toggle config panel open if needed
    const configField = page.locator('[data-testid="schema-field-operation"]');
    if (!(await configField.isVisible())) {
      // Click the config panel toggle in the toolbar
      await page.getByRole("button", { name: /config/i }).click();
    }
    await configField.waitFor({ timeout: 5000 });

    // Operation = select (enum)
    await expect(configField.locator('[data-testid^="control-select"]')).toBeVisible();

    // Quality = slider (bounded number 1-100)
    const qualityField = page.locator('[data-testid="schema-field-quality"]');
    await expect(qualityField.locator('[data-testid^="control-slider"]')).toBeVisible();
  });

  test("image node: visibleWhen shows width/height for resize", async ({ page }) => {
    await addAndSelectNode(page, "Image");

    const configField = page.locator('[data-testid="schema-field-operation"]');
    if (!(await configField.isVisible())) {
      await page.getByRole("button", { name: /config/i }).click();
    }
    await configField.waitFor({ timeout: 5000 });

    // Width/height NOT visible by default
    await expect(page.locator('[data-testid="schema-field-width"]')).not.toBeVisible();

    // Select "resize" operation
    await page.locator('[data-testid^="control-select-param-operation"]').click();
    await page.getByRole("option", { name: "resize" }).click();

    // Now width/height should appear
    await expect(page.locator('[data-testid="schema-field-width"]')).toBeVisible();
    await expect(page.locator('[data-testid="schema-field-height"]')).toBeVisible();

    // Width = number input (unbounded), not slider
    await expect(
      page.locator('[data-testid="schema-field-width"]').locator('[data-testid^="control-number"]'),
    ).toBeVisible();

    // maintainAspect = switch (boolean)
    await expect(
      page
        .locator('[data-testid="schema-field-maintainAspect"]')
        .locator('[data-testid^="control-switch"]'),
    ).toBeVisible();
  });

  test("spreadsheet node: select for operation, switches for clean params", async ({ page }) => {
    await addAndSelectNode(page, "Spreadsheet");

    const configField = page.locator('[data-testid="schema-field-operation"]');
    if (!(await configField.isVisible())) {
      await page.getByRole("button", { name: /config/i }).click();
    }
    await configField.waitFor({ timeout: 5000 });

    // Operation = select (enum with engine-backed operations: clean, rename)
    await expect(configField.locator('[data-testid^="control-select"]')).toBeVisible();

    // trimWhitespace = switch (boolean, default true)
    await expect(
      page
        .locator('[data-testid="schema-field-trimWhitespace"]')
        .locator('[data-testid^="control-switch"]'),
    ).toBeVisible();

    // removeEmptyRows = switch (boolean, default true)
    await expect(
      page
        .locator('[data-testid="schema-field-removeEmptyRows"]')
        .locator('[data-testid^="control-switch"]'),
    ).toBeVisible();

    // removeDuplicates = switch (boolean, default true)
    await expect(
      page
        .locator('[data-testid="schema-field-removeDuplicates"]')
        .locator('[data-testid^="control-switch"]'),
    ).toBeVisible();
  });
});
