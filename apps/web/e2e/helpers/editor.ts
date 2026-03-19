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
 * This navigates to the tool page and clicks "Open in Editor",
 * which creates a personal recipe from the definition and navigates
 * to `/editor?recipe={id}`.
 *
 * Without a slug, navigates directly to `/editor` (blank canvas).
 *
 * Waits for the editor to render at least two node cards (I/O nodes)
 * and the render pipeline to complete (placeholder or divider visible).
 */
export async function navigateToEditor(page: Page, slug?: string) {
  if (slug) {
    // Navigate to the tool page and clone via "Open in Editor"
    await page.goto(`/${slug}`);
    const openButton = page.getByRole("button", { name: /Open in Editor/i });
    await openButton.waitFor({ timeout: 5_000 });
    await openButton.click();
    // Wait for navigation to /editor?recipe=...
    await page.waitForURL(/\/editor\?recipe=/, { timeout: 10_000 });
  } else {
    await page.goto("/editor");
  }

  // Dismiss the beta dialog if it appears — it's a modal overlay that
  // blocks canvas interaction. Tests that explicitly verify the dialog
  // (BN1, BN2) use page.goto directly and skip this helper.
  await dismissBetaDialog(page);

  // Wait for the editor to stabilize — recipe-editor testid visible
  // AND at least one node card rendered.
  const editor = page.getByTestId("recipe-editor");
  const nodeCards = page.getByTestId("node-card");

  await expect(async () => {
    await expect(editor).toBeVisible();
    const count = await nodeCards.count();
    expect(count).toBeGreaterThanOrEqual(2);
  }).toPass({ timeout: 15_000, intervals: [500, 500, 1000, 1000, 2000] });

  // Wait for the render pipeline to complete.
  // Pipeline: store.nodes → layout → execution → placeholder → dividers.
  // Blank canvas has a placeholder (no dividers). Predefined recipes
  // may have dividers instead. Wait for either.
  const pipelineReady = page.getByTestId("placeholder-node").or(page.getByTestId("add-divider"));
  await pipelineReady.first().waitFor({ timeout: 5_000 });
}

// ---------------------------------------------------------------------------
// Beta dialog
// ---------------------------------------------------------------------------

/**
 * Dismiss the editor beta dialog if visible.
 *
 * Waits briefly for the dialog to appear (it renders after hydration),
 * then clicks "Get started" and waits for it to close.
 * No-op if the dialog was already dismissed (localStorage).
 */
export async function dismissBetaDialog(page: Page) {
  const dialog = page.getByTestId("editor-beta-dialog");
  // Wait up to 3s for the dialog to appear — it renders after hydration.
  // If it doesn't appear (e.g. localStorage already has the dismissal flag),
  // this times out silently and we move on.
  try {
    await dialog.waitFor({ state: "visible", timeout: 3_000 });
  } catch {
    return; // Dialog didn't appear — already dismissed or not present
  }
  await dialog.getByTestId("beta-get-started").click();
  await expect(dialog).not.toBeVisible();
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
  // Use data-testid locators with force:true to bypass ReactFlow's
  // continuous re-rendering which detaches DOM elements between
  // Playwright's resolution and click dispatch.
  const nodeCards = page.getByTestId("node-card");
  const countBefore = await nodeCards.count();

  const placeholder = page.getByTestId("placeholder-node");
  const divider = page.getByTestId("add-divider");

  if ((await placeholder.count()) > 0) {
    await placeholder.click({ force: true });
  } else if ((await divider.count()) > 0) {
    // Divider buttons are disabled={!hovered}. Hover to trigger the parent's
    // onMouseEnter which enables the button. Retry because React's synthetic
    // mouseenter can miss under CPU load (ReactFlow continuous re-renders).
    const firstDivider = divider.first();
    let enabled = false;
    for (let attempt = 0; attempt < 5 && !enabled; attempt++) {
      await firstDivider.hover({ force: true });
      await page.waitForTimeout(200);
      enabled = await firstDivider.isEnabled();
    }
    await expect(firstDivider).toBeEnabled({ timeout: 2_000 });
    await firstDivider.click({ force: true });
  } else {
    throw new Error("addNodeFromPalette: no placeholder or divider trigger found");
  }

  // Wait for palette dialog to open
  const dialog = page.getByTestId("node-palette-dialog");
  await expect(dialog).toBeVisible({ timeout: 5_000 });

  // Click the palette item. The dialog is a React portal (outside ReactFlow),
  // so Playwright's native click works — no coordinate workaround needed.
  //
  // Note: hasText uses textContent which concatenates child spans without
  // spaces ("Compress ImagesReduce..."). The `^` anchor is sufficient —
  // \b would fail at the boundary between label and description text.
  const itemButton = dialog
    .getByTestId("palette-item-*")
    .filter({ hasText: new RegExp(`^${nodeLabel}`) });
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
  const node = page.getByTestId("node-card").filter({ hasText: new RegExp(nodeLabel) });
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
  const configField = page.getByTestId("schema-field-*").first();
  if (!(await configField.isVisible().catch(() => false))) {
    await page.getByTestId("toolbar-properties").click();
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
  const input = page.getByTestId(`control-number-param-${paramName}`);
  await input.waitFor({ timeout: 3_000 });
  await input.fill(String(value));
}

/**
 * Set a text input parameter value in the config panel.
 *
 * Requires a node to be selected and the config panel to be open.
 */
export async function setTextParam(page: Page, paramName: string, value: string) {
  const input = page.getByTestId(`control-text-param-${paramName}`);
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
  const trigger = page.getByTestId(`control-select-param-${paramName}`);
  await trigger.waitFor({ timeout: 3_000 });
  await trigger.click();
  await page.getByTestId(`select-option-${optionValue}`).click();
}

// ---------------------------------------------------------------------------
// Execution & Export (re-exported from editor-execution.ts)
// ---------------------------------------------------------------------------

export { runEditorWithFiles, openRunPanel, getResultCount, exportRecipe } from "./editor-execution";
