/**
 * Type augmentation for enhanced Playwright page selectors.
 *
 * The setupEnhancedPage() fixture monkey-patches page.getByTestId() to accept
 * optional CSS modifier args (":visible", '[data-step="completed"]', etc.)
 * and prefix matching via trailing `*` (e.g. "control-slider*").
 *
 * Returned locators are also enhanced — chained .getByTestId() calls support
 * the same syntax for scoped prefix queries.
 */

import type { Locator } from "@playwright/test";

declare module "@playwright/test" {
  interface Locator {
    /**
     * Enhanced getByTestId on locators — supports modifiers and prefix matching.
     *
     * Scoped prefix match:
     *   parent.getByTestId("control-slider*")  // [data-testid^="control-slider"] within parent
     */
    getByTestId(id: string, ...modifiers: string[]): Locator;
  }

  interface Page {
    /**
     * Enhanced getByTestId — supports optional CSS modifier args and prefix matching.
     *
     * Exact match:
     *   page.getByTestId("run-button")                            // [data-testid="run-button"]
     *   page.getByTestId("run-button", ":visible")                // [data-testid="run-button"]:visible
     *   page.getByTestId("run-button", '[data-step="completed"]') // [data-testid="run-button"][data-step="completed"]
     *
     * Prefix match (trailing *):
     *   page.getByTestId("control-slider*")                       // [data-testid^="control-slider"]
     *
     * Scoped prefix (chained):
     *   page.getByTestId("schema-field-compression").getByTestId("control-slider*")
     */
    getByTestId(id: string, ...modifiers: string[]): Locator;
  }
}
