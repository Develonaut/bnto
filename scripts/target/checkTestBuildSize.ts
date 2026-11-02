/**
 * Fail when one build of the engine's tests outgrows its budget
 * (testBuildBudget.ts, ISS-120). It asks cargo for the units of
 * `cargo test --workspace` (a no-op after the tests ran, otherwise a build)
 * and adds up what those units hold in the target folder, so units left over
 * from other builds don't count. Binaries (`bnto`, `tonkotsu-demo`) and
 * bnto-wasm's library aren't counted: cargo names them without a unit hash.
 * Nor are object files: macOS keeps them beside the binaries for debug info,
 * and keeps earlier rebuilds' objects inside units still in use (ISS-121), so
 * counting them would grow with every rebuild. Debug info growth still shows
 * on Linux and Windows, which keep it in binaries and `.pdb` files.
 * Catches full debug info coming back or test binaries multiplying, each of
 * which links the whole engine. It then marks those units as used
 * (markUsed.ts), so the prune keeps the build `task check` makes most, even
 * where a read doesn't count as use.
 *   pnpm exec tsx scripts/target/checkTestBuildSize.ts [engine folder]
 * The engine folder defaults to `engine`; the target folder is the one cargo
 * builds into (targetDir.ts).
 */

import { execFileSync } from "node:child_process";
import { artifactKeys } from "./artifactKeys.ts";
import { markUsed } from "./markUsed.ts";
import { targetDir } from "./targetDir.ts";
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
  const target = targetDir(engine);
  const build = targetEntries(target).filter((entry) => keys.has(entry.key));
  const counted = build.filter((entry) => !entry.path.endsWith(".o"));
  if (keys.size > 0 && build.length === 0) {
    throw new Error(`cargo listed ${keys.size} units, and none of them is in ${target}`);
  }
  markUsed(build, new Date());
  return Math.round(counted.reduce((sum, entry) => sum + entry.bytes, 0) / 1e6);
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
