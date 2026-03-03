import { execSync } from "child_process";
import path from "path";

/**
 * Playwright global teardown — runs after all tests complete.
 *
 * Calls the Convex cleanup mutation to delete all @test.bnto.dev accounts
 * and cascaded data. Non-blocking: if cleanup fails, tests still pass.
 * Stale accounts are harmless; cleanup catches up on next run.
 */
export default function globalTeardown() {
  // Resolve backend dir from this file's location so it works in worktrees too.
  // This file:      apps/web/e2e/global-teardown.ts
  // Backend dir:    packages/@bnto/backend
  const backendDir = path.resolve(__dirname, "../../../packages/@bnto/backend");

  try {
    const result = execSync(
      "npx convex run _dev_cleanup:cleanTestAccounts",
      {
        cwd: backendDir,
        stdio: "pipe",
        timeout: 30_000,
      },
    );
    console.log(`[e2e teardown] ${result.toString().trim()}`);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    console.warn(
      `[e2e teardown] cleanup failed — test accounts may persist until next run: ${message}`,
    );
  }
}
