/**
 * downloadDefinition — export the current recipe definition as a .bnto.json file.
 *
 * Calls exportAsRecipe() to get a validated recipe, serializes the
 * definition to JSON, and triggers a browser download. Returns true
 * if the download was triggered, false if validation failed.
 */

import { downloadBlob } from "@bnto/core";
import type { DefinitionClient } from "../editorTypes";

function downloadDefinition(definitionClient: DefinitionClient): boolean {
  const result = definitionClient.exportAsRecipe();
  if (!result.recipe) return false;
  const json = JSON.stringify(result.recipe.definition, null, 2);
  downloadBlob(new Blob([json], { type: "application/json" }), `${result.recipe.slug}.bnto.json`);
  return true;
}

export { downloadDefinition };
