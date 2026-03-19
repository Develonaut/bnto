/**
 * Definition client — composes definitionService with recipe export.
 *
 * Adds exportAsRecipe() which calls exportAsDefinition() then
 * definitionToRecipe() for the full export pipeline.
 */

import { definitionToRecipe, validateDefinition } from "@bnto/core";
import type { NodeRecipeMetadata } from "@bnto/core";
import type { DefinitionService, DefinitionClient, ExportResult } from "../editorTypes";

function createDefinitionClient(definitionService: DefinitionService): DefinitionClient {
  return {
    ...definitionService,

    exportAsRecipe(metadata?: NodeRecipeMetadata): ExportResult {
      const definition = definitionService.exportAsDefinition();
      const errors = validateDefinition(definition);
      if (errors.length > 0) return { recipe: null, errors };
      const recipe = definitionToRecipe(definition, metadata);
      return { recipe, errors: [] };
    },
  };
}

export { createDefinitionClient };
