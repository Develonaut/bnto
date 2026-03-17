import type { Locator, Page } from "@playwright/test";

/**
 * Playwright selector methods that are deprecated in favor of getByTestId().
 *
 * Standard:  page.getByTestId("run-button")
 * Enhanced:  page.getByTestId("run-button", ":visible", '[data-phase="completed"]')
 * Prefix:    page.getByTestId("control-slider*")  -- startsWith match
 * Deprecated: page.getByRole("button", { name: "Run" })
 *
 * These methods still work but emit a console warning to encourage migration.
 * Suppress at lint level: // eslint-disable-next-line -- REASON
 */
const DEPRECATED_SELECTORS = [
  "getByRole",
  "getByText",
  "getByPlaceholder",
  "getByLabel",
  "getByAltText",
  "getByTitle",
] as const;

/**
 * Build a data-testid CSS selector from an id and optional modifiers.
 *
 * - Exact match:  buildSelector("run-button")         -> [data-testid="run-button"]
 * - Prefix match: buildSelector("control-slider*")    -> [data-testid^="control-slider"]
 * - With mods:    buildSelector("run-button", ":visible") -> [data-testid="run-button"]:visible
 */
function buildSelector(id: string, modifiers: string[]): string {
  const isPrefix = id.endsWith("*");
  const cleanId = isPrefix ? id.slice(0, -1) : id;
  const op = isPrefix ? "^=" : "=";
  return `[data-testid${op}"${cleanId}"]${modifiers.join("")}`;
}

/**
 * Wrap a Playwright Locator so its `.getByTestId()` method supports the
 * same enhanced syntax (modifiers, prefix matching) as page.getByTestId().
 *
 * Uses a Proxy to intercept only `getByTestId` — all other Locator
 * methods pass through untouched.
 */
function enhanceLocator(locator: Locator): Locator {
  return new Proxy(locator, {
    get(target, prop, receiver) {
      if (prop === "getByTestId") {
        return (id: string, ...modifiers: string[]) => {
          const child = target.locator(buildSelector(id, modifiers));
          return enhanceLocator(child);
        };
      }
      return Reflect.get(target, prop, receiver);
    },
  }) as Locator;
}

/**
 * Enhance Playwright's page object with two features:
 *
 * 1. **Enhanced getByTestId** -- hijacks the native `getByTestId` to support
 *    modifier args (`:visible`, `[data-phase="..."]`) and prefix matching
 *    (trailing `*`). Builds a CSS selector under the hood and calls
 *    `page.locator()`. Returned locators are also enhanced so chained
 *    `.getByTestId()` calls support the same syntax.
 *
 * 2. **Deprecation warnings** -- wraps other locator methods (getByRole, etc.)
 *    to emit a one-time warning per method per test encouraging migration to
 *    getByTestId.
 */
export function setupEnhancedPage(page: Page): void {
  // ── Enhanced getByTestId ─────────────────────────────────────────────
  // Hijack Playwright's native getByTestId to support modifier args
  // and prefix matching:
  //
  //   page.getByTestId("run-button")                            -> [data-testid="run-button"]
  //   page.getByTestId("run-button", ":visible")                -> [data-testid="run-button"]:visible
  //   page.getByTestId("run-button", '[data-phase="completed"]') -> [data-testid="run-button"][data-phase="completed"]
  //   page.getByTestId("control-slider*")                       -> [data-testid^="control-slider"]
  //
  // Under the hood it builds a CSS selector and calls page.locator().
  // This is THE standard selector method for all bnto E2E tests.
  //
  // Returned locators are wrapped with enhanceLocator() so scoped
  // .getByTestId() calls also support modifiers and prefix matching:
  //
  //   page.getByTestId("schema-field-compression").getByTestId("control-slider*")
  //
  (page as unknown as Record<string, unknown>).getByTestId = (
    id: string,
    ...modifiers: string[]
  ) => {
    return enhanceLocator(page.locator(buildSelector(id, modifiers)));
  };

  // ── Deprecation warnings ─────────────────────────────────────────────
  const warned = new Set<string>();

  for (const method of DEPRECATED_SELECTORS) {
    const original = (page as unknown as Record<string, unknown>)[method] as (
      ...args: unknown[]
    ) => unknown;
    (page as unknown as Record<string, unknown>)[method] = (...args: unknown[]) => {
      if (!warned.has(method)) {
        const sep = "=".repeat(70);
        console.warn(
          `\n${sep}\n` +
            `⚠️  DEPRECATED SELECTOR: page.${method}()\n\n` +
            `   Use page.getByTestId("...") instead.\n` +
            `   Add data-testid to the source component if missing.\n\n` +
            `   Suppress at lint level with a comment explaining why:\n` +
            `   // eslint-disable-next-line -- REASON\n` +
            `${sep}\n`,
        );
        warned.add(method);
      }
      return original.call(page, ...args);
    };
  }
}
