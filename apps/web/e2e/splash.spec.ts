import { test, expect } from "@playwright/test";

test.describe("Splash page", () => {
  test.beforeEach(async ({ context }) => {
    await context.clearCookies();
  });

  test("loads with hero content and passphrase input", async ({ page }) => {
    await page.goto("/");

    await expect(page.locator("h1")).toHaveText("Bento");
    await expect(page.getByText("Workflow automation, simplified.")).toBeVisible();
    await expect(page.getByPlaceholder("Enter passphrase")).toBeVisible();
    await expect(page.getByRole("link", { name: "Sign In" })).not.toBeVisible();

    await page.screenshot({ path: "e2e/__screenshots__/splash/01-locked.png", fullPage: true });
  });

  test("wrong passphrase shows error and does not unlock", async ({ page }) => {
    await page.goto("/");

    const input = page.getByPlaceholder("Enter passphrase");
    await input.fill("wrongpassword");
    await input.press("Enter");

    await expect(page.getByText("Nope. Try again.")).toBeVisible();
    await expect(input).toHaveValue("");
    await expect(page.getByRole("link", { name: "Sign In" })).not.toBeVisible();

    await page.screenshot({ path: "e2e/__screenshots__/splash/02-wrong-passphrase.png", fullPage: true });
  });

  test("correct passphrase reveals sign in and sign up buttons", async ({ page }) => {
    await page.goto("/");

    const input = page.getByPlaceholder("Enter passphrase");
    await input.fill("testpassphrase");
    await input.press("Enter");

    await expect(page.getByRole("link", { name: "Sign In" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Sign Up" })).toBeVisible();
    await expect(input).not.toBeVisible();

    await page.screenshot({ path: "e2e/__screenshots__/splash/03-unlocked.png", fullPage: true });
  });

  test("cookie persists unlock across refresh", async ({ page }) => {
    await page.goto("/");

    const input = page.getByPlaceholder("Enter passphrase");
    await input.fill("testpassphrase");
    await input.press("Enter");

    await expect(page.getByRole("link", { name: "Sign In" })).toBeVisible();

    await page.waitForFunction(() =>
      document.cookie.includes("bento-access=granted")
    );
    await page.reload();

    await expect(page.getByRole("link", { name: "Sign In" })).toBeVisible({ timeout: 10000 });
    await expect(page.getByRole("link", { name: "Sign Up" })).toBeVisible();
    await expect(page.getByPlaceholder("Enter passphrase")).not.toBeVisible();

    await page.screenshot({ path: "e2e/__screenshots__/splash/04-persisted-unlock.png", fullPage: true });
  });
});
