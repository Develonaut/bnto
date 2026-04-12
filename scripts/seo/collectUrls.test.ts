/**
 * Smoke test for collectUrls. Uses node:assert (no test runner dependency).
 * Run: npx tsx scripts/seo/collectUrls.test.ts
 */

import { strict as assert } from "node:assert";
import { collectUrls } from "./collectUrls";

const urls = collectUrls();

assert.ok(urls.length > 5, `Expected more than 5 URLs, got ${urls.length}`);
assert.equal(urls[0], "https://bnto.io/");
assert.ok(urls.includes("https://bnto.io/explore"), "Missing /explore");
assert.ok(urls.includes("https://bnto.io/pricing"), "Missing /pricing");
assert.ok(urls.includes("https://bnto.io/faq"), "Missing /faq");
assert.ok(urls.includes("https://bnto.io/privacy"), "Missing /privacy");

for (const url of urls) {
  assert.match(url, /^https:\/\/bnto\.io\//, `URL not prefixed: ${url}`);
}

const unique = new Set(urls);
assert.equal(unique.size, urls.length, "Duplicate URLs found");

console.log(`collectUrls: ${urls.length} URLs, all checks passed`);
