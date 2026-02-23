import { test, expect } from "./fixtures";

test.describe("Theme Demo — Depth Shadows", () => {
  test.use({ reducedMotion: "reduce" });

  test("shadow scale renders directional cast shadows", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-shadow-scale"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-shadow-scale.png");
  });

  test("depth scale renders ::before pseudo-element shadows", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-depth-scale"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-depth-scale.png");
  });

  test("elevation cards show depth progression", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-elevation"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-elevation-cards.png");
  });

  test("press + shadow cards show depth effect", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-press"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-press-cards.png");
  });

  test("color palette swatches with directional shadows", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-colors"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-color-palette.png");
  });

  test("button depth states — resting, hover, active", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-button-depth"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("depth-button-states.png");
  });

  test("pressable button (Katherine Kato)", async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    const section = page.locator('[data-testid="demo-pressable"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("pressable-button.png");
  });

  test("button variants showcase", async ({ page }) => {
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

  test("notification cards with press", async ({ page }) => {
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
// Press Interaction — 3D effect verification (Katherine Kato ::before)
//
// Three states:
//   1. Resting — block floats with ::before edge + ground shadow visible
//   2. Hover   — block sinks, ::before edge shrinks, ground shadow fades
//   3. Active  — block flush with surface, edge/shadow gone
// ---------------------------------------------------------------------------

test.describe("Theme Demo — Press 3D Effect", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/dev/showcase");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(1000);
  });

  test("all depth levels at rest", async ({ page }) => {
    const section = page.locator('[data-testid="demo-press"]');
    await section.scrollIntoViewIfNeeded();
    await expect(section).toHaveScreenshot("press-all-depths-resting.png");
  });

  // Focused single-card tests on 2xl (depth=14) — largest, most visible effect
  test("resting state: ::before edge + ground shadow visible", async ({ page }) => {
    const press2xl = page.locator('[data-testid="press-2xl"]');
    await press2xl.scrollIntoViewIfNeeded();

    // Screenshot the single card at rest — edge and ground shadow should be clearly visible
    await expect(press2xl).toHaveScreenshot("press-2xl-resting.png");

    // Verify ::before has the full diagonal transform via z-axis
    const beforeTransform = await press2xl.evaluate((el) =>
      getComputedStyle(el, "::before").transform,
    );
    // matrix3d with translate3d(14, 14, -Xem) — the z-axis pushes it behind
    expect(beforeTransform).toContain("matrix3d");
  });

  test("hover state: element sinks, ::before edge shrinks", async ({ page }) => {
    const press2xl = page.locator('[data-testid="press-2xl"]');
    await press2xl.scrollIntoViewIfNeeded();

    // Capture resting position of the press element itself
    const restingRect = await press2xl.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });

    // Hover
    await press2xl.hover();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(200);

    // Screenshot — edge should be visibly thinner, card shifted
    await expect(press2xl).toHaveScreenshot("press-2xl-hover.png");

    // Element itself sunk diagonally (both X and Y increased)
    const hoveredRect = await press2xl.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });
    expect(hoveredRect.top).toBeGreaterThan(restingRect.top);
    expect(hoveredRect.left).toBeGreaterThan(restingRect.left);

    // ::before transform shrunk to ~33% of depth
    const beforeTransform = await press2xl.evaluate((el) =>
      getComputedStyle(el, "::before").transform,
    );
    expect(beforeTransform).toContain("matrix3d");
  });

  test("active state: block flush, edge minimal, shadow gone", async ({ page }) => {
    const press2xl = page.locator('[data-testid="press-2xl"]');
    await press2xl.scrollIntoViewIfNeeded();

    // Capture resting position
    const restingRect = await press2xl.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });

    // Mouse down (active state)
    await press2xl.hover();
    await page.mouse.down();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(100);

    // Screenshot — card should be nearly flush, minimal edge
    await expect(press2xl).toHaveScreenshot("press-2xl-active.png");

    // Element nearly flush — sunk by (depth - 1px) = 13px diagonally
    const activeRect = await press2xl.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });
    expect(activeRect.top - restingRect.top).toBeGreaterThan(10); // ~13px down
    expect(activeRect.left - restingRect.left).toBeGreaterThan(10); // ~13px right

    await page.mouse.up();
  });

  test("release: returns to resting position", async ({ page }) => {
    const press2xl = page.locator('[data-testid="press-2xl"]');
    await press2xl.scrollIntoViewIfNeeded();

    // Resting position
    const restingRect = await press2xl.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });

    // Hover then leave
    await press2xl.hover();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(200);
    await page.mouse.move(0, 0);
    // Wait for release transition
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(300);

    // Should be back at resting
    const returnedRect = await press2xl.evaluate((el) => {
      const r = el.getBoundingClientRect();
      return { top: r.top, left: r.left };
    });
    expect(Math.abs(returnedRect.top - restingRect.top)).toBeLessThan(2);
    expect(Math.abs(returnedRect.left - restingRect.left)).toBeLessThan(2);
  });
});
