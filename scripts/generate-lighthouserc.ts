/**
 * Generate lighthouserc.json dynamically from the recipe registry.
 *
 * Reads RECIPES from @bnto/nodes and generates the URL list for
 * Lighthouse CI. Accepts flags:
 *   --base-url <url>       Target URL (default: http://localhost:3000)
 *   --output <path>        Output file path (default: lighthouserc.json)
 *   --with-server          Include startServerCommand for local builds
 *
 * Usage:
 *   npx tsx scripts/generate-lighthouserc.ts
 *   npx tsx scripts/generate-lighthouserc.ts --base-url https://xyz.vercel.app
 *   npx tsx scripts/generate-lighthouserc.ts --with-server
 */

import { writeFileSync } from "fs";
import { RECIPES } from "../packages/@bnto/registry/src/recipesCatalog";

function getArg(flag: string): string | undefined {
  const idx = process.argv.indexOf(flag);
  return idx !== -1 ? process.argv[idx + 1] : undefined;
}

const baseUrl = getArg("--base-url") ?? "http://localhost:3000";
const outPath = getArg("--output") ?? "lighthouserc.json";
const withServer = process.argv.includes("--with-server");
const isPreview = baseUrl.includes(".vercel.app");

const STATIC_PAGES = ["/", "/pricing", "/faq", "/privacy"];

const recipeUrls = RECIPES.map((r) => `${baseUrl}/${r.slug}`);
const staticUrls = STATIC_PAGES.map((p) => `${baseUrl}${p}`);
const allUrls = [...staticUrls, ...recipeUrls];

const bypassSecret = process.env.VERCEL_AUTOMATION_BYPASS_SECRET;

const settings: Record<string, unknown> = {
  preset: "desktop",
  chromeFlags: "--no-sandbox --disable-gpu",
};

// Bypass Vercel Deployment Protection for preview URLs
if (bypassSecret) {
  settings.extraHeaders = JSON.stringify({
    "x-vercel-protection-bypass": bypassSecret,
  });
}

const collect: Record<string, unknown> = {
  url: allUrls,
  numberOfRuns: 1,
  settings,
};

if (withServer) {
  collect.startServerCommand = "pnpm --filter @bnto/web start";
  collect.startServerReadyPattern = "Ready in";
}

const config = {
  ci: {
    collect,
    assert: {
      assertions: {
        "categories:performance": ["warn", { minScore: 0.9 }],
        "categories:accessibility": ["error", { minScore: 0.9 }],
        "categories:best-practices": ["error", { minScore: 0.9 }],
        // Vercel previews send x-robots-tag: noindex, tanking SEO score
        "categories:seo": [isPreview ? "warn" : "error", { minScore: 0.9 }],
      },
    },
    upload: { target: "temporary-public-storage" },
  },
};

const output = JSON.stringify(config, null, 2);
writeFileSync(outPath, output + "\n");
console.log(`Generated ${outPath} with ${allUrls.length} URLs (base: ${baseUrl})`);
