/**
 * Fail when one build of the engine's tests outgrows its budget
 * (testBuildBudget.ts, ISS-120). It asks cargo for the units of
 * `cargo test --workspace` (a no-op after the tests ran, otherwise a build)
 * and adds up what those units hold in the target folder, so units left over
 * from other builds don't count. Catches full debug info coming back or test
 * binaries multiplying, each of which links the whole engine.
 *   pnpm exec tsx scripts/target/checkTestBuildSize.ts [engine folder]
 * The engine folder defaults to `engine`; the target folder is
 * `$CARGO_TARGET_DIR`, else `<engine>/target`.
 */

import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { artifactKeys } from "./artifactKeys.ts";
import { targetEntries } from "./targetEntries.ts";
import { TEST_BUILD_BUDGET_MB } from "./testBuildBudget.ts";

function buildOutput(engine: string): string {
  return execFileSync("cargo", ["test", "--workspace", "--no-run", "--message-format=json"], {
    cwd: engine,
    encoding: "utf8",
    maxBuffer: 256 * 1024 * 1024,
    stdio: ["ignore", "pipe", "inherit"],
  });
}

function buildMegabytes(engine: string): number {
  const keys = artifactKeys(buildOutput(engine));
  const target = process.env.CARGO_TARGET_DIR || join(engine, "target");
  const bytes = targetEntries(target)
    .filter((entry) => keys.has(entry.key))
    .reduce((sum, entry) => sum + entry.bytes, 0);
  return Math.round(bytes / 1e6);
}

function main() {
  const size = buildMegabytes(process.argv[2] ?? "engine");
  const budget = TEST_BUILD_BUDGET_MB[process.platform];
  const what = `One build of the engine's tests takes ${size} MB`;
  if (budget === undefined) {
    console.warn(
      `warning: ${what}; ${process.platform} has no budget in scripts/target/testBuildBudget.ts`,
    );
  } else if (size > budget) {
    console.error(
      `${what}, over its ${budget} MB budget on ${process.platform} (scripts/target/testBuildBudget.ts).`,
    );
    console.error(
      "Find what grew (a new test binary? more debug info?), or raise the budget saying why.",
    );
    process.exit(1);
  } else {
    console.log(`${what} (budget ${budget} MB on ${process.platform})`);
  }
}

main();
