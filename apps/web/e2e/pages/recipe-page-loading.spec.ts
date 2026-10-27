import { test, expect } from "../fixtures";

/**
 * Recipe page loading E2E tests (ISS-47).
 *
 * The recipe pages have the homepage's problem (see homepage-loading.spec.ts):
 * an eager `<img>` far down the page is preloaded as early as the hero and
 * competes with the render-blocking CSS. The hero mascot, the LCP element on
 * pages such as /svg-to-png, is fetched at high priority; Lighthouse's
 * `lcp-discovery-insight` assertion fails the release otherwise.
 */

const LOGO = "/favicon/Logo.svg";
const FAQ_MASCOT = "/mascots/sushi-onigiri.svg";
const TRUST_MASCOT = "/mascots/sumo-sushi.svg";

const PAGES = [
  { path: "/svg-to-png", hero: "/mascots/penguin-chef.svg" },
  { path: "/compress-images", hero: "/mascots/sushi-thumbsup.svg" },
];

for (const { path, hero } of PAGES) {
  test.describe(`${path} loading @browser`, () => {
    test.beforeEach(async ({ page }) => {
      await page.goto(path);
    });

    test("preloads only above-the-fold images", async ({ page }) => {
      const preloads = await page
        .locator('link[rel="preload"][as="image"]')
        .evaluateAll((links) => links.map((link) => link.getAttribute("href")));

      expect([...new Set(preloads)].sort()).toEqual([LOGO, hero].sort());
    });

    test("fetches the hero mascot at high priority", async ({ page }) => {
      const heroImg = page.locator(`img[src="${hero}"][fetchpriority="high"]`);

      await expect(heroImg).toHaveCount(1);
      await expect(heroImg).not.toHaveAttribute("loading", "lazy");
    });

    for (const src of [FAQ_MASCOT, TRUST_MASCOT]) {
      test(`lazy-loads ${src}`, async ({ page }) => {
        const below = page.locator(`img[src="${src}"]:not([fetchpriority="high"])`);

        await expect(below.first()).toHaveAttribute("loading", "lazy");
      });
    }
  });
}
