/**
 * The bnto release version, which every bnto crate shares and
 * `cargo install bnto` installs. Read from the engine workspace manifest at
 * build time (server only), so the site never shows a hand-written version.
 */

import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseReleaseVersion } from "./parseReleaseVersion";

/** From apps/web, where Next.js, Vitest and Playwright all run. */
const WORKSPACE_MANIFEST = "../../engine/Cargo.toml";

export function getBntoCliVersion(): string {
  return parseReleaseVersion(readFileSync(resolve(process.cwd(), WORKSPACE_MANIFEST), "utf-8"));
}
