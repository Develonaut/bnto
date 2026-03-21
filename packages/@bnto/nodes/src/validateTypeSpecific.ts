/**
 * Type-specific parameter validators.
 *
 * Each function validates the `parameters` of a specific node type.
 * Returns an array of ValidationError (never throws).
 */

import type { Definition } from "./definition";
import { LOOP_MODES } from "./schemas/loop";
import type { ValidationError } from "./validationError";

/** Valid loop modes — derived from the schema's canonical array. */
const VALID_LOOP_MODES = new Set<string>(LOOP_MODES);

function err(nodeId: string, field: string, message: string): ValidationError {
  return { nodeId, field, message };
}

function getStringParam(def: Definition, key: string): string | undefined {
  const val = def.parameters[key];
  if (typeof val === "string" && val !== "") return val;
  return undefined;
}

/**
 * Mode-specific required parameters: mode -> [paramName, ...].
 *
 * Note: `forEach` has NO required params. The Rust engine iterates over the
 * incoming file batch directly — it doesn't read from an `items` parameter.
 */
const LOOP_MODE_REQUIRED_PARAMS: Record<string, string[]> = {
  forEach: [],
  times: ["count"],
  while: ["condition"],
};

/** Validates loop node: mode required, mode-specific params required. */
function validateLoop(def: Definition): ValidationError[] {
  const mode = getStringParam(def, "mode");
  if (!mode) {
    return [err(def.id, "mode", `loop node '${def.id}' missing required parameter 'mode'`)];
  }
  if (!VALID_LOOP_MODES.has(mode)) {
    return [
      err(
        def.id,
        "mode",
        `loop node '${def.id}' has invalid mode '${mode}' (must be forEach, times, or while)`,
      ),
    ];
  }

  const requiredParams = LOOP_MODE_REQUIRED_PARAMS[mode] ?? [];
  return requiredParams
    .filter((param) => def.parameters[param] == null)
    .map((param) =>
      err(
        def.id,
        param,
        `loop node '${def.id}' with mode '${mode}' missing required parameter '${param}'`,
      ),
    );
}

/** Validates edit-fields node: values parameter required. */
function validateEditFields(def: Definition): ValidationError[] {
  if (def.parameters["values"] == null) {
    return [
      err(def.id, "values", `edit-fields node '${def.id}' missing required parameter 'values'`),
    ];
  }
  return [];
}

/**
 * Dispatch map from node type name to its validator function.
 *
 * Per-operation node types (image-compress, image-resize, image-convert,
 * spreadsheet-clean, spreadsheet-rename, file-rename) have no type-specific
 * validation — their per-operation Zod schemas handle field-level constraints.
 */
export const TYPE_VALIDATORS: Record<string, ((def: Definition) => ValidationError[]) | undefined> =
  {
    loop: validateLoop,
    "edit-fields": validateEditFields,
    // These types have no type-specific validation
    group: undefined,
    parallel: undefined,
    "image-compress": undefined,
    "image-resize": undefined,
    "image-convert": undefined,
    "spreadsheet-clean": undefined,
    "spreadsheet-rename": undefined,
    "file-rename": undefined,
    transform: undefined,
    "http-request": undefined,
    "shell-command": undefined,
  };
