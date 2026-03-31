import { test as base, expect } from "@playwright/test";
import { setupEnhancedPage } from "./fixtures/enhancedPage";
import {
  setupErrorCapture,
  hideDevToolbars,
  checkErrorOverlay,
  KNOWN_ERROR_PATTERNS,
  type KnownErrorPattern,
} from "./fixtures/errorCapture";

/**
 * Shared E2E test fixture with automatic error capture and selector enforcement.
 *
 * Five automatic behaviors:
 *
 * 1. **Enhanced getByTestId** -- supports modifier args (`:visible`, `[data-step="..."]`).
 *    Other locator methods emit a deprecation warning once per method per test.
 *
 * 2. **Console/page error capture** -- logs errors with `[e2e errors]` prefix.
 *
 * 3. **Next.js dev overlay hidden** -- hides the dev tools badge from screenshots.
 *
 * 4. **Next.js error overlay detection** -- fails the test if real errors occurred.
 *
 * 5. **Expected error silencing** -- tests opt into silencing known error patterns:
 *
 *    ```ts
 *    test.use({ expectedErrors: ["CONVEX_UNAUTH"] });
 *    ```
 *
 *    Without this, ALL console errors are captured and logged. Strict by default.
 */
export const test = base.extend<{
  errors: string[];
  expectedErrors: KnownErrorPattern[];
}>({
  expectedErrors: [[], { option: true }],

  page: async ({ page }, use) => {
    setupEnhancedPage(page);
    await use(page);
  },

  errors: [
    async ({ page, expectedErrors }, use) => {
      const silenced = expectedErrors.map((key) => KNOWN_ERROR_PATTERNS[key]);
      const errors = setupErrorCapture(page, silenced);
      await hideDevToolbars(page);

      await use(errors);

      await checkErrorOverlay(page, errors);
    },
    { auto: true },
  ],
});

export { expect };
export type { KnownErrorPattern };
