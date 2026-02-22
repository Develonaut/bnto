import { test, expect } from "@playwright/test";

test.use({ reducedMotion: "reduce" });

const PAGES = [
  { name: "home", path: "/" },
  { name: "about", path: "/about" },
  { name: "pricing", path: "/pricing" },
  { name: "faq", path: "/faq" },
  { name: "contact", path: "/contact" },
  { name: "privacy", path: "/privacy" },
];

// Original template at port 3200
test.describe("Original template (reference)", () => {
  for (const { name, path } of PAGES) {
    test(`${name} page`, async ({ page }) => {
      await page.goto(`http://localhost:3200${path}`);
      await page.waitForTimeout(2000);
      await expect(page).toHaveScreenshot(`original-${name}.png`, {
        fullPage: true,
        maxDiffPixelRatio: 1, // always write, don't compare
      });
    });
  }

  test("home dark mode", async ({ page }) => {
    await page.goto("http://localhost:3200/");
    await page.waitForTimeout(1000);
    const toggle = page.getByRole("button", { name: /toggle theme/i });
    await toggle.click();
    await page.waitForTimeout(500);
    await expect(page).toHaveScreenshot("original-home-dark.png", {
      fullPage: true,
      maxDiffPixelRatio: 1,
    });
  });
});

// Our drop-in at port 3100
test.describe("Bnto drop-in (comparison)", () => {
  for (const { name, path } of PAGES) {
    test(`${name} page`, async ({ page }) => {
      await page.goto(`http://localhost:3100${path}`);
      await page.waitForTimeout(2000);
      await expect(page).toHaveScreenshot(`dropin-${name}.png`, {
        fullPage: true,
        maxDiffPixelRatio: 1,
      });
    });
  }

  test("home dark mode", async ({ page }) => {
    await page.goto("http://localhost:3100/");
    await page.waitForTimeout(1000);
    const toggle = page.getByRole("button", { name: /toggle theme/i });
    await toggle.click();
    await page.waitForTimeout(500);
    await expect(page).toHaveScreenshot("dropin-home-dark.png", {
      fullPage: true,
      maxDiffPixelRatio: 1,
    });
  });
});
