import { test, expect } from "./fixtures";

test.use({ reducedMotion: "reduce" });

const SECTION = '[data-testid="demo-bento-grid"]';

async function selectTab(
  page: import("@playwright/test").Page,
  group: string,
  value: string,
) {
  const section = page.locator(SECTION);
  const tabGroup = section.locator(`text=${group}`).locator("..").locator('[role="tablist"]');
  await tabGroup.getByRole("tab", { name: value, exact: true }).click();
  // Wait for bouncy stagger to settle
  await page.waitForTimeout(600);
}

test.describe("Bento Grid Showcase", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/dev/showcase");
    const section = page.locator(SECTION);
    await section.scrollIntoViewIfNeeded();
    await page.waitForTimeout(600);
  });

  test("default state: 6 items, 3 cols", async ({ page }) => {
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-6-items-3-cols.png");
  });

  /* ── Item count variations ───────────────────────────────────── */

  test("1 item: full-width hero", async ({ page }) => {
    await selectTab(page, "Items", "1");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-1-item.png");
  });

  test("2 items: featured + tall sidebar", async ({ page }) => {
    await selectTab(page, "Items", "2");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-2-items.png");
  });

  test("3 items: featured + 2 small", async ({ page }) => {
    await selectTab(page, "Items", "3");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-3-items.png");
  });

  test("4 items: 2×1 featured + mixed", async ({ page }) => {
    await selectTab(page, "Items", "4");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-4-items.png");
  });

  test("5 items: featured + wide card", async ({ page }) => {
    await selectTab(page, "Items", "5");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-5-items.png");
  });

  /* ── Column variations ───────────────────────────────────────── */

  test("1 column: stacked layout", async ({ page }) => {
    await selectTab(page, "Cols", "1");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-6-items-1-col.png");
  });

  test("2 columns: medium layout", async ({ page }) => {
    await selectTab(page, "Cols", "2");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-6-items-2-cols.png");
  });

  /* ── Combined variations ─────────────────────────────────────── */

  test("4 items, 2 cols", async ({ page }) => {
    await selectTab(page, "Items", "4");
    await selectTab(page, "Cols", "2");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-4-items-2-cols.png");
  });

  test("2 items, 1 col", async ({ page }) => {
    await selectTab(page, "Items", "2");
    await selectTab(page, "Cols", "1");
    const section = page.locator(SECTION);
    await expect(section).toHaveScreenshot("bento-2-items-1-col.png");
  });
});
