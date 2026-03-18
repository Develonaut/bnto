import { test, expect } from "../fixtures";

/**
 * Site navigation journey — verifies every public route on the site loads
 * without errors and confirms navbar/footer navigation works end-to-end.
 *
 * Desktop: all 11 public routes + navbar + footer + 404
 * Mobile: representative subset + mobile menu + 404
 */

test.use({ reducedMotion: "reduce" });

const PUBLIC_ROUTES = [
  { path: "/", name: "home" },
  { path: "/compress-images", name: "compress-images" },
  { path: "/resize-images", name: "resize-images" },
  { path: "/convert-image-format", name: "convert-image-format" },
  { path: "/rename-files", name: "rename-files" },
  { path: "/clean-csv", name: "clean-csv" },
  { path: "/rename-csv-columns", name: "rename-csv-columns" },
  { path: "/pricing", name: "pricing" },
  { path: "/faq", name: "faq" },
  { path: "/privacy", name: "privacy" },
  { path: "/signin", name: "signin" },
] as const;

/* ── Desktop ─────────────────────────────────────────────────── */

test.describe("Site navigation — desktop @browser", () => {
  for (const route of PUBLIC_ROUTES) {
    test(`${route.name} renders without errors`, async ({ page }) => {
      const response = await page.goto(route.path);
      expect(response?.status()).toBeLessThan(400);
      await expect(page.locator("body")).toBeVisible();
    });
  }

  test("navbar: Explore dropdown opens and navigates to tool page", async ({ page }) => {
    await page.goto("/");

    // Open Explore dropdown (categorized recipe links)
    await page.getByTestId("explore-button").click();

    // Find the Compress Images link in the explore dropdown
    const compressLink = page.getByTestId("explore-link-compress-images");
    await expect(compressLink).toBeVisible();

    // Navigate to tool page via dropdown
    await compressLink.click();
    await expect(page).toHaveURL("/compress-images");
  });

  test("navbar: Pricing and FAQ links navigate correctly", async ({ page }) => {
    await page.goto("/");

    // Navigate to Pricing via navbar
    await page.getByTestId("nav-link-pricing").click();
    await expect(page).toHaveURL("/pricing");

    // Navigate to FAQ
    await page.getByTestId("nav-link-faq").click();
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
    await expect(page.getByTestId("footer-link-pricing")).toBeVisible();
    await expect(page.getByTestId("footer-link-privacy")).toBeVisible();

    // Navigate via footer link
    await page.getByTestId("footer-link-pricing").click();
    await expect(page).toHaveURL("/pricing");
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

  // Representative subset — home, one tool, plus static pages
  const MOBILE_ROUTES = [
    { path: "/", name: "home" },
    { path: "/compress-images", name: "compress-images" },
    { path: "/pricing", name: "pricing" },
    { path: "/faq", name: "faq" },
    { path: "/privacy", name: "privacy" },
    { path: "/signin", name: "signin" },
  ] as const;

  for (const route of MOBILE_ROUTES) {
    test(`${route.name} renders on mobile`, async ({ page }) => {
      const response = await page.goto(route.path);
      expect(response?.status()).toBeLessThan(400);
      await expect(page.locator("body")).toBeVisible();
    });
  }

  test("mobile menu: opens, shows recipes, and navigates", async ({ page }) => {
    await page.goto("/");

    // Open mobile menu via hamburger button
    await page.getByTestId("mobile-menu-button").click();

    // Wait for Sheet dialog to be visible
    await expect(page.getByTestId("mobile-nav-dialog")).toBeVisible();

    // Navigate to a tool page via mobile menu
    await page.getByTestId("mobile-link-compress-images").click();
    await expect(page).toHaveURL("/compress-images");
  });

  test("mobile menu: Pricing and FAQ links navigate", async ({ page }) => {
    await page.goto("/");

    // Open mobile menu via hamburger button
    await page.getByTestId("mobile-menu-button").click();
    await expect(page.getByTestId("mobile-nav-dialog")).toBeVisible();

    // Navigate to Pricing via mobile menu
    await page.getByTestId("mobile-link-pricing").click();
    await expect(page).toHaveURL("/pricing");
  });

  test("404: not-found page on mobile", async ({ page }) => {
    const response = await page.goto("/not-a-real-page");
    if (!process.env.PLAYWRIGHT_BASE_URL) {
      expect(response?.status()).toBe(404);
    }
    await expect(page.getByTestId("not-found-heading")).toBeVisible();
  });
});
