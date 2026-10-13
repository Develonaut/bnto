/**
 * Decide how an IndexNow submission went across all endpoints.
 *
 * IndexNow shares each submission with every participating engine, so one
 * accepted batch is enough: partial failures only warn, and the run fails
 * only when no endpoint accepted it.
 */

export type EndpointResult =
  | { endpoint: string; status: number }
  | { endpoint: string; error: string };

export function indexNowOutcome(results: EndpointResult[]) {
  const failed = results.flatMap((r) => {
    if ("error" in r) return [`${r.endpoint}: ${r.error}`];
    return r.status >= 200 && r.status < 300 ? [] : [`${r.endpoint}: HTTP ${r.status}`];
  });
  const level = failed.length === 0 ? "ok" : failed.length < results.length ? "warn" : "fail";
  return { level, failed };
}
