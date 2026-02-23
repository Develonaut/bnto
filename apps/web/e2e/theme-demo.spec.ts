import { test, expect } from "./fixtures";

test.describe("Theme Demo — Depth Shadows", () => {
  test.use({ reducedMotion: "reduce" });

  test("cards section shows static and pressable depth", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-cards"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-cards.png");
  });

  test("color palette swatches with directional shadows", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-colors"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-color-palette.png");
  });

  test("button variants (all sizes + icon)", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-buttons"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-buttons.png");
  });

  test("form elements inside elevated card", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-form"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-form-elements.png");
  });

  test("dropzone with depth", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-dropzone"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-dropzone.png");
  });

  test("animated tool grid", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-tool-grid"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-tool-grid.png");
  });

  test("notification cards with depth", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-notifications"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-notification-cards.png");
  });

  test("full page overview", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);
    await expect(page).toHaveScreenshot("depth-full-page.png", {
      fullPage: true,
    });
  });
});

// ---------------------------------------------------------------------------
// Pressable Interaction — 3D effect verification
//
// Three states:
//   1. Resting — button floats with walls + ground shadow visible
//   2. Hover   — button sinks 33%, walls shrink to 66%, shadow fades
//   3. Active  — button flush with surface, walls gone, shadow invisible
// ---------------------------------------------------------------------------

test.describe("Theme Demo — Pressable 3D Effect", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(1000);
  });

  test("pressable buttons at rest", async ({ page }) => {
    const section = page.locator('[data-testid="demo-buttons"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("pressable-all-resting.png");
  });

  // Focused tests on the large default button — most visible effect
  test("resting state: walls + ground shadow visible", async ({ page }) => {
    const target = page.locator('[data-testid="pressable-default-lg"]');
    await target.scrollIntoViewIfNeeded();

    await expect(target).toHaveScreenshot("pressable-default-lg-resting.png");

    // Verify ::before (walls) has the diagonal transform via z-axis
    const beforeTransform = await target.evaluate((el) =>
      getComputedStyle(el, "::before").transform,
    );
    expect(beforeTransform).toContain("matrix3d");
  });

  test("hover state: button sinks, walls shrink", async ({ page }) => {
    const target = page.locator('[data-testid="pressable-default-lg"]');
    await target.scrollIntoViewIfNeeded();

    // Capture resting position
    const restingRect = await target.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });

    // Hover
    await target.hover();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(200);

    // Screenshot — walls should be visibly thinner, button shifted
    await expect(target).toHaveScreenshot("pressable-default-lg-hover.png");

    // Button sunk diagonally (both X and Y increased)
    const hoveredRect = await target.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });
    expect(hoveredRect.top).toBeGreaterThan(restingRect.top);
    expect(hoveredRect.left).toBeGreaterThan(restingRect.left);
  });

  test("active state: button flush, walls gone", async ({ page }) => {
    const target = page.locator('[data-testid="pressable-default-lg"]');
    await target.scrollIntoViewIfNeeded();

    // Capture resting position
    const restingRect = await target.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });

    // Mouse down (active state)
    await target.hover();
    await page.mouse.down();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(100);

    // Screenshot — button should be flush
    await expect(target).toHaveScreenshot("pressable-default-lg-active.png");

    // Button sunk by full --depth (7px for depth-lg)
    const activeRect = await target.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });
    expect(activeRect.top - restingRect.top).toBeGreaterThan(4);
    expect(activeRect.left - restingRect.left).toBeGreaterThan(4);

    await page.mouse.up();
  });

  test("release: returns to resting position", async ({ page }) => {
    const target = page.locator('[data-testid="pressable-default-lg"]');
    await target.scrollIntoViewIfNeeded();

    // Resting position
    const restingRect = await target.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });

    // Hover then leave
    await target.hover();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(200);
    await page.mouse.move(0, 0);
    // Wait for release transition
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(300);

    // Should be back at resting
    const returnedRect = await target.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });
    expect(Math.abs(returnedRect.top - restingRect.top)).toBeLessThan(2);
    expect(Math.abs(returnedRect.left - restingRect.left)).toBeLessThan(2);
  });
});
