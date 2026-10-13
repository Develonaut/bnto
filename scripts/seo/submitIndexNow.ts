/**
 * Submit URLs to IndexNow endpoints (Bing, Yandex, and the shared API).
 *
 * Reads INDEXNOW_KEY from the environment. Sends a single batch POST
 * to each endpoint with the full URL list.
 */

import { collectUrls } from "./collectUrls";
import { indexNowOutcome, type EndpointResult } from "./indexNowOutcome";

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

  const results = await Promise.all(INDEXNOW_ENDPOINTS.map((endpoint) => submit(endpoint, body)));
  report(indexNowOutcome(results));
}

/** POST the batch to one endpoint; network errors become a result, not a throw. */
async function submit(endpoint: string, body: string): Promise<EndpointResult> {
  try {
    const res = await fetch(endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body,
    });
    console.log(`  ${endpoint}: ${res.status} ${res.statusText}`);
    return { endpoint, status: res.status };
  } catch (e) {
    const error = e instanceof Error ? (e.cause?.toString() ?? e.message) : String(e);
    console.log(`  ${endpoint}: ${error}`);
    return { endpoint, error };
  }
}

/** Warn on partial failure (IndexNow shares submissions); fail only if none got through. */
function report({ level, failed }: ReturnType<typeof indexNowOutcome>) {
  if (level === "ok") return;
  const detail = failed.join("; ");
  if (level === "warn") {
    console.log(`::warning::IndexNow: ${failed.length} endpoint(s) failed (${detail})`);
    return;
  }
  console.error(`IndexNow: every endpoint failed (${detail})`);
  process.exitCode = 1;
}

main();
