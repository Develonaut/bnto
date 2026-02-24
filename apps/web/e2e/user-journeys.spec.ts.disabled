import { test, expect } from "./fixtures";

test.use({ reducedMotion: "reduce" });

test.describe("Home page", () => {
  test("renders hero section", async ({ page }) => {
    await page.goto("/");

    await expect(
      page.getByRole("heading", { name: /Mainline Next\.js template/i }),
    ).toBeVisible();

    await expect(
      page.getByRole("link", { name: /Get template/i }).first(),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("home-hero.png");
  });

  test("renders logos section", async ({ page }) => {
    await page.goto("/");

    await expect(
      page.getByText("Powering the world's best product teams."),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("home-logos.png", {
      fullPage: true,
    });
  });

  test("renders bnto gallery with tool cards", async ({ page }) => {
    await page.goto("/");

    const gallery = page.getByRole("heading", {
      name: /Pick a tool\. Drop your files\./i,
    });
    await gallery.scrollIntoViewIfNeeded();
    await expect(gallery).toBeVisible();

    // Verify all 6 Tier 1 tool cards link to their pages
    await expect(
      page.getByRole("link", { name: /Compress Images/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: /Resize Images/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: /Convert Image Format/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: /Rename Files/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: /Clean CSV/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: /Rename CSV Columns/ }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("home-bnto-gallery.png");
  });

  test("renders features section", async ({ page }) => {
    await page.goto("/");

    const features = page.getByText("Made for modern product teams");
    await features.scrollIntoViewIfNeeded();
    await expect(features).toBeVisible();

    await expect(page).toHaveScreenshot("home-features.png");
  });

  test("renders pricing section", async ({ page }) => {
    await page.goto("/");

    const pricing = page.getByRole("heading", { name: "Pricing" });
    await pricing.scrollIntoViewIfNeeded();
    await expect(pricing).toBeVisible();

    // Check for pricing plan headings specifically
    await expect(page.getByRole("heading", { name: "Free" }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Startup" }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Enterprise" }).first()).toBeVisible();

    await expect(page).toHaveScreenshot("home-pricing.png");
  });

  test("renders FAQ section", async ({ page }) => {
    await page.goto("/");

    const faq = page.getByRole("heading", { name: /Got Questions/i });
    await faq.scrollIntoViewIfNeeded();
    await expect(faq).toBeVisible();

    await expect(page).toHaveScreenshot("home-faq.png");
  });

  test("footer renders with CTA and navigation", async ({ page }) => {
    await page.goto("/");

    const footer = page.getByRole("contentinfo");
    await footer.scrollIntoViewIfNeeded();

    await expect(
      footer.getByText("Start your free trial today"),
    ).toBeVisible();
    await expect(
      footer.getByRole("link", { name: /Get template/i }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("home-footer.png");
  });
});

test.describe("About page", () => {
  test("renders about hero and team sections", async ({ page }) => {
    await page.goto("/about");

    await expect(
      page.getByRole("heading", { name: /Democratising quality software/i }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("about-hero.png");

    const team = page.getByRole("heading", { name: "The team" });
    await team.scrollIntoViewIfNeeded();
    await expect(team).toBeVisible();

    await expect(page).toHaveScreenshot("about-team.png");
  });
});

test.describe("Pricing page", () => {
  test("renders pricing cards", async ({ page }) => {
    await page.goto("/pricing");

    await expect(
      page.getByRole("heading", { name: "Pricing" }),
    ).toBeVisible();

    await expect(page.getByRole("heading", { name: "Free" }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Startup" }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Enterprise" }).first()).toBeVisible();

    await expect(page).toHaveScreenshot("pricing-page.png");
  });
});

test.describe("FAQ page", () => {
  test("renders FAQ with accordion sections", async ({ page }) => {
    await page.goto("/faq");

    await expect(
      page.getByRole("heading", { name: /Got Questions/i }),
    ).toBeVisible();

    await expect(page.getByRole("heading", { name: "Support" }).first()).toBeVisible();

    await expect(page).toHaveScreenshot("faq-page.png");
  });
});

test.describe("Contact page", () => {
  test("renders contact form", async ({ page }) => {
    await page.goto("/contact");

    await expect(
      page.getByRole("heading", { name: /Contact us/i }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("contact-page.png");
  });
});

test.describe("Privacy page", () => {
  test("renders privacy policy with prose content", async ({ page }) => {
    await page.goto("/privacy");

    await expect(
      page.getByRole("heading", { name: /Privacy Policy/i }).first(),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("privacy-page.png");
  });
});

test.describe("404 page", () => {
  test("renders not found page for unknown routes", async ({ page }) => {
    await page.goto("/not-a-real-page");
    await page.waitForTimeout(1000);

    await expect(
      page.getByRole("heading", { name: /Page Not Found/i }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("404.png");
  });
});

test.describe("Navbar navigation", () => {
  test("navbar links work", async ({ page }) => {
    await page.goto("/");

    const nav = page.getByLabel("Main");

    // Navigate to About
    await nav.getByRole("link", { name: "About Us" }).click();
    await expect(page).toHaveURL(/about/);
    await expect(
      page.getByRole("heading", { name: /Democratising quality software/i }),
    ).toBeVisible();

    // Navigate to Pricing
    await nav.getByRole("link", { name: "Pricing" }).click();
    await expect(page).toHaveURL(/pricing/);

    // Navigate to FAQ
    await nav.getByRole("link", { name: "FAQ" }).click();
    await expect(page).toHaveURL(/faq/);

    // Navigate to Contact
    await nav.getByRole("link", { name: "Contact" }).click();
    await expect(page).toHaveURL(/contact/);
  });
});

test.describe("Dark mode", () => {
  test("theme toggle switches to dark mode", async ({ page }) => {
    await page.goto("/");

    await expect(
      page.getByRole("heading", { name: /Mainline Next\.js template/i }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("light-mode.png");

    // Click theme toggle
    const themeToggle = page.getByRole("button", { name: /toggle theme/i });
    await themeToggle.click();

    // Wait for theme change
    await page.waitForTimeout(500);

    await expect(page).toHaveScreenshot("dark-mode.png");
  });
});

test.describe("Bnto tool page (/compress-images)", () => {
  test("renders tool page with bnto content", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("tool-compress-images.png");
  });
});

test.describe("Auth pages", () => {
  test("signup page renders form", async ({ page }) => {
    await page.goto("/signup");

    await expect(
      page.getByRole("heading", { name: "Create an account" }),
    ).toBeVisible({ timeout: 15000 });

    await expect(page.getByPlaceholder("Your name")).toBeVisible();
    await expect(page.getByPlaceholder("Enter your email")).toBeVisible();
    await expect(page.getByPlaceholder("Enter your password")).toBeVisible();

    await expect(page).toHaveScreenshot("signup.png");
  });
});

test.describe("Protected pages (authenticated)", () => {
  test.beforeEach(async ({ context }) => {
    await context.addCookies([
      {
        name: "better-auth.session_token",
        value: "fake-session-for-e2e",
        domain: "localhost",
        path: "/",
      },
    ]);
  });

  test("workflows page", async ({ page }) => {
    await page.goto("/workflows");

    await expect(
      page.getByRole("heading", { name: "Workflows" }),
    ).toBeVisible({ timeout: 15000 });

    await expect(page).toHaveScreenshot("workflows.png");
  });

  test("executions page", async ({ page }) => {
    await page.goto("/executions");

    await expect(
      page.getByRole("heading", { name: "Executions" }),
    ).toBeVisible({ timeout: 15000 });

    await expect(page).toHaveScreenshot("executions.png");
  });

  test("settings page", async ({ page }) => {
    await page.goto("/settings");

    await expect(
      page.getByRole("heading", { name: "Settings" }),
    ).toBeVisible({ timeout: 15000 });

    await expect(page).toHaveScreenshot("settings.png");
  });
});
