import path from "path";
import { test, expect } from "../../fixtures";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";
import {
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  IMAGE_FIXTURES_DIR,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Sign up a new user and wait until we land on home. */
async function signUp(
  page: import("@playwright/test").Page,
  email: string,
) {
  await page.goto("/signin");
  await expect(
    page.getByRole("heading", { name: "Create an account" }),
  ).toBeVisible();
  await page.getByPlaceholder("Your name").fill(TEST_NAME);
  await page.getByPlaceholder("Enter your email").fill(email);
  await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
  await page.getByRole("button", { name: "Create account" }).click();
  await page.waitForURL("/", { timeout: 15000 });
}

/** Run a recipe and wait for completion. Returns the slug used. */
async function runRecipe(page: import("@playwright/test").Page) {
  const slug = "compress-images";
  await navigateToRecipe(page, slug, "Compress Images Online Free");
  await uploadFiles(page, [
    path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
  ]);
  await runAndComplete(page);
  return slug;
}

/** Navigate to /my-recipes and wait for the History tab content. */
async function goToHistory(page: import("@playwright/test").Page) {
  await page.goto("/my-recipes");
  // History tab is the default. Wait for either an execution card or empty state.
  await expect(
    page.getByText("No runs yet").or(page.getByText("Re-run").first()),
  ).toBeVisible({ timeout: 15000 });
}

// ---------------------------------------------------------------------------
// Re-run button: unauthenticated
// ---------------------------------------------------------------------------

test.describe("Execution history re-run — unauthenticated @browser", () => {
  test("clicking re-run opens AuthGate conversion dialog", async ({
    page,
  }) => {
    // 1. Run a recipe as unauthenticated user (writes to IndexedDB)
    await runRecipe(page);

    // 2. Navigate to /my-recipes — history tab shows local history
    await goToHistory(page);

    // 3. Verify the execution appears in history
    await expect(page.getByText("Compress Images Online Free")).toBeVisible();

    // 4. Click the re-run button — should open AuthGate dialog
    const rerunButton = page.getByRole("button", { name: "Re-run" }).first();
    await expect(rerunButton).toBeVisible();
    await rerunButton.click();

    // 5. AuthGate conversion dialog should appear
    await expect(
      page.getByText("Sign up to re-run recipes"),
    ).toBeVisible({ timeout: 5000 });
    await expect(
      page.getByText("Create a free account to re-run recipes"),
    ).toBeVisible();

    // 6. Dialog has CTA buttons
    await expect(
      page.getByRole("link", { name: "Sign up free" }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Sign in" }),
    ).toBeVisible();

    // 7. Cancel dismisses the dialog
    await page.getByRole("button", { name: "Cancel" }).click();
    await expect(
      page.getByText("Sign up to re-run recipes"),
    ).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Re-run button: authenticated
// ---------------------------------------------------------------------------

test.describe("Execution history re-run — authenticated @auth", () => {
  test("clicking re-run navigates to recipe page", async ({ page }) => {
    const email = testEmail();

    // 1. Sign up
    await signUp(page, email);

    // 2. Run a recipe as authenticated user (writes to both IndexedDB + Convex)
    const slug = await runRecipe(page);

    // 3. Navigate to /my-recipes — history tab shows server history
    await goToHistory(page);

    // 4. Verify the execution appears in history
    await expect(page.getByText("Compress Images Online Free")).toBeVisible({
      timeout: 10000,
    });

    // 5. Click re-run — should navigate directly (no AuthGate dialog)
    const rerunLink = page.getByRole("link", { name: "Re-run" }).first();
    await expect(rerunLink).toBeVisible();
    await rerunLink.click();

    // 6. Should navigate to the recipe page
    await page.waitForURL(`/${slug}`, { timeout: 10000 });
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// C1: Local history migration on signup
// ---------------------------------------------------------------------------

test.describe("Execution history migration @auth", () => {
  test("C1: local history appears in server-backed history after signup", async ({
    page,
  }) => {
    // 1. Run a recipe as unauthenticated user (writes to IndexedDB)
    await runRecipe(page);

    // 2. Verify local history has the execution
    await goToHistory(page);
    await expect(page.getByText("Compress Images Online Free")).toBeVisible();

    // 3. Sign up — triggers useHistorySync migration (unauth→auth transition)
    const email = testEmail();
    await signUp(page, email);

    // 4. Navigate to /my-recipes — should now show server-backed history
    //    The migration should have moved IndexedDB entries to Convex
    await goToHistory(page);

    // 5. The same execution should appear in the server-backed history
    await expect(page.getByText("Compress Images Online Free")).toBeVisible({
      timeout: 15000,
    });

    // 6. Re-run button should work as a direct link (authenticated, no AuthGate)
    const rerunLink = page.getByRole("link", { name: "Re-run" }).first();
    await expect(rerunLink).toBeVisible();
  });
});
