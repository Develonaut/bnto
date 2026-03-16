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
import { RECIPES } from "../packages/@bnto/nodes/src/recipes";

function getArg(flag: string): string | undefined {
  const idx = process.argv.indexOf(flag);
  return idx !== -1 ? process.argv[idx + 1] : undefined;
}

const baseUrl = getArg("--base-url") ?? "http://localhost:3000";
const outPath = getArg("--output") ?? "lighthouserc.json";
const withServer = process.argv.includes("--with-server");

const STATIC_PAGES = ["/", "/pricing", "/faq", "/privacy"];

const recipeUrls = RECIPES.map((r) => `${baseUrl}/${r.slug}`);
const staticUrls = STATIC_PAGES.map((p) => `${baseUrl}${p}`);
const allUrls = [...staticUrls, ...recipeUrls];

const collect: Record<string, unknown> = {
  url: allUrls,
  numberOfRuns: 1,
  settings: { preset: "desktop", chromeFlags: "--no-sandbox --disable-gpu" },
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
        "categories:seo": ["error", { minScore: 0.9 }],
      },
    },
    upload: { target: "temporary-public-storage" },
  },
};

const output = JSON.stringify(config, null, 2);
writeFileSync(outPath, output + "\n");
console.log(`Generated ${outPath} with ${allUrls.length} URLs (base: ${baseUrl})`);
