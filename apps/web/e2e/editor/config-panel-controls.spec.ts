import { test, expect } from "../fixtures";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
} from "../helpers/editor";

/**
 * Editor config panel — schema-driven form controls.
 *
 * Verifies that SchemaForm renders the correct UI control for each
 * parameter type via the CONTROL_REGISTRY lookup (select, switch,
 * slider, number, text, tagPicker, keyValue).
 *
 * Each per-operation node type has its own schema.
 * Tests verify the surfaced params for each type.
 *
 * @browser — no Convex backend needed.
 */

test.describe("config panel controls @browser", () => {
  test.beforeEach(async ({ page }) => {
    await navigateToEditor(page);
  });

  test("image node: slider for quality", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);

    // Quality = slider (bounded number 1-100)
    const qualityField = page.getByTestId("schema-field-quality");
    await qualityField.waitFor({ timeout: 5000 });
    await expect(qualityField.getByTestId("control-slider*")).toBeVisible();
  });

  test("image node: visibleWhen shows width/height for resize", async ({ page }) => {
    await addNodeFromPalette(page, "Resize Images");
    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);

    // Width should be visible (operation=resize is pre-set from palette)
    const widthField = page.getByTestId("schema-field-width");
    await widthField.waitFor({ timeout: 5000 });
    await expect(widthField).toBeVisible();

    // Width = number input (unbounded), not slider
    await expect(widthField.getByTestId("control-number*")).toBeVisible();

    // Height = number input
    await expect(
      page.getByTestId("schema-field-height").getByTestId("control-number*"),
    ).toBeVisible();

    // maintainAspect = switch (boolean)
    await expect(
      page.getByTestId("schema-field-maintainAspect").getByTestId("control-switch*"),
    ).toBeVisible();
  });

  test("spreadsheet node: switches for clean params", async ({ page }) => {
    await addNodeFromPalette(page, "Clean CSV");
    await selectNode(page, "Clean");
    await ensureConfigPanelOpen(page);

    // trimWhitespace = switch (boolean, default true)
    const trimField = page.getByTestId("schema-field-trimWhitespace");
    await trimField.waitFor({ timeout: 5000 });
    await expect(trimField.getByTestId("control-switch*")).toBeVisible();

    // removeEmptyRows = switch (boolean, default true)
    await expect(
      page.getByTestId("schema-field-removeEmptyRows").getByTestId("control-switch*"),
    ).toBeVisible();

    // removeDuplicates = switch (boolean, default true)
    await expect(
      page.getByTestId("schema-field-removeDuplicates").getByTestId("control-switch*"),
    ).toBeVisible();
  });

  test("spreadsheet node: keyValue for columns when operation=rename", async ({ page }) => {
    await addNodeFromPalette(page, "Rename CSV Columns");
    await selectNode(page, "Rename");
    await ensureConfigPanelOpen(page);

    // columns = keyValue (z.record(z.string()), visible when operation=rename)
    const columnsField = page.getByTestId("schema-field-columns");
    await columnsField.waitFor({ timeout: 5000 });
    await expect(columnsField.getByTestId("control-keyvalue*")).toBeVisible();
  });

  test("transform node: text for expression, keyValue for mappings", async ({ page }) => {
    await addNodeFromPalette(page, "Transform");
    await selectNode(page, "Transform");
    await ensureConfigPanelOpen(page);

    // expression = text (z.string())
    const expressionField = page.getByTestId("schema-field-expression");
    await expressionField.waitFor({ timeout: 5000 });
    await expect(expressionField.getByTestId("control-text*")).toBeVisible();

    // mappings = keyValue (z.record(z.string()))
    await expect(
      page.getByTestId("schema-field-mappings").getByTestId("control-keyvalue*"),
    ).toBeVisible();
  });

  test("input node: tagPicker for extensions when mode=file-upload", async ({ page }) => {
    // Select the existing input node (always present on canvas)
    await selectNode(page, "Input");
    await ensureConfigPanelOpen(page);

    // extensions = tagPicker (z.array(z.string()), visible when mode=file-upload)
    const extensionsField = page.getByTestId("schema-field-extensions");
    await extensionsField.waitFor({ timeout: 5000 });
    await expect(extensionsField.getByTestId("control-tagpicker*")).toBeVisible();
  });
});
