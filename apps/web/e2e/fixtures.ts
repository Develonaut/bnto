import { test as base, expect } from "@playwright/test";

/**
 * Shared E2E test fixture with automatic error capture.
 *
 * Captures console errors and page errors during each test, then logs
 * them with an `[e2e errors]` prefix so agents can programmatically
 * detect issues without visually inspecting every screenshot.
 *
 * Usage: import { test, expect } from "./fixtures" instead of "@playwright/test"
 */
export const test = base.extend<{ errors: string[] }>({
  errors: [
    async ({ page }, use) => {
      const errors: string[] = [];

      page.on("console", (msg) => {
        if (msg.type() === "error") {
          errors.push(`[console] ${msg.text()}`);
        }
      });

      page.on("pageerror", (err) => {
        errors.push(`[pageerror] ${err.message}`);
      });

      await use(errors);

      if (errors.length > 0) {
        console.log(`\n[e2e errors] ${errors.length} error(s) captured:`);
        for (const e of errors) {
          console.log(`  ${e}`);
        }
      }
    },
    { auto: true },
  ],
});

export { expect };
