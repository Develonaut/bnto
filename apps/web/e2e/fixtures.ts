import { test as base, expect } from "@playwright/test";
import { setupEnhancedPage } from "./fixtures/enhancedPage";
import { setupErrorCapture, hideDevToolbars, checkErrorOverlay } from "./fixtures/errorCapture";

/**
 * Shared E2E test fixture with automatic error capture and selector enforcement.
 *
 * Four automatic behaviors:
 *
 * 1. **Enhanced getByTestId** -- supports modifier args (`:visible`, `[data-phase="..."]`).
 *    Other locator methods emit a deprecation warning once per method per test.
 *
 * 2. **Console/page error capture** -- logs errors with `[e2e errors]` prefix.
 *
 * 3. **Next.js dev overlay hidden** -- hides the dev tools badge from screenshots.
 *
 * 4. **Next.js error overlay detection** -- fails the test if real errors occurred.
 *
 * Usage: import { test, expect } from "./fixtures" instead of "@playwright/test"
 */
export const test = base.extend<{ errors: string[] }>({
  page: async ({ page }, use) => {
    setupEnhancedPage(page);
    await use(page);
  },

  errors: [
    async ({ page }, use) => {
      const errors = setupErrorCapture(page);
      await hideDevToolbars(page);

      await use(errors);

      await checkErrorOverlay(page, errors);
    },
    { auto: true },
  ],
});

export { expect };
