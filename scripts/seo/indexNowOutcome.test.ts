/**
 * Tests for indexNowOutcome. Uses node:assert (no test runner dependency).
 * Run: npx tsx scripts/seo/indexNowOutcome.test.ts
 */

import { strict as assert } from "node:assert";
import { indexNowOutcome } from "./indexNowOutcome";

const ok = (endpoint: string, status = 200) => ({ endpoint, status });
const err = (endpoint: string, reason: string) => ({ endpoint, error: reason });

// Every endpoint accepted the batch.
assert.deepEqual(indexNowOutcome([ok("a"), ok("b", 202)]), { level: "ok", failed: [] });

// One endpoint down: the shared API forwards to every participant, so warn.
assert.deepEqual(indexNowOutcome([ok("a"), err("b", "fetch failed")]), {
  level: "warn",
  failed: ["b: fetch failed"],
});

// A non-2xx response counts as a failure, with its status named.
assert.deepEqual(indexNowOutcome([ok("a"), ok("b", 422)]), {
  level: "warn",
  failed: ["b: HTTP 422"],
});

// Nothing got through: that is a real failure.
assert.deepEqual(indexNowOutcome([err("a", "timeout"), ok("b", 500)]), {
  level: "fail",
  failed: ["a: timeout", "b: HTTP 500"],
});

console.log("indexNowOutcome: all assertions passed");
