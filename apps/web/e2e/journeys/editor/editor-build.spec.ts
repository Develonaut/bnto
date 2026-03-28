import { test, expect } from "../../fixtures";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
} from "../../helpers/editor";

/**
 * Editor build & configure — BC1-BC4, BC8
 *
 * Tests for adding nodes, removing nodes, selecting and configuring,
 * updating params, and I/O node protection. All @browser.
 *
 * Convention: SETUP → BUILD → VERIFY.
 */

test.describe("editor build & configure @browser", () => {
  test.beforeEach(async ({ page }) => {
    await navigateToEditor(page);
  });

  test("BC1: add node from palette", async ({ page }) => {
    // Blank canvas starts with 2 I/O nodes
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(2);

    // Add an image compress node
    await addNodeFromPalette(page, "Compress Images");

    // Should now have 3 nodes: Input, Compress Images, Output
    await expect(nodeCards).toHaveCount(3);
    await expect(nodeCards.filter({ hasText: /Compress/i })).toHaveCount(1);
  });

  test("BC2: remove processing node", async ({ page }) => {
    // Add a node first
    await addNodeFromPalette(page, "Compress Images");
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3);

    // Select the processing node and wait for config panel
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);

    // Click delete button — force: true because an extended hit-area
    // span (-m-4 p-4) overlaps the button in the config panel header
    const deleteBtn = page.getByTestId("config-node-delete");
    await deleteBtn.waitFor({ timeout: 5_000 });
    await deleteBtn.click({ force: true });

    // Should be back to 2 I/O nodes
    await expect(nodeCards).toHaveCount(2);
  });

  test("BC3: select node opens config panel", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");

    // Select the node
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);

    // Config panel should show schema fields for image-compress
    const qualityField = page.getByTestId("schema-field-quality");
    await expect(qualityField).toBeVisible({ timeout: 5000 });
  });

  test("BC4: update params via config panel", async ({ page }) => {
    await addNodeFromPalette(page, "Resize Images");

    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);

    // Width field should be visible for resize operation
    const widthField = page.getByTestId("schema-field-width");
    await expect(widthField).toBeVisible();

    // Verify it has a number input control
    await expect(widthField.getByTestId("control-number*")).toBeVisible();
  });

  test("BC8: I/O nodes cannot be deleted", async ({ page }) => {
    // Click the Input node
    await selectNode(page, "Input");

    // Delete button should NOT be present for I/O nodes
    const deleteBtn = page.getByTestId("config-node-delete");
    await expect(deleteBtn).toHaveCount(0);

    // Click the Output node
    await selectNode(page, "Output");
    await expect(deleteBtn).toHaveCount(0);
  });

  test("BC5: add multiple nodes", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");
    await addNodeFromPalette(page, "Rename Files");

    const nodeCards = page.getByTestId("node-card");
    // I/O (2) + Compress + Rename = 4
    await expect(nodeCards).toHaveCount(4);
    await expect(nodeCards.filter({ hasText: /Compress/i })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: /Rename/i })).toHaveCount(1);
  });

  test("BC6: undo restores deleted node", async ({ page }) => {
    await addNodeFromPalette(page, "Compress Images");
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3);

    // Delete the processing node via config panel
    await selectNode(page, "Compress");
    await ensureConfigPanelOpen(page);
    const deleteBtn = page.getByTestId("config-node-delete");
    await deleteBtn.waitFor({ timeout: 5_000 });
    await deleteBtn.click({ force: true });
    await expect(nodeCards).toHaveCount(2);

    // Undo via keyboard shortcut (Cmd+Z on macOS / Ctrl+Z on Linux)
    await page.keyboard.press("ControlOrMeta+z");
    await expect(nodeCards).toHaveCount(3);
  });
});
