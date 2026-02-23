import { test, expect } from "./fixtures";

test.use({ reducedMotion: "reduce" });

test.describe("File drop interface", () => {
  test("renders drop zone on compress-images page", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    await expect(page.getByText("Drag & drop files here")).toBeVisible();

    await expect(page.getByText("JPEG, PNG, or WebP images")).toBeVisible();

    await expect(page).toHaveScreenshot("file-drop-compress-images.png");
  });

  test("renders drop zone on clean-csv page", async ({ page }) => {
    await page.goto("/clean-csv");

    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible({ timeout: 15000 });

    await expect(page.getByText("Drag & drop files here")).toBeVisible();

    await expect(page.getByText("CSV files")).toBeVisible();

    await expect(page).toHaveScreenshot("file-drop-clean-csv.png");
  });

  test("renders drop zone on rename-files page", async ({ page }) => {
    await page.goto("/rename-files");

    await expect(
      page.getByRole("heading", { name: "Rename Files Online Free" }),
    ).toBeVisible();

    await expect(page.getByText("any files")).toBeVisible();

    await expect(page).toHaveScreenshot("file-drop-rename-files.png");
  });

  test("shows selected files after upload", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');

    await fileInput.setInputFiles([
      {
        name: "photo1.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
      {
        name: "photo2.png",
        mimeType: "image/png",
        buffer: Buffer.from("fake-png-content"),
      },
    ]);

    await expect(page.getByText("2 files selected")).toBeVisible();
    await expect(page.getByText("photo1.jpg")).toBeVisible();
    await expect(page.getByText("photo2.png")).toBeVisible();

    await expect(page).toHaveScreenshot("file-drop-with-files.png");
  });

  test("removes individual files", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');

    await fileInput.setInputFiles([
      {
        name: "photo1.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
      {
        name: "photo2.png",
        mimeType: "image/png",
        buffer: Buffer.from("fake-png-content"),
      },
    ]);

    await expect(page.getByText("2 files selected")).toBeVisible();

    const removeButton = page.getByRole("button", { name: "Remove photo1.jpg" });
    await removeButton.click();

    await expect(page.getByText("1 file selected")).toBeVisible();
    await expect(page.getByText("photo1.jpg")).not.toBeVisible();
    await expect(page.getByText("photo2.png")).toBeVisible();
  });

  test("clears all files", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');

    await fileInput.setInputFiles([
      {
        name: "photo1.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    await page.getByRole("button", { name: "Clear all" }).click();

    await expect(page.getByText("1 file selected")).not.toBeVisible();
    await expect(page.getByText("photo1.jpg")).not.toBeVisible();
  });
});
