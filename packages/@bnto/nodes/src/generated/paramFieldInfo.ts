/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeParamFieldInfo } from "../schemas/types";

/** Pre-computed field info for all node parameters — replaces runtime Zod introspection. */
export const NODE_PARAM_FIELD_INFO: Record<string, Record<string, NodeParamFieldInfo>> = {
  "edit-fields": {
    values: { type: "record", control: "keyValue", required: true },
    keepOnlySet: { type: "boolean", control: "switch", required: false },
  },
  "file-collect": {
    pattern: { type: "string", control: "text", required: false },
    recursive: { type: "boolean", control: "switch", required: false },
    flatten: { type: "boolean", control: "switch", required: false },
  },
  "file-copy": {
    destination: { type: "string", control: "text", required: false },
    create_dirs: { type: "boolean", control: "switch", required: false },
    conflict: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["skip", "overwrite", "rename"] as const,
    },
  },
  "file-filter": {
    extensions: { type: "string", control: "text", required: false },
    name_pattern: { type: "string", control: "text", required: false },
    pattern_mode: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["glob", "regex"] as const,
    },
    min_size: { type: "number", control: "number", required: false, min: 0 },
    max_size: { type: "number", control: "number", required: false, min: 0 },
  },
  "file-metadata": {
    include_hash: { type: "boolean", control: "switch", required: false },
  },
  "file-rename": {
    sanitize: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["slugify", "strip", "normalize"] as const,
    },
    separator: { type: "string", control: "text", required: false },
    max_length: { type: "number", control: "number", required: false, min: 0 },
    find: { type: "string", control: "text", required: false },
    replace: { type: "string", control: "text", required: false },
    case: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["lower", "upper", "title"] as const,
    },
    prefix: { type: "string", control: "text", required: false },
    suffix: { type: "string", control: "text", required: false },
    pattern: { type: "string", control: "text", required: false },
    counter_start: { type: "number", control: "number", required: false, min: 0 },
    counter_pad: { type: "number", control: "slider", required: false, min: 0, max: 10 },
    extension: { type: "string", control: "text", required: false },
  },
  group: {
    mode: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["sequential", "parallel"] as const,
    },
  },
  "image-compress": {
    quality: { type: "number", control: "slider", required: false, min: 1, max: 100 },
  },
  "image-convert": {
    format: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["jpeg", "png", "webp"] as const,
    },
    quality: { type: "number", control: "slider", required: false, min: 1, max: 100 },
  },
  "image-overlay": {
    overlay: { type: "string", control: "file", required: true },
    position: {
      type: "enum",
      control: "watermarkPreview",
      required: false,
      enumValues: [
        "top-left",
        "top-center",
        "top-right",
        "middle-left",
        "center",
        "middle-right",
        "bottom-left",
        "bottom-center",
        "bottom-right",
      ] as const,
    },
    size: { type: "number", control: "slider", required: false, min: 1, max: 500 },
    opacity: { type: "number", control: "slider", required: false, min: 0, max: 100 },
    offsetX: { type: "number", control: "slider", required: false, min: -500, max: 500 },
    offsetY: { type: "number", control: "slider", required: false, min: -500, max: 500 },
    quality: { type: "number", control: "slider", required: false, min: 1, max: 100 },
  },
  "image-resize": {
    width: { type: "number", control: "number", required: false, min: 1 },
    height: { type: "number", control: "number", required: false, min: 1 },
    maintainAspect: { type: "boolean", control: "switch", required: false },
    quality: { type: "number", control: "slider", required: false, min: 1, max: 100 },
  },
  "image-strip-exif": {
    quality: { type: "number", control: "slider", required: false, min: 1, max: 100 },
  },
  input: {
    mode: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["file-upload", "text", "url"] as const,
    },
    accept: { type: "array", control: "tagPicker", required: false },
    extensions: { type: "array", control: "tagPicker", required: false },
    label: { type: "string", control: "text", required: false },
    multiple: { type: "boolean", control: "switch", required: false },
    maxFileSize: { type: "number", control: "number", required: false, min: 0 },
    maxFiles: { type: "number", control: "number", required: false, min: 0 },
    placeholder: { type: "string", control: "text", required: false },
  },
  loop: {
    mode: {
      type: "enum",
      control: "select",
      required: true,
      enumValues: ["forEach", "times", "while"] as const,
    },
    items: { type: "string", control: "text", required: false },
    count: { type: "number", control: "number", required: false, min: 0 },
    condition: { type: "string", control: "text", required: false },
    breakCondition: { type: "string", control: "text", required: false },
  },
  output: {
    mode: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["download", "display", "preview"] as const,
    },
    filename: { type: "string", control: "text", required: false },
    zip: { type: "boolean", control: "switch", required: false },
    label: { type: "string", control: "text", required: false },
    autoDownload: { type: "boolean", control: "switch", required: false },
  },
  parallel: {
    tasks: { type: "record", control: "keyValue", required: true },
    maxWorkers: { type: "number", control: "number", required: false, min: 1 },
    errorStrategy: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["failFast", "collectAll"] as const,
    },
  },
  "shell-command": {
    command: { type: "string", control: "text", required: false },
    args: { type: "array", control: "tagPicker", required: false },
    outputMode: { type: "string", control: "text", required: false },
    timeout: { type: "number", control: "number", required: false },
    maxOutputSize: { type: "number", control: "number", required: false },
    env: { type: "record", control: "keyValue", required: false },
  },
  "spreadsheet-clean": {
    trimWhitespace: { type: "boolean", control: "switch", required: false },
    removeEmptyRows: { type: "boolean", control: "switch", required: false },
    removeDuplicates: { type: "boolean", control: "switch", required: false },
  },
  "spreadsheet-convert": {
    delimiter: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["comma", "semicolon", "tab", "pipe"] as const,
    },
    pretty: { type: "boolean", control: "switch", required: false },
  },
  "spreadsheet-merge": {
    headerHandling: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["first-file", "union"] as const,
    },
    deduplicate: { type: "boolean", control: "switch", required: false },
  },
  "spreadsheet-read": {
    hasHeaders: { type: "boolean", control: "switch", required: false },
    delimiter: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["comma", "semicolon", "tab", "pipe"] as const,
    },
    maxRows: { type: "number", control: "slider", required: false, min: 1, max: 10000000 },
  },
  "spreadsheet-rename": {
    columns: { type: "record", control: "keyValue", required: false },
  },
  transform: {
    expression: { type: "string", control: "textarea", required: false },
    mappings: { type: "record", control: "keyValue", required: false },
  },
  "vector-optimize": {
    removeComments: { type: "boolean", control: "switch", required: false },
    removeMetadata: { type: "boolean", control: "switch", required: false },
    collapseGroups: { type: "boolean", control: "switch", required: false },
    minify: { type: "boolean", control: "switch", required: false },
    precision: { type: "number", control: "slider", required: false, min: 0, max: 8 },
  },
  "vector-rasterize": {
    format: {
      type: "enum",
      control: "select",
      required: false,
      enumValues: ["png", "jpeg", "webp"] as const,
    },
    quality: { type: "number", control: "slider", required: false, min: 1, max: 100 },
  },
};

/** Look up pre-computed field info for a specific node type and parameter. */
export function getParamFieldInfo(
  nodeType: string,
  paramName: string,
): NodeParamFieldInfo | undefined {
  return NODE_PARAM_FIELD_INFO[nodeType]?.[paramName];
}
