/**
 * Codegen: Generate the README recipe table from the registry.
 *
 * Reads all predefined recipes from `@bnto/registry` and generates
 * a Markdown table. Updates README.md in-place between marker comments.
 *
 * Run via: `task readme:generate`
 */

import { readFileSync, writeFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { RECIPES } from "../src/recipesCatalog";

const SCRIPT_DIR = dirname(import.meta.url.replace("file://", ""));
const README_PATH = resolve(SCRIPT_DIR, "../../../../README.md");

const BEGIN_MARKER = "<!-- BEGIN AUTO-GENERATED RECIPES TABLE -->";
const END_MARKER = "<!-- END AUTO-GENERATED RECIPES TABLE -->";

function generateTable(): string {
  const header = "| Recipe | What it does | Try it |";
  const separator = "| --- | --- | --- |";
  const rows = RECIPES.map(
    (r) => `| ${r.name} | ${r.description} | [bnto.io/${r.slug}](https://bnto.io/${r.slug}) |`,
  );

  return [BEGIN_MARKER, header, separator, ...rows, END_MARKER].join("\n");
}

const readme = readFileSync(README_PATH, "utf-8");
const beginIndex = readme.indexOf(BEGIN_MARKER);
const endIndex = readme.indexOf(END_MARKER);

if (beginIndex === -1 || endIndex === -1) {
  console.error(
    "Missing marker comments in README.md. Expected:\n" + `  ${BEGIN_MARKER}\n  ${END_MARKER}`,
  );
  process.exit(1);
}

const before = readme.slice(0, beginIndex);
const after = readme.slice(endIndex + END_MARKER.length);
const updated = before + generateTable() + after;

if (updated === readme) {
  console.log("README recipe table is already up to date.");
} else {
  writeFileSync(README_PATH, updated, "utf-8");
  console.log(`Updated README recipe table (${RECIPES.length} recipes).`);
}
