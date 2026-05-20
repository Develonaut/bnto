import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import sitemap from "@astrojs/sitemap";

// https://astro.build/config
export default defineConfig({
  site: "https://ryanmchenry.me",
  integrations: [mdx(), sitemap()],
  markdown: {
    shikiConfig: {
      // Calm, paper-friendly code theme
      theme: "github-light-default",
      wrap: true,
    },
  },
  image: {
    // Allow remote images if you ever embed one; local images are preferred.
    domains: [],
  },
});
