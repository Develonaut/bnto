/**
 * Generate lighthouserc.json dynamically from the recipe registry.
 *
 * Reads RECIPES from @bnto/registry and generates the URL list for
 * Lighthouse CI. Accepts flags:
 *   --base-url <url>       Target URL, served elsewhere (default: http://localhost:3000)
 *   --output <path>        Output file path (default: lighthouserc.json)
 *   --with-server          Start the local build on a free port and audit that
 *
 * --with-server asks the OS for a free port rather than using Next's default
 * 3000, so audits in several checkouts at once each audit their own server
 * (lhci carries on after a server that failed to start, auditing whatever
 * holds the port). Generate the config just before `lhci autorun`, after the
 * build: something could still take the port in the second or two before
 * `next start` binds it, and generating late keeps that window short.
 *
 * Usage:
 *   npx tsx scripts/generate-lighthouserc.ts
 *   npx tsx scripts/generate-lighthouserc.ts --base-url https://xyz.vercel.app
 *   npx tsx scripts/generate-lighthouserc.ts --with-server
 */

import { writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { RECIPES } from "../packages/@bnto/registry/src/recipesCatalog";

function getArg(flag: string): string | undefined {
  const idx = process.argv.indexOf(flag);
  return idx !== -1 ? process.argv[idx + 1] : undefined;
}

/** A port nothing listens on now, as the OS assigns one for port 0. */
function freePort() {
  return new Promise<number>((resolve, reject) => {
    const server = createServer();
    server.once("error", reject);
    server.listen(0, () => {
      const address = server.address();
      if (typeof address !== "object" || !address) return reject(new Error("no port assigned"));
      server.close(() => resolve(address.port));
    });
  });
}

const STATIC_PAGES = ["/", "/pricing", "/faq", "/privacy"];

function auditUrls(baseUrl: string) {
  const recipeUrls = RECIPES.map((r) => `${baseUrl}/${r.slug}`);
  const staticUrls = STATIC_PAGES.map((p) => `${baseUrl}${p}`);
  return [...staticUrls, ...recipeUrls];
}

function auditSettings() {
  const settings: Record<string, unknown> = {
    preset: "desktop",
    chromeFlags: "--no-sandbox --disable-gpu",
  };
  // Bypass Vercel Deployment Protection for preview URLs, and skip the Vercel
  // Toolbar Vercel injects into previews: its vercel.live script sets
  // third-party cookies and logs console errors, which fail Best Practices
  // for every page even though production never serves it.
  const bypassSecret = process.env.VERCEL_AUTOMATION_BYPASS_SECRET;
  if (bypassSecret) {
    settings.extraHeaders = JSON.stringify({
      "x-vercel-protection-bypass": bypassSecret,
      "x-vercel-skip-toolbar": "1",
    });
  }
  return settings;
}

/** Starts the local build on `port`; no server for a deployed site. */
function serverOptions(port: number | undefined) {
  if (!port) return {};
  return {
    startServerCommand: `pnpm --filter @bnto/web start --port ${port}`,
    startServerReadyPattern: "Ready in",
  };
}

function assertions(isPreview: boolean) {
  return {
    "categories:performance": ["warn", { minScore: 0.9 }],
    // The LCP image must be in the HTML, eager and fetchpriority=high (ISS-30).
    // A pass/fail check, unlike LCP timing, so it can gate. N/A when LCP is text.
    "lcp-discovery-insight": ["error", { minScore: 1 }],
    "categories:accessibility": ["error", { minScore: 0.9 }],
    "categories:best-practices": ["error", { minScore: 0.9 }],
    // Vercel previews send x-robots-tag: noindex, tanking SEO score
    "categories:seo": [isPreview ? "warn" : "error", { minScore: 0.9 }],
  };
}

function writeConfig(baseUrl: string, port?: number) {
  const outPath = getArg("--output") ?? "lighthouserc.json";
  const url = auditUrls(baseUrl);
  const collect = { url, numberOfRuns: 1, settings: auditSettings(), ...serverOptions(port) };
  const config = {
    ci: {
      collect,
      assert: { assertions: assertions(baseUrl.includes(".vercel.app")) },
      upload: { target: "temporary-public-storage" },
    },
  };
  writeFileSync(outPath, JSON.stringify(config, null, 2) + "\n");
  console.log(`Generated ${outPath} with ${url.length} URLs (base: ${baseUrl})`);
}

async function main() {
  if (!process.argv.includes("--with-server")) {
    writeConfig(getArg("--base-url") ?? "http://localhost:3000");
    return;
  }
  if (getArg("--base-url")) {
    console.error("--with-server audits the local build on a free port; drop --base-url");
    process.exit(1);
  }
  const port = await freePort();
  writeConfig(`http://localhost:${port}`, port);
}

main();
