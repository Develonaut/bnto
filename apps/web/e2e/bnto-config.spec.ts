import { test, expect } from "@playwright/test";

test.use({ reducedMotion: "reduce" });

test.describe("Per-Bnto configuration panel", () => {
  test("compress-images shows quality slider", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Config panel renders with quality slider
    await expect(page.getByText("Settings")).toBeVisible();
    await expect(page.getByText("Quality", { exact: true })).toBeVisible();
    await expect(page.getByText("80%")).toBeVisible();

    await expect(page).toHaveScreenshot("config-compress-images.png");
  });

  test("resize-images shows width input and aspect ratio toggle", async ({
    page,
  }) => {
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    await expect(page.getByText("Settings")).toBeVisible();
    await expect(page.getByText("Width (px)")).toBeVisible();
    await expect(page.getByText("Maintain aspect ratio")).toBeVisible();

    // Width input has default value
    const widthInput = page.getByRole("spinbutton");
    await expect(widthInput).toHaveValue("800");

    await expect(page).toHaveScreenshot("config-resize-images.png");
  });

  test("convert-image-format shows format selector and quality", async ({
    page,
  }) => {
    await page.goto("/convert-image-format");

    await expect(
      page.getByRole("heading", {
        name: "Convert Image Format Online Free",
      }),
    ).toBeVisible();

    await expect(page.getByText("Settings")).toBeVisible();
    await expect(page.getByText("Output format")).toBeVisible();
    await expect(page.getByText("Quality", { exact: true })).toBeVisible();

    // Default format is WebP — the select trigger shows the current value
    const selectTrigger = page.getByRole("combobox");
    await expect(selectTrigger).toBeVisible();
    await expect(selectTrigger).toHaveText("WebP");

    await expect(page).toHaveScreenshot("config-convert-format.png");
  });

  test("rename-files shows pattern input with preview", async ({ page }) => {
    await page.goto("/rename-files");

    await expect(
      page.getByRole("heading", { name: "Rename Files Online Free" }),
    ).toBeVisible();

    await expect(page.getByText("Settings")).toBeVisible();
    await expect(page.getByText("Rename pattern")).toBeVisible();

    // Pattern input has default value
    const patternInput = page.getByRole("textbox");
    await expect(patternInput).toHaveValue("renamed-{{name}}");

    // Preview shows the pattern applied
    await expect(page.getByText("Preview:")).toBeVisible();
    await expect(page.getByText("renamed-photo")).toBeVisible();

    await expect(page).toHaveScreenshot("config-rename-files.png");
  });

  test("clean-csv shows toggle switches", async ({ page }) => {
    await page.goto("/clean-csv");

    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    await expect(page.getByText("Settings")).toBeVisible();
    await expect(
      page.getByText("Trim whitespace", { exact: true }),
    ).toBeVisible();
    await expect(
      page.getByText("Remove empty rows", { exact: true }),
    ).toBeVisible();
    await expect(
      page.getByText("Remove duplicates", { exact: true }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("config-clean-csv.png");
  });

  test("rename-csv-columns shows placeholder message", async ({ page }) => {
    await page.goto("/rename-csv-columns");

    await expect(
      page.getByRole("heading", {
        name: "Rename CSV Columns Online Free",
      }),
    ).toBeVisible();

    await expect(page.getByText("Settings")).toBeVisible();
    await expect(
      page.getByText("Column remapping will be available in a future update"),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("config-rename-csv-columns.png");
  });
});
