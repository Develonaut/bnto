import { test, expect } from "../fixtures";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
  setSelectParam,
} from "../helpers/editor";

/**
 * Editor config panel — schema-driven form controls.
 *
 * Verifies that SchemaForm renders the correct UI control for each
 * parameter type via the CONTROL_REGISTRY lookup (select, switch,
 * slider, number, text, tagPicker, keyValue).
 *
 * Note: `operation` is hidden on image/spreadsheet/file-system nodes
 * (pre-set from palette). Tests verify the surfaced params instead.
 *
 * @browser — no Convex backend needed.
 */

test.describe("config panel controls @browser", () => {
  test.beforeEach(async ({ page }) => {
    await navigateToEditor(page);
  });

  test("image node: slider for compression", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);

    // Compression = slider (bounded number 1-100, visible when operation=compress)
    const compressionField = page.locator('[data-testid="schema-field-compression"]');
    await compressionField.waitFor({ timeout: 5000 });
    await expect(compressionField.locator('[data-testid^="control-slider"]')).toBeVisible();
  });

  test("image node: visibleWhen shows width/height for resize", async ({ page }) => {
    await addNodeFromPalette(page, "Resize Images");
    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);

    // Width should be visible (operation=resize is pre-set from palette)
    const widthField = page.locator('[data-testid="schema-field-width"]');
    await widthField.waitFor({ timeout: 5000 });
    await expect(widthField).toBeVisible();

    // Width = number input (unbounded), not slider
    await expect(widthField.locator('[data-testid^="control-number"]')).toBeVisible();

    // Height = number input
    await expect(
      page
        .locator('[data-testid="schema-field-height"]')
        .locator('[data-testid^="control-number"]'),
    ).toBeVisible();

    // maintainAspect = switch (boolean)
    await expect(
      page
        .locator('[data-testid="schema-field-maintainAspect"]')
        .locator('[data-testid^="control-switch"]'),
    ).toBeVisible();
  });

  test("spreadsheet node: switches for clean params", async ({ page }) => {
    await addNodeFromPalette(page, "Clean CSV");
    await selectNode(page, "Clean");
    await ensureConfigPanelOpen(page);

    // trimWhitespace = switch (boolean, default true)
    const trimField = page.locator('[data-testid="schema-field-trimWhitespace"]');
    await trimField.waitFor({ timeout: 5000 });
    await expect(trimField.locator('[data-testid^="control-switch"]')).toBeVisible();

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

  test("spreadsheet node: keyValue for columns when operation=rename", async ({ page }) => {
    await addNodeFromPalette(page, "Rename CSV Columns");
    await selectNode(page, "Rename");
    await ensureConfigPanelOpen(page);

    // columns = keyValue (z.record(z.string()), visible when operation=rename)
    const columnsField = page.locator('[data-testid="schema-field-columns"]');
    await columnsField.waitFor({ timeout: 5000 });
    await expect(columnsField.locator('[data-testid^="control-keyvalue"]')).toBeVisible();
  });

  test("transform node: text for expression, keyValue for mappings", async ({ page }) => {
    await addNodeFromPalette(page, "Transform");
    await selectNode(page, "Transform");
    await ensureConfigPanelOpen(page);

    // expression = text (z.string())
    const expressionField = page.locator('[data-testid="schema-field-expression"]');
    await expressionField.waitFor({ timeout: 5000 });
    await expect(expressionField.locator('[data-testid^="control-text"]')).toBeVisible();

    // mappings = keyValue (z.record(z.string()))
    await expect(
      page
        .locator('[data-testid="schema-field-mappings"]')
        .locator('[data-testid^="control-keyvalue"]'),
    ).toBeVisible();
  });

  test("input node: tagPicker for extensions when mode=file-upload", async ({ page }) => {
    // Select the existing input node (always present on canvas)
    await selectNode(page, "Input");
    await ensureConfigPanelOpen(page);

    // extensions = tagPicker (z.array(z.string()), visible when mode=file-upload)
    const extensionsField = page.locator('[data-testid="schema-field-extensions"]');
    await extensionsField.waitFor({ timeout: 5000 });
    await expect(extensionsField.locator('[data-testid^="control-tagpicker"]')).toBeVisible();
  });
});
