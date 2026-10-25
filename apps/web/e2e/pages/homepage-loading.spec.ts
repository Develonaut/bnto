import { test, expect } from "../fixtures";

/**
 * Homepage loading E2E tests (ISS-30).
 *
 * The server HTML decides what the browser fetches first. On a slow
 * connection every early request shares the bandwidth the render-blocking
 * CSS needs, so the page paints late. React preloads every eager `<img>` a
 * Server Component renders, and the browser's preload scanner fetches the
 * rest straight from the HTML, so a below-the-fold image must be lazy or it
 * competes with the CSS. The hero mascot (the LCP element) is fetched at
 * high priority.
 */

const HERO_MASCOT = "/mascots/sushi-thumbsup.svg";
const ABOVE_THE_FOLD_IMAGES = ["/favicon/Logo.svg", HERO_MASCOT];
const BELOW_THE_FOLD_MASCOTS = [
  "/mascots/octopus-chef.svg",
  "/mascots/sumo-sushi.svg",
  "/mascots/sushi-shop.svg",
];

test.describe("homepage loading @browser", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("preloads only above-the-fold images", async ({ page }) => {
    const preloads = await page
      .locator('link[rel="preload"][as="image"]')
      .evaluateAll((links) => links.map((link) => link.getAttribute("href")));

    expect([...preloads].sort()).toEqual([...ABOVE_THE_FOLD_IMAGES].sort());
  });

  test("fetches the hero mascot at high priority", async ({ page }) => {
    const hero = page.locator(`img[src="${HERO_MASCOT}"]`);

    await expect(hero).toHaveAttribute("fetchpriority", "high");
    await expect(hero).not.toHaveAttribute("loading", "lazy");
  });

  for (const src of BELOW_THE_FOLD_MASCOTS) {
    test(`lazy-loads ${src}`, async ({ page }) => {
      await expect(page.locator(`img[src="${src}"]`)).toHaveAttribute("loading", "lazy");
    });
  }
});
