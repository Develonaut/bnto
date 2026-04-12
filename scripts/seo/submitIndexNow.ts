/**
 * Submit URLs to IndexNow endpoints (Bing, Yandex, and the shared API).
 *
 * Reads INDEXNOW_KEY from the environment. Sends a single batch POST
 * to each endpoint with the full URL list.
 */

import { collectUrls } from "./collectUrls";

const INDEXNOW_ENDPOINTS = [
  "https://api.indexnow.org/indexnow",
  "https://www.bing.com/indexnow",
  "https://yandex.com/indexnow",
];

const HOST = "bnto.io";

async function main() {
  const key = process.env.INDEXNOW_KEY;
  if (!key) {
    console.error("INDEXNOW_KEY not set — skipping IndexNow submission");
    process.exitCode = 1;
    return;
  }

  const urls = collectUrls();
  console.log(`Submitting ${urls.length} URLs to ${INDEXNOW_ENDPOINTS.length} IndexNow endpoints`);

  const body = JSON.stringify({ host: HOST, key, urlList: urls });

  const results = await Promise.allSettled(
    INDEXNOW_ENDPOINTS.map(async (endpoint) => {
      const res = await fetch(endpoint, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body,
      });
      console.log(`  ${endpoint}: ${res.status} ${res.statusText}`);
      return { endpoint, status: res.status };
    }),
  );

  const failures = results.filter((r) => r.status === "rejected");
  if (failures.length > 0) {
    console.error(`${failures.length} endpoint(s) failed`);
    process.exitCode = 1;
  }
}

main();
