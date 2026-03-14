/**
 * Editor E2E helpers — shared utilities for editor journey tests.
 *
 * All helpers are pure Playwright interactions — no React, no store access.
 * They abstract common editor interactions so tests read as user stories.
 */

import type { Page } from "@playwright/test";
import { expect } from "../fixtures";

// ---------------------------------------------------------------------------
// Navigation
// ---------------------------------------------------------------------------

/**
 * Navigate to the editor page and wait for it to stabilize.
 *
 * For predefined recipes, pass the slug (e.g. "compress-images").
 *
 * Waits for the editor to render at least two node cards (I/O nodes)
 * and the render pipeline to complete (placeholder or divider visible).
 */
export async function navigateToEditor(page: Page, slug?: string) {
  const url = slug ? `/editor?from=${slug}` : "/editor";
  await page.goto(url);

  // Wait for the editor to stabilize — recipe-editor testid visible
  // AND at least one node card rendered.
  const editor = page.locator('[data-testid="recipe-editor"]');
  const nodeCards = page.locator('[data-testid="node-card"]');

  await expect(async () => {
    await expect(editor).toBeVisible();
    const count = await nodeCards.count();
    expect(count).toBeGreaterThanOrEqual(2);
  }).toPass({ timeout: 15_000, intervals: [500, 500, 1000, 1000, 2000] });

  // Wait for the render pipeline to complete.
  // Pipeline: store.nodes → layout → execution → placeholder → dividers.
  // Blank canvas has a placeholder (no dividers). Predefined recipes
  // may have dividers instead. Wait for either.
  const pipelineReady = page
    .locator('[data-testid="placeholder-node"]')
    .or(page.locator('[data-testid="add-divider"]'));
  await pipelineReady.first().waitFor({ timeout: 5_000 });
}

// ---------------------------------------------------------------------------
// Beta dialog
// ---------------------------------------------------------------------------

/**
 * Dismiss the editor beta dialog if visible.
 *
 * Clicks "Get started" and waits for the dialog to close.
 * No-op if the dialog was already dismissed (localStorage).
 */
export async function dismissBetaDialog(page: Page) {
  const dialog = page.locator('[data-testid="editor-beta-dialog"]');
  if (await dialog.isVisible().catch(() => false)) {
    await dialog.getByRole("button", { name: "Get started" }).click();
    await expect(dialog).not.toBeVisible();
  }
}

// ---------------------------------------------------------------------------
// Node palette
// ---------------------------------------------------------------------------

/**
 * Open the palette and add a node by its label text.
 *
 * Clicks the first available "Add node" trigger (placeholder or divider),
 * then selects the matching item in the palette dialog.
 */
export async function addNodeFromPalette(page: Page, nodeLabel: string) {
  // Two trigger types exist on the canvas:
  // - PlaceholderNode (data-testid="placeholder-node"): always enabled
  // - AddDividerNode (data-testid="add-divider"): disabled by default
  //
  // ReactFlow continuously re-renders canvas nodes, detaching DOM elements
  // between Playwright's resolution and click dispatch. Use coordinate-based
  // clicks (page.mouse.click) which bypass element stability checks.
  const nodeCards = page.locator('[data-testid="node-card"]');
  const countBefore = await nodeCards.count();

  const placeholder = page.locator('[data-testid="placeholder-node"]');
  const divider = page.locator('[data-testid="add-divider"]').first();

  let box: { x: number; y: number; width: number; height: number } | null;

  if ((await placeholder.count()) > 0) {
    box = await placeholder.boundingBox();
  } else if ((await divider.count()) > 0) {
    box = await divider.boundingBox();
  } else {
    throw new Error("addNodeFromPalette: no placeholder or divider trigger found");
  }

  if (!box) {
    throw new Error("addNodeFromPalette: trigger element has no bounding box");
  }

  // Hover first — divider buttons are `disabled={!hovered}` and only enable
  // when the mouse enters. Move the mouse to trigger onMouseEnter, then click.
  const cx = box.x + box.width / 2;
  const cy = box.y + box.height / 2;
  await page.mouse.move(cx, cy);
  await page.waitForTimeout(100); // Let React process the hover state
  await page.mouse.click(cx, cy);

  // Wait for palette dialog to open
  const dialog = page.getByRole("dialog", { name: "Add Node" });
  await expect(dialog).toBeVisible();

  // Click the palette item. The dialog is a React portal (outside ReactFlow),
  // so Playwright's native click works — no coordinate workaround needed.
  const itemButton = dialog
    .getByRole("button", { name: new RegExp(`^${nodeLabel}\\b`) });
  await itemButton.click({ timeout: 5_000 });

  // Wait for the dialog to close and a new node card to appear on canvas.
  // Node card labels differ from palette labels (e.g. palette "Compress
  // Images" → card "Compress" + sublabel "Image"), so verify by count.
  await expect(nodeCards).toHaveCount(countBefore + 1, { timeout: 5_000 });
}

