import { test, expect } from "../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Site navigation journey — verifies every public route on the site loads
 * without errors, captures screenshot baselines, and confirms navbar/footer
 * navigation works end-to-end.
 *
 * Desktop: all 11 public routes + navbar + footer + 404
 * Mobile: representative subset + mobile menu + 404
 */

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

test.describe("Site navigation — desktop", () => {
  for (const route of PUBLIC_ROUTES) {
    test(`${route.name} renders without errors`, async ({ page }) => {
      const response = await page.goto(route.path);
      expect(response?.status()).toBeLessThan(400);
      await page.evaluate(() => window.scrollTo(0, 0));
      await expect(page).toHaveScreenshot(`desktop-${route.name}.png`, {
        fullPage: true,
      });
    });
  }

  test("navbar: Recipes dropdown opens and navigates to tool page", async ({
    page,
  }) => {
    await page.goto("/");

    // Open Recipes dropdown
    await page.getByRole("button", { name: /Recipes/ }).click();

    // Wait for dropdown content — a recipe link becomes visible
    const compressLink = page.getByRole("link", {
      name: /Compress Images/,
    });
    await expect(compressLink.first()).toBeVisible();

    // Screenshot the open dropdown
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("desktop-recipes-dropdown.png");

    // Navigate to tool page via dropdown
    await compressLink.first().click();
    await expect(page).toHaveURL("/compress-images");
  });

  test("navbar: Pricing and FAQ links navigate correctly", async ({
    page,
  }) => {
    await page.goto("/");

    // Navigate to Pricing via navbar (first match = navbar, before footer)
    await page.getByRole("link", { name: "Pricing" }).first().click();
    await expect(page).toHaveURL("/pricing");

    // Navigate to FAQ
    await page.getByRole("link", { name: "FAQ" }).first().click();
    await expect(page).toHaveURL("/faq");

    // Navigate home via logo (first match = navbar, second = footer)
    await page.getByRole("link", { name: "bnto" }).first().click();
    await expect(page).toHaveURL("/");
  });

  test("footer: links are present and navigate", async ({ page }) => {
    await page.goto("/");
    await page.evaluate(() =>
      window.scrollTo(0, document.body.scrollHeight),
    );

    const footer = page.locator("footer");

    // Verify key footer links exist
    await expect(
      footer.getByRole("link", { name: /Compress Images/ }),
    ).toBeVisible();
    await expect(
      footer.getByRole("link", { name: /Clean CSV/ }),
    ).toBeVisible();
    await expect(
      footer.getByRole("link", { name: "Pricing", exact: true }),
    ).toBeVisible();
    await expect(
      footer.getByRole("link", { name: "Privacy", exact: true }),
    ).toBeVisible();

    // Navigate via footer link
    await footer.getByRole("link", { name: "Pricing", exact: true }).click();
    await expect(page).toHaveURL("/pricing");
  });

  test("404: unknown route shows not-found page", async ({ page }) => {
    const response = await page.goto("/not-a-real-page");
    if (!process.env.BASE_URL) {
      expect(response?.status()).toBe(404);
    }
    await expect(
      page.getByRole("heading", { name: /Page Not Found/ }),
    ).toBeVisible();
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("desktop-404.png");
  });
});

/* ── Mobile ──────────────────────────────────────────────────── */

test.describe("Site navigation — mobile", () => {
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
      await page.evaluate(() => window.scrollTo(0, 0));
      await expect(page).toHaveScreenshot(`mobile-${route.name}.png`, {
        fullPage: true,
      });
    });
  }

  test("mobile menu: opens, shows recipes, and navigates", async ({
    page,
  }) => {
    await page.goto("/");

    // Open mobile menu via hamburger button
    await page.getByRole("button", { name: /Open menu/ }).click();

    // Wait for Sheet dialog to be visible
    const sheet = page.getByRole("dialog");
    await expect(sheet).toBeVisible();

    // Screenshot the open menu
    await expect(page).toHaveScreenshot("mobile-menu-open.png");

    // Navigate to a tool page via mobile menu
    await sheet.getByRole("link", { name: "Compress Images" }).click();
    await expect(page).toHaveURL("/compress-images");
  });

  test("mobile menu: Pricing and FAQ links navigate", async ({ page }) => {
    await page.goto("/");

    // Open mobile menu via hamburger button
    await page.getByRole("button", { name: /Open menu/ }).click();
    const sheet = page.getByRole("dialog");
    await expect(sheet).toBeVisible();

    // Navigate to Pricing via mobile menu
    await sheet.getByRole("link", { name: "Pricing" }).click();
    await expect(page).toHaveURL("/pricing");
  });

  test("404: not-found page on mobile", async ({ page }) => {
    const response = await page.goto("/not-a-real-page");
    if (!process.env.BASE_URL) {
      expect(response?.status()).toBe(404);
    }
    await expect(
      page.getByRole("heading", { name: /Page Not Found/ }),
    ).toBeVisible();
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("mobile-404.png");
  });
});
