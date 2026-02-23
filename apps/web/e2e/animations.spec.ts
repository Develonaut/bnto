import { test, expect } from "./fixtures";

/**
 * Sections whose entrance animation settles to a stable frame.
 *
 * Breathe is excluded — it loops infinitely and never stabilizes.
 */
const STABLE_SECTIONS = [
  "demo-pop-in",
  "demo-slide-in",
  "demo-stagger",
  "demo-path-draw",
  "demo-layout",
  "demo-presence",
  "demo-number-roll",
  "demo-press",
] as const;

// ---------------------------------------------------------------------------
// Settled state — baseline screenshots after entrance animations complete
// ---------------------------------------------------------------------------

test.describe("Animation Primitives — Settled", () => {
  test("entrance animations reach final state", async ({ page }) => {
    await page.goto("/dev/animations");

    // Wait for all entrance animations (springs, fades) to complete
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(2000);

    for (const section of STABLE_SECTIONS) {
      const el = page.locator(`[data-testid="${section}"]`);
      await el.scrollIntoViewIfNeeded();
      await expect(el).toHaveScreenshot(`${section}-settled.png`);
    }
  });
});

// ---------------------------------------------------------------------------
// Reduced motion — verify every component renders its static fallback
// ---------------------------------------------------------------------------

test.describe("Animation Primitives — Reduced Motion", () => {
  test.use({ reducedMotion: "reduce" });

  test("renders static fallbacks without animation wrappers", async ({
    page,
  }) => {
    await page.goto("/dev/animations");

    // Brief wait for useReducedMotion() hooks to initialize
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    for (const section of STABLE_SECTIONS) {
      const el = page.locator(`[data-testid="${section}"]`);
      await el.scrollIntoViewIfNeeded();
      await expect(el).toHaveScreenshot(`${section}-reduced.png`);
    }
  });
});

// ---------------------------------------------------------------------------
// Motion library verification — prove motion elements are wired up
// ---------------------------------------------------------------------------

test.describe("Animation Primitives — Motion Wiring", () => {
  test("PopIn elements use motion.div with spring animation", async ({
    page,
  }) => {
    await page.goto("/dev/animations");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(2000);

    // After animation settles, motion.div should have inline style applied
    // by the motion library (it manages styles directly, not via CSS classes).
    // A plain div without motion would have no inline transform/opacity.
    const popInCard = page
      .locator('[data-testid="demo-pop-in"]')
      .locator("div")
      .first();

    // motion.div gets a style attribute managed by the library
    const hasStyle = await popInCard.evaluate(
      (el) => el.hasAttribute("style") || el.style.length > 0,
    );
    expect(hasStyle).toBe(true);
  });

  test("SlideIn elements arrive at correct position", async ({ page }) => {
    await page.goto("/dev/animations");
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(2000);

    // All 4 SlideIn cards should have settled at transform: none
    const cards = page
      .locator('[data-testid="demo-slide-in"]')
      .locator(".grid > div");
    const count = await cards.count();
    expect(count).toBe(4);

    for (let i = 0; i < count; i++) {
      const transform = await cards.nth(i).evaluate(
        (el) => getComputedStyle(el).transform,
      );
      // "none" or identity matrix means animation completed
      expect(transform === "none" || transform === "matrix(1, 0, 0, 1, 0, 0)").toBe(true);
    }
  });
});

// ---------------------------------------------------------------------------
// Interactions — prove visible state changes via paired screenshots
// ---------------------------------------------------------------------------

test.describe("Animation Primitives — Interactions", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/dev/animations");
    // Let entrance animations settle before interacting
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(2000);
  });

  test("Pressable button sinks on hover", async ({ page }) => {
    const section = page.locator('[data-testid="demo-press"]');
    const target = page.locator('[data-testid="press-target"]');
    await section.scrollIntoViewIfNeeded();

    // Resting state
    await expect(section).toHaveScreenshot("press-resting.png");

    // Capture resting Y position
    const restingY = await target.evaluate(
      (el) => el.getBoundingClientRect().top,
    );

    // Hover the pressable button
    await target.hover();
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(300);

    // Hovered state — button sinks, walls shrink, shadow fades
    await expect(section).toHaveScreenshot("press-hovered.png");

    // Verify the button actually moved down (.pressable translates)
    const hoveredY = await target.evaluate(
      (el) => el.getBoundingClientRect().top,
    );
    expect(hoveredY).toBeGreaterThan(restingY);
  });

  test("PresenceGate toggles element visibility", async ({ page }) => {
    const section = page.locator('[data-testid="demo-presence"]');
    await section.scrollIntoViewIfNeeded();

    // Visible state — card should exist
    await expect(section).toHaveScreenshot("presence-visible.png");
    await expect(section.getByText("Now you see me")).toBeVisible();

    // Toggle off
    await page.click('[data-testid="presence-toggle"]');
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    // Hidden state — card should be gone
    await expect(section).toHaveScreenshot("presence-hidden.png");
    await expect(section.getByText("Now you see me")).not.toBeVisible();

    // Toggle back on
    await page.click('[data-testid="presence-toggle"]');
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(1000);

    // Restored — card should be back
    await expect(section).toHaveScreenshot("presence-restored.png");
    await expect(section.getByText("Now you see me")).toBeVisible();
  });

  test("LayoutShift changes card size on toggle", async ({ page }) => {
    const section = page.locator('[data-testid="demo-layout"]');
    await section.scrollIntoViewIfNeeded();

    // Compact state
    await expect(section).toHaveScreenshot("layout-compact.png");
    const compactHeight = await section.evaluate(
      (el) => el.getBoundingClientRect().height,
    );

    // Expand
    await page.click('[data-testid="layout-toggle"]');
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(500);

    // Expanded — should be taller
    await expect(section).toHaveScreenshot("layout-expanded.png");
    const expandedHeight = await section.evaluate(
      (el) => el.getBoundingClientRect().height,
    );
    expect(expandedHeight).toBeGreaterThan(compactHeight);
  });

  test("NumberRoll updates displayed value", async ({ page }) => {
    const section = page.locator('[data-testid="demo-number-roll"]');
    await section.scrollIntoViewIfNeeded();

    // Initial value
    await expect(section).toHaveScreenshot("number-initial.png");
    await expect(section.locator("span.font-display")).toHaveText("42");

    // Increment
    await page.click('[data-testid="counter-increment"]');
    // eslint-disable-next-line playwright/no-wait-for-timeout
    await page.waitForTimeout(800);

    // New value
    await expect(section).toHaveScreenshot("number-incremented.png");
    await expect(section.locator("span.font-display")).toHaveText("52");
  });
});