// ---------------------------------------------------------------------------
// Node selection & config panel
// ---------------------------------------------------------------------------

/** Click a node on the canvas to select it and open the config panel. */
export async function selectNode(page: Page, nodeLabel: string) {
  // Wait for the node to exist, then click entirely in the page context.
  // ReactFlow re-renders can detach node elements between Playwright's
  // element resolution and click dispatch. Using page.evaluate avoids
  // this by finding and clicking in a single synchronous execution.
  const node = page
    .locator('[data-testid="node-card"]')
    .filter({ hasText: new RegExp(nodeLabel) });
  await node.waitFor({ timeout: 5_000 });
  await page.evaluate((label) => {
    const cards = document.querySelectorAll('[data-testid="node-card"]');
    for (const card of cards) {
      if (card.textContent?.includes(label)) {
        (card as HTMLElement).click();
        return;
      }
    }
  }, nodeLabel);
}

/** Ensure the config panel is visible (opens it if needed). */
export async function ensureConfigPanelOpen(page: Page) {
  const configField = page.locator('[data-testid^="schema-field-"]').first();
  if (!(await configField.isVisible().catch(() => false))) {
    await page.getByRole("button", { name: /properties/i }).click();
  }
}

// ---------------------------------------------------------------------------
// Parameter configuration
// ---------------------------------------------------------------------------

/**
 * Set a number input parameter value in the config panel.
 *
 * Requires a node to be selected and the config panel to be open.
 * Clears existing value before typing the new one.
 */
export async function setNumberParam(page: Page, paramName: string, value: number) {
  const input = page.locator(`[data-testid="control-number-param-${paramName}"]`);
  await input.waitFor({ timeout: 3_000 });
  await input.fill(String(value));
}

/**
 * Set a text input parameter value in the config panel.
 *
 * Requires a node to be selected and the config panel to be open.
 */
export async function setTextParam(page: Page, paramName: string, value: string) {
  const input = page.locator(`[data-testid="control-text-param-${paramName}"]`);
  await input.waitFor({ timeout: 3_000 });
  await input.fill(value);
}

/**
 * Select an option from a select dropdown parameter in the config panel.
 *
 * Clicks the select trigger, then clicks the matching option item.
 * Requires a node to be selected and the config panel to be open.
 */
export async function setSelectParam(page: Page, paramName: string, optionValue: string) {
  const trigger = page.locator(`[data-testid="control-select-param-${paramName}"]`);
  await trigger.waitFor({ timeout: 3_000 });
  await trigger.click();
  await page.getByRole("option", { name: optionValue }).click();
}

// ---------------------------------------------------------------------------
// Execution & Export (re-exported from editor-execution.ts)
// ---------------------------------------------------------------------------

export {
  runEditorWithFiles,
  openRunPanel,
  getResultCount,
  exportRecipe,
} from "./editor-execution";
