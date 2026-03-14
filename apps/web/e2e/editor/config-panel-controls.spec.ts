import { test, expect } from "../fixtures";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
} from "../helpers/editor";

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

test.describe("config panel controls @browser", () => {
  test.beforeEach(async ({ page }) => {
    await navigateToEditor(page);
  });

  test("image node: select for operation, slider for quality", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);

    // Operation = select (enum)
    const configField = page.locator('[data-testid="schema-field-operation"]');
    await configField.waitFor({ timeout: 5000 });
    await expect(configField.locator('[data-testid^="control-select"]')).toBeVisible();

    // Compression = slider (bounded number 1-100, visible when operation=compress)
    const compressionField = page.locator('[data-testid="schema-field-compression"]');
    await expect(compressionField.locator('[data-testid^="control-slider"]')).toBeVisible();
  });

  test("image node: visibleWhen shows width/height for resize", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);

    const configField = page.locator('[data-testid="schema-field-operation"]');
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
    await addNodeFromPalette(page, "Clean CSV");
    await selectNode(page, "Clean");
    await ensureConfigPanelOpen(page);

    const configField = page.locator('[data-testid="schema-field-operation"]');
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
