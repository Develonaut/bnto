import { test as base, expect } from "@playwright/test";

/**
 * Whether tests are running against a remote URL (Vercel preview).
 *
 * Used by test helpers that need to adapt to the deployment environment
 * (e.g., deriving the correct cookie domain from the page URL instead
 * of hardcoding "localhost").
 */
export const isRemote = !!process.env.BASE_URL;

/**
 * Known harmless error patterns — upstream bugs, not ours.
 *
 * 1. Radix useId() hydration mismatch — React 19's useId() generates
 *    IDs from tree position. next-themes shifts the tree between SSR
 *    and hydration, so Radix IDs don't match. Upstream React 19 + Radix
 *    interaction (radix-ui/primitives#3700). No user impact.
 *
 * 2. Convex "Failed to fetch" — when E2E runs without Convex dev
 *    backend, anonymous session signIn fails with a network error.
 */
const HARMLESS_PATTERNS = [
  /hydration failed/i,
  /did not match/i,
  /server rendered HTML/i,
  /a tree hydrated but some attributes/i,
  /text content does not match/i,
  /Failed to fetch/i,
  /Failed to load resource/i,
];

function isHarmlessError(msg: string): boolean {
  return HARMLESS_PATTERNS.some((p) => p.test(msg));
}

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
 *    shadow DOM to check for `data-error="true"` on the dev badge. Known
 *    harmless errors (hydration mismatches, fetch failures) are filtered
 *    out. Only REAL errors fail the test.
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

      // Fail the test if the Next.js dev error overlay has REAL errors.
      //
      // The overlay lives inside a <nextjs-portal> shadow DOM. We pierce
      // the shadow root to check for [data-next-badge][data-error="true"].
      //
      // Strategy: when the overlay shows errors, check if ALL captured
      // errors match known harmless patterns. If yes, pass. If ANY error
      // is unrecognized, fail — that's a real bug we need to catch.
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
        const realErrors = errors.filter((e) => !isHarmlessError(e));

        if (realErrors.length > 0) {
          const errorDetail =
            `\nReal errors (${realErrors.length}):\n` +
            realErrors.map((e) => `  ${e}`).join("\n") +
            (errors.length > realErrors.length
              ? `\n\nFiltered out ${errors.length - realErrors.length} known harmless error(s)`
              : "");
          expect.soft(
            overlayError,
            `Next.js error overlay detected: "${overlayError}". ` +
              `Real runtime errors occurred during the test. ` +
              `Fix the underlying errors before proceeding.${errorDetail}`,
          ).toBeNull();
        }
        // If all errors are harmless (hydration mismatches, fetch failures),
        // the overlay detection is silently ignored. The test passes.
      }
    },
    { auto: true },
  ],
});

export { expect };
