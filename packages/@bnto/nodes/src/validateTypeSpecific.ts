/**
 * Type-specific parameter validators.
 *
 * Each function validates the `parameters` of a specific node type.
 * Returns an array of ValidationError (never throws).
 */

import type { Definition } from "./definition";
import { LOOP_MODES } from "./schemas/loop";
import { FILE_OPERATIONS } from "./schemas/fileSystem";
import type { ValidationError } from "./validationError";

/** Valid loop modes — derived from the schema's canonical array. */
const VALID_LOOP_MODES = new Set<string>(LOOP_MODES);

/** Valid file-system operations — derived from the engine catalog. */
const VALID_FILE_OPERATIONS = new Set<string>(FILE_OPERATIONS);

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
export function validateLoop(def: Definition): ValidationError[] {
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

/** Validates file-system node: operation required, must be valid. */
export function validateFileSystem(def: Definition): ValidationError[] {
  const errors: ValidationError[] = [];

  const operation = getStringParam(def, "operation");
  if (!operation) {
    errors.push(
      err(
        def.id,
        "operation",
        `file-system node '${def.id}' missing required parameter 'operation'`,
      ),
    );
  } else if (!VALID_FILE_OPERATIONS.has(operation)) {
    errors.push(
      err(
        def.id,
        "operation",
        `file-system node '${def.id}' has invalid operation '${operation}' (must be ${FILE_OPERATIONS.join(", ")})`,
      ),
    );
  }

  return errors;
}

/** Validates edit-fields node: values parameter required. */
export function validateEditFields(def: Definition): ValidationError[] {
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
 * Types not listed here (group, parallel, spreadsheet, image, transform,
 * http-request, shell-command) have no type-specific validation.
 */
export const TYPE_VALIDATORS: Record<string, ((def: Definition) => ValidationError[]) | undefined> =
  {
    "file-system": validateFileSystem,
    loop: validateLoop,
    "edit-fields": validateEditFields,
    // These types have no type-specific validation
    group: undefined,
    parallel: undefined,
    spreadsheet: undefined,
    image: undefined,
    transform: undefined,
    "http-request": undefined,
    "shell-command": undefined,
  };
