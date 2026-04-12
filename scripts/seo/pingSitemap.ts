/**
 * Ping Google with the sitemap URL.
 *
 * Note: Google deprecated the sitemap ping endpoint in June 2023.
 * The endpoint returns 404 — we keep this script for documentation
 * and in case Google re-enables it. A 404 is treated as expected.
 */

const SITEMAP_URL = "https://bnto.io/sitemap.xml";
const PING_URL = `https://www.google.com/ping?sitemap=${encodeURIComponent(SITEMAP_URL)}`;

async function main() {
  console.log(`Pinging Google with sitemap: ${SITEMAP_URL}`);
  const res = await fetch(PING_URL);
  console.log(`Response: ${res.status} ${res.statusText}`);
  if (res.status === 404) {
    console.log("(Google deprecated sitemap ping — this is expected)");
  }
}

main();
