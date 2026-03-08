/**
 * Type-specific parameter validators.
 *
 * Each function validates the `parameters` of a specific node type.
 * Returns an array of ValidationError (never throws).
 *
 * NOTE: 6 validators are co-located here because each is short (4-18 lines),
 * they share private helpers (err, getStringParam), and they form a single
 * cohesive dispatch table. Extracting each to its own file would add overhead
 * without meaningful clarity gain. Revisit if any validator exceeds 30 lines.
 */

import type { Definition } from "./definition";
import { HTTP_METHODS } from "./schemas/httpRequest";
import { LOOP_MODES } from "./schemas/loop";
import { FILE_OPERATIONS } from "./schemas/fileSystem";
import type { ValidationError } from "./validationError";

/** Valid HTTP methods — derived from the schema's canonical array. */
const VALID_HTTP_METHODS = new Set<string>(HTTP_METHODS);

/** Valid loop modes — derived from the schema's canonical array. */
const VALID_LOOP_MODES = new Set<string>(LOOP_MODES);

/** Valid file-system operations — derived from the schema's canonical array. */
const VALID_FILE_OPERATIONS = new Set<string>(FILE_OPERATIONS);

function err(nodeId: string, field: string, message: string): ValidationError {
  return { nodeId, field, message };
}

function getStringParam(def: Definition, key: string): string | undefined {
  const val = def.parameters[key];
  if (typeof val === "string" && val !== "") return val;
  return undefined;
}

/** Validates the method parameter value is a known HTTP method. */
function validateMethod(def: Definition, method: string): ValidationError[] {
  if (VALID_HTTP_METHODS.has(method)) return [];
  return [
    err(
      def.id,
      "method",
      `http-request node '${def.id}' has invalid method '${method}' (must be GET, POST, PUT, PATCH, DELETE, HEAD, or OPTIONS)`,
    ),
  ];
}

/** Validates http-request node: url and method required, method must be valid. */
export function validateHttpRequest(def: Definition): ValidationError[] {
  const errors: ValidationError[] = [];
  if (!getStringParam(def, "url")) {
    errors.push(
      err(def.id, "url", `http-request node '${def.id}' missing required parameter 'url'`),
    );
  }
  const method = getStringParam(def, "method");
  if (!method) {
    errors.push(
      err(def.id, "method", `http-request node '${def.id}' missing required parameter 'method'`),
    );
  } else {
    errors.push(...validateMethod(def, method));
  }
  return errors;
}

/**
 * Mode-specific required parameters: mode -> [paramName, ...].
 *
 * Note: `forEach` has NO required params. The Rust engine iterates over the
 * incoming file batch directly — it doesn't read from an `items` parameter.
 * The Go engine used `items` with template expressions like
 * `{{index . "input" "files"}}`, but those are deprecated.
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
        `file-system node '${def.id}' has invalid operation '${operation}' (must be rename, read, write, copy, move, delete, mkdir, exists, or list)`,
      ),
    );
  }

  return errors;
}

/** Validates shell-command node: command required (non-empty). */
export function validateShellCommand(def: Definition): ValidationError[] {
  if (!getStringParam(def, "command")) {
    return [
      err(def.id, "command", `shell-command node '${def.id}' missing required parameter 'command'`),
    ];
  }
  return [];
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
 * Types not listed here (group, parallel, spreadsheet, image, transform)
 * have no type-specific validation.
 */
export const TYPE_VALIDATORS: Record<string, ((def: Definition) => ValidationError[]) | undefined> =
  {
    "http-request": validateHttpRequest,
    "file-system": validateFileSystem,
    "shell-command": validateShellCommand,
    loop: validateLoop,
    "edit-fields": validateEditFields,
    // These types have no type-specific validation
    group: undefined,
    parallel: undefined,
    spreadsheet: undefined,
    image: undefined,
    transform: undefined,
  };
