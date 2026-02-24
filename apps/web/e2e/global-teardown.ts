import { execSync } from "child_process";

/**
 * Playwright global teardown for integration tests.
 * Purges dev R2 transit bucket after the suite finishes.
 *
 * Best-effort — failure logs a warning but doesn't fail the suite.
 * R2 lifecycle rules are the final safety net.
 */
export default function globalTeardown() {
  try {
    execSync("task r2:cleanup-dev", {
      cwd: "../..",
      stdio: "pipe",
      timeout: 30_000,
    });
    console.log("[teardown] R2 dev bucket purged");
  } catch (e) {
    console.warn(
      `[teardown] R2 cleanup failed (non-fatal): ${e instanceof Error ? e.message : String(e)}`,
    );
  }
}
