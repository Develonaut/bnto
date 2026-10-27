import { test, expect } from "../fixtures";

/**
 * Recipe page loading E2E tests (ISS-47).
 *
 * The recipe pages have the homepage's problem (see homepage-loading.spec.ts):
 * an eager `<img>` far down the page is preloaded as early as the hero and
 * competes with the render-blocking CSS. The hero mascot, the LCP element on
 * pages such as /svg-to-png, is fetched at high priority; Lighthouse's
 * `lcp-discovery-insight` assertion fails the release otherwise.
 *
 * Pages render per request, so a `loading.tsx` streams its fallback first:
 * the first paint then shows the footer, and its mascot becomes the LCP
 * image. The hero must come in the first HTML, before the footer.
 */

const LOGO = "/favicon/Logo.svg";
const FAQ_MASCOT = "/mascots/sushi-onigiri.svg";
const TRUST_MASCOT = "/mascots/sumo-sushi.svg";
const FOOTER_MASCOT = "/mascots/sushi-shop.svg";

const PAGES = [
  { path: "/svg-to-png", hero: "/mascots/penguin-chef.svg" },
  { path: "/compress-images", hero: "/mascots/sushi-thumbsup.svg" },
];

for (const { path, hero } of PAGES) {
  test(`${path} sends the hero before the footer, with no streamed fallback @browser`, async ({
    request,
  }) => {
    const html = await (await request.get(path)).text();

    expect(html).not.toContain('<template id="B:');
    expect(html.indexOf(hero)).toBeGreaterThan(-1);
    expect(html.indexOf(hero)).toBeLessThan(html.indexOf(FOOTER_MASCOT));
  });

  test.describe(`${path} loading @browser`, () => {
    test.beforeEach(async ({ page }) => {
      await page.goto(path);
    });

    test("preloads only above-the-fold images", async ({ page }) => {
      const preloads = await page
        .locator('link[rel="preload"][as="image"]')
        .evaluateAll((links) => links.map((link) => link.getAttribute("href")));

      expect([...preloads].sort()).toEqual([LOGO, hero].sort());
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
