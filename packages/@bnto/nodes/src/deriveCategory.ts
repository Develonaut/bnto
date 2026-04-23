import type { Definition } from "./definition";
import { NODE_TYPE_INFO } from "./generated/catalog";
import type { NodeTypeName } from "./generated/catalog";
import { isIoNodeType } from "./isIoNodeType";
import { isContainerNodeType } from "./isContainerNodeType";

/**
 * Derives the category of a recipe definition by examining its processing nodes.
 *
 * Recursively walks the definition tree to find the first non-I/O, non-container
 * node and returns its category from NODE_TYPE_INFO.
 *
 * @param definition - The recipe definition to analyze
 * @returns The category string (e.g., "image", "file", "spreadsheet"), or "custom"
 *          if no processing nodes are found
 *
 * @example
 * ```typescript
 * const category = deriveCategory(compressImagesDefinition);
 * // => "image" (from the nested image node inside the loop)
 * ```
 */
export function deriveCategory(definition: Definition): string {
  // Explicit category in metadata takes precedence (e.g., shell-command recipes
  // where the node type is generic but the recipe purpose is specific)
  const explicit = definition.metadata?.category;
  if (typeof explicit === "string" && explicit.length > 0) {
    return explicit;
  }

  const category = findFirstProcessingCategory(definition);
  return category ?? "custom";
}

/**
 * Recursively searches for the first processing node's category.
 *
 * @param node - The current node to examine
 * @returns The category string if a processing node is found, null otherwise
 */
function findFirstProcessingCategory(node: Definition): string | null {
  const nodeType = node.type as NodeTypeName;

  // Skip I/O nodes (input, output) and container nodes (loop, group)
  if (!isIoNodeType(nodeType) && !isContainerNodeType(nodeType)) {
    const typeInfo = NODE_TYPE_INFO[nodeType];
    if (typeInfo) {
      return typeInfo.category;
    }
  }

  // Recurse into child nodes if this is a container
  if (node.nodes && node.nodes.length > 0) {
    for (const child of node.nodes) {
      const category = findFirstProcessingCategory(child);
      if (category) {
        return category;
      }
    }
  }

  return null;
}
