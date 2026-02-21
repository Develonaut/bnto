import { test, expect } from "@playwright/test";

test.describe("Navigation", () => {
  test("sign-in page loads with a working form", async ({ page }) => {
    await page.goto("/sign-in");

    await expect(page.locator("[data-slot='card-title']")).toHaveText("Sign In");
    await expect(page.getByLabel("Email")).toBeVisible();
    await expect(page.getByLabel("Password")).toBeVisible();
    await expect(page.getByRole("button", { name: "Sign In" })).toBeVisible();

    await page.screenshot({ path: "e2e/__screenshots__/navigation/05-sign-in-form.png", fullPage: true });
  });

  test("unauthenticated /dashboard visit redirects to /sign-in", async ({ page }) => {
    await page.goto("/dashboard");

    await page.waitForURL("**/sign-in**");
    await expect(page.locator("[data-slot='card-title']")).toHaveText("Sign In");

    await page.screenshot({ path: "e2e/__screenshots__/navigation/06-dashboard-redirect.png", fullPage: true });
  });
});
