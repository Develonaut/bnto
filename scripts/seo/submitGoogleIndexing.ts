/**
 * Submit URLs to Google Indexing API v3.
 *
 * Authenticates with a GCP service account (GOOGLE_INDEXING_KEY env var
 * contains the JSON key). Sends one URL_UPDATED notification per URL.
 */

import { GoogleAuth } from "google-auth-library";
import { collectUrls } from "./collectUrls";

const INDEXING_API = "https://indexing.googleapis.com/v3/urlNotifications:publish";
const SCOPES = ["https://www.googleapis.com/auth/indexing"];

async function main() {
  const keyJson = process.env.GOOGLE_INDEXING_KEY;
  if (!keyJson) {
    console.error("GOOGLE_INDEXING_KEY not set — skipping Google Indexing API");
    process.exitCode = 1;
    return;
  }

  const credentials = JSON.parse(keyJson) as { client_email: string; private_key: string };
  const auth = new GoogleAuth({ credentials, scopes: SCOPES });
  const client = await auth.getClient();

  const urls = collectUrls();
  console.log(`Submitting ${urls.length} URLs to Google Indexing API`);

  let failures = 0;
  for (const url of urls) {
    try {
      const res = await client.request({
        url: INDEXING_API,
        method: "POST",
        data: { url, type: "URL_UPDATED" },
      });
      console.log(`  ${url}: ${res.status}`);
    } catch (err: unknown) {
      failures++;
      const message = err instanceof Error ? err.message : String(err);
      console.error(`  ${url}: FAILED - ${message}`);
    }
  }

  if (failures > 0) {
    console.error(`${failures}/${urls.length} URL(s) failed`);
    process.exitCode = 1;
  }
}

main();
