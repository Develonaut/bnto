/**
 * Render README.md's generated parts from the registry: the recipe count and
 * the recipe table, prettier-formatted the way the checked-in file is, so a
 * test can tell when the README has gone stale.
 */

import { format, resolveConfig } from "prettier";
import { getCliRunCommand } from "../src/getCliRunCommand";
import { isRecipeBrowserCapable } from "../src/isRecipeBrowserCapable";
import type { Recipe } from "../src/recipe";

interface Markers {
  begin: string;
  end: string;
}

const TABLE: Markers = {
  begin: "<!-- BEGIN AUTO-GENERATED RECIPES TABLE -->",
  end: "<!-- END AUTO-GENERATED RECIPES TABLE -->",
};

const COUNT: Markers = { begin: "<!-- recipe-count -->", end: "<!-- /recipe-count -->" };

function replaceBetween(text: string, markers: Markers, body: string): string {
  const begin = text.indexOf(markers.begin);
  const end = text.indexOf(markers.end);
  if (begin === -1 || end === -1) {
    throw new Error(`README.md is missing ${markers.begin} … ${markers.end}`);
  }
  return text.slice(0, begin + markers.begin.length) + body + text.slice(end);
}

/** Browser recipes link to bnto.io; CLI-only ones show the command to run. */
function tryIt(r: Recipe): string {
  if (isRecipeBrowserCapable(r)) return `[bnto.io/${r.slug}](https://bnto.io/${r.slug})`;
  return `\`${getCliRunCommand(r)}\``;
}

function recipeTable(recipes: readonly Recipe[]): string {
  const rows = recipes.map((r) => `| ${r.name} | ${r.description} | ${tryIt(r)} |`);
  return ["| Recipe | What it does | Try it |", "| --- | --- | --- |", ...rows].join("\n");
}

/** `readme` with its generated parts rebuilt from `recipes`; `filepath` picks the prettier config. */
export async function renderReadme(
  readme: string,
  recipes: readonly Recipe[],
  filepath: string,
): Promise<string> {
  const withCount = replaceBetween(readme, COUNT, String(recipes.length));
  const withTable = replaceBetween(withCount, TABLE, `\n${recipeTable(recipes)}\n`);
  const options = await resolveConfig(filepath);
  return format(withTable, { ...options, filepath });
}
