import { test, expect } from "../fixtures";

/**
 * Site navigation journey — verifies every public route on the site loads
 * without errors and confirms navbar/footer navigation works end-to-end.
 *
 * Desktop: all public routes + navbar + footer + 404
 * Mobile: representative subset + mobile menu + 404
 */

const PUBLIC_ROUTES = [
  { path: "/", name: "home" },
  { path: "/explore", name: "explore" },
  { path: "/compress-images", name: "compress-images" },
  { path: "/resize-images", name: "resize-images" },
  { path: "/convert-image-format", name: "convert-image-format" },
  { path: "/rename-files", name: "rename-files" },
  { path: "/clean-csv", name: "clean-csv" },
  { path: "/rename-csv-columns", name: "rename-csv-columns" },
  { path: "/pricing", name: "pricing" },
  { path: "/faq", name: "faq" },
  { path: "/privacy", name: "privacy" },
] as const;

/* ── Desktop ─────────────────────────────────────────────────── */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("Site navigation — desktop @browser", () => {
  for (const route of PUBLIC_ROUTES) {
    test(`${route.name} renders without errors`, async ({ page }) => {
      const response = await page.goto(route.path);
      expect(response?.status()).toBeLessThan(400);
      await expect(page.locator("body")).toBeVisible();
    });
  }

  test("navbar: Explore dropdown opens and recipe link navigates", async ({ page }) => {
    await page.goto("/", { waitUntil: "networkidle" });

    // Open Explore dropdown — click with retry to handle hydration timing
    const exploreBtn = page.getByTestId("explore-button");
    await expect(exploreBtn).toBeVisible();
    await exploreBtn.click();
    await expect(page.getByTestId("explore-dropdown")).toBeVisible();

    // Click a recipe link inside the dropdown
    await page.getByTestId("explore-link-compress-images").click();
    await expect(page).toHaveURL("/compress-images");
  });

  test("footer: FAQ link navigates correctly", async ({ page }) => {
    await page.goto("/");
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));

    // Navigate to FAQ via footer
    await page.getByTestId("footer-link-faq").click();
    await expect(page).toHaveURL("/faq");

    // Navigate home via logo
    await page.getByTestId("nav-link-home").click();
    await expect(page).toHaveURL("/");
  });

  test("footer: links are present and navigate", async ({ page }) => {
    await page.goto("/");
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));

    // Verify key footer links exist
    await expect(page.getByTestId("footer-link-compress-images")).toBeVisible();
    await expect(page.getByTestId("footer-link-clean-csv")).toBeVisible();
    await expect(page.getByTestId("footer-link-privacy")).toBeVisible();
    await expect(page.getByTestId("footer-link-faq")).toBeVisible();

    // Navigate via footer link
    await page.getByTestId("footer-link-privacy").click();
    await expect(page).toHaveURL("/privacy");
  });

  test("404: unknown route shows not-found page", async ({ page }) => {
    const response = await page.goto("/not-a-real-page");
    if (!process.env.PLAYWRIGHT_BASE_URL) {
      expect(response?.status()).toBe(404);
    }
    await expect(page.getByTestId("not-found-heading")).toBeVisible();
  });
});

/* ── Mobile ──────────────────────────────────────────────────── */

test.describe("Site navigation — mobile @browser", () => {
  test.use({ viewport: { width: 375, height: 812 } });

  // Representative subset — home, explore, one tool, plus static pages
  const MOBILE_ROUTES = [
    { path: "/", name: "home" },
    { path: "/explore", name: "explore" },
    { path: "/compress-images", name: "compress-images" },
    { path: "/pricing", name: "pricing" },
    { path: "/faq", name: "faq" },
    { path: "/privacy", name: "privacy" },
  ] as const;

  for (const route of MOBILE_ROUTES) {
    test(`${route.name} renders on mobile`, async ({ page }) => {
      const response = await page.goto(route.path);
      expect(response?.status()).toBeLessThan(400);
      await expect(page.locator("body")).toBeVisible();
    });
  }

  test("mobile menu: opens and navigates to explore page", async ({ page }) => {
    await page.goto("/");

    // Open mobile menu via hamburger button
    await page.getByTestId("mobile-menu-button").click();

    // Wait for Sheet dialog to be visible
    await expect(page.getByTestId("mobile-nav-dialog")).toBeVisible();

    // Navigate to explore page via mobile menu
    await page.getByTestId("mobile-link-explore").click();
    await expect(page).toHaveURL("/explore");
  });

  test("footer: FAQ link navigates on mobile", async ({ page }) => {
    await page.goto("/");
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));

    // Navigate to FAQ via footer (FAQ removed from mobile menu, lives in footer)
    await page.getByTestId("footer-link-faq").click();
    await expect(page).toHaveURL("/faq");
  });

  test("404: not-found page on mobile", async ({ page }) => {
    const response = await page.goto("/not-a-real-page");
    if (!process.env.PLAYWRIGHT_BASE_URL) {
      expect(response?.status()).toBe(404);
    }
    await expect(page.getByTestId("not-found-heading")).toBeVisible();
  });
});
