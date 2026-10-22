/**
 * Codegen: update README.md's recipe count and recipe table from the registry.
 * `scripts/renderReadme.test.ts` fails when the checked-in README is stale.
 *
 * Run via: `task readme:generate`
 */

import { readFileSync, writeFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { RECIPES } from "../src/recipesCatalog";
import { renderReadme } from "./renderReadme";

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const README_PATH = resolve(SCRIPT_DIR, "../../../../README.md");

async function main() {
  const readme = readFileSync(README_PATH, "utf-8");
  const updated = await renderReadme(readme, RECIPES, README_PATH);
  if (updated === readme) {
    console.log("README recipe count and table are already up to date.");
    return;
  }
  writeFileSync(README_PATH, updated, "utf-8");
  console.log(`Updated README recipe count and table (${RECIPES.length} recipes).`);
}

void main();
