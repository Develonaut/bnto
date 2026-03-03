import { execSync } from "child_process";

/**
 * Playwright global teardown — runs after all tests complete.
 *
 * Calls the Convex cleanup mutation to delete all @test.bnto.dev accounts
 * and cascaded data. Non-blocking: if cleanup fails, tests still pass.
 * Stale accounts are harmless; cleanup catches up on next run.
 */
export default function globalTeardown() {
  try {
    execSync(
      "npx convex run _dev_cleanup:cleanTestAccounts",
      {
        cwd: "../../packages/@bnto/backend",
        stdio: "pipe",
        timeout: 30_000,
      },
    );
    console.log("[e2e teardown] test account cleanup complete");
  } catch {
    console.warn(
      "[e2e teardown] cleanup failed -- test accounts may persist until next run",
    );
  }
}
