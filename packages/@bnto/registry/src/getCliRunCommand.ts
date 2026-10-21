/**
 * The `bnto run` command for a recipe, with the argument its input node
 * takes. Mirrors `run_hint` in the CLI's `bnto info` (engine/crates/bnto/src/info.rs).
 */

import { getInputNode } from "./ioNodes";
import type { Recipe } from "./recipe";

const ARGUMENT_BY_MODE: Record<string, string> = {
  none: "",
  url: " <url>",
  text: " <text>",
};

/** File upload, the default input mode. */
const FILES_ARGUMENT = " <files...>";

export function getCliRunCommand(recipe: Recipe): string {
  const mode = getInputNode(recipe.definition)?.parameters?.mode;
  const argument =
    typeof mode === "string" ? (ARGUMENT_BY_MODE[mode] ?? FILES_ARGUMENT) : FILES_ARGUMENT;
  return `bnto run ${recipe.slug}${argument}`;
}
