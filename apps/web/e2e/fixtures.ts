import { test as base, expect } from "@playwright/test";

/**
 * Shared E2E test fixture with automatic error capture.
 *
 * Three automatic behaviors:
 *
 * 1. **Console/page error capture** — logs errors with `[e2e errors]` prefix
 *    so issues are visible in test output without inspecting screenshots.
 *
 * 2. **Next.js dev overlay hidden** — hides the dev tools badge so it doesn't
 *    pollute screenshots. The badge accumulates HMR warnings across the dev
 *    session and would appear in every full-page screenshot.
 *
 * 3. **Next.js error overlay detection** — after each test, pierces the
 *    shadow DOM to check for `data-error="true"` on the dev badge. If real
 *    errors occurred (not just warnings), the test FAILS with details.
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

      // Hide the Next.js dev tools badge from screenshots.
      // It lives in a shadow DOM inside <nextjs-portal> and accumulates
      // HMR warnings across the dev session. We inject a script that runs
      // on every page load to hide it before screenshots are taken.
      await page.addInitScript(() => {
        const hide = () => {
          const el = document.querySelector("nextjs-portal");
          if (el instanceof HTMLElement) el.style.display = "none";
        };
        if (document.body) {
          new MutationObserver(hide).observe(document.body, {
            childList: true,
            subtree: true,
          });
        } else {
          document.addEventListener("DOMContentLoaded", () => {
            hide();
            new MutationObserver(hide).observe(document.body, {
              childList: true,
              subtree: true,
            });
          });
        }
      });

      await use(errors);

      // Log captured errors for debugging
      if (errors.length > 0) {
        console.log(`\n[e2e errors] ${errors.length} error(s) captured:`);
        for (const e of errors) {
          console.log(`  ${e}`);
        }
      }

      // Fail the test if the Next.js dev error overlay has real errors.
      // The overlay lives inside a <nextjs-portal> shadow DOM. We pierce
      // the shadow root to check for [data-next-badge][data-error="true"].
      // Warnings (data-error="false") are ignored — only errors fail tests.
      const overlayError = await page.evaluate(() => {
        const portal = document.querySelector("nextjs-portal");
        if (!portal?.shadowRoot) return null;
        const badge = portal.shadowRoot.querySelector(
          '[data-next-badge][data-error="true"]',
        );
        if (!badge) return null;
        const issueEl = badge.querySelector("[data-issues-open]");
        return issueEl?.textContent?.trim() || "errors detected";
      });

      if (overlayError) {
        const errorDetail = errors.length > 0
          ? `\nCaptured errors:\n${errors.map((e) => `  ${e}`).join("\n")}`
          : "";
        expect.soft(
          overlayError,
          `Next.js error overlay detected: "${overlayError}". ` +
            `This means runtime errors occurred during the test. ` +
            `Fix the underlying errors before proceeding.${errorDetail}`,
        ).toBeNull();
      }
    },
    { auto: true },
  ],
});

export { expect };
