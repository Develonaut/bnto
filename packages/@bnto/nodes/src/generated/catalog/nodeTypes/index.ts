/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeName, NodeTypeInfo } from "../types";

import { editFieldsNodeType } from "./editFields";
import { fileRenameNodeType } from "./fileRename";
import { groupNodeType } from "./group";
import { httpRequestNodeType } from "./httpRequest";
import { imageCompressNodeType } from "./imageCompress";
import { imageConvertNodeType } from "./imageConvert";
import { imageOverlayNodeType } from "./imageOverlay";
import { imageResizeNodeType } from "./imageResize";
import { imageStripExifNodeType } from "./imageStripExif";
import { inputNodeType } from "./input";
import { loopNodeType } from "./loop";
import { outputNodeType } from "./output";
import { parallelNodeType } from "./parallel";
import { shellCommandNodeType } from "./shellCommand";
import { spreadsheetCleanNodeType } from "./spreadsheetClean";
import { spreadsheetConvertNodeType } from "./spreadsheetConvert";
import { spreadsheetMergeNodeType } from "./spreadsheetMerge";
import { spreadsheetRenameNodeType } from "./spreadsheetRename";
import { transformNodeType } from "./transform";

/**
 * All registered node type names.
 * Maps camelCase keys to kebab-case type name strings.
 */
export const NODE_TYPES = {
  editFields: "edit-fields",
  fileRename: "file-rename",
  group: "group",
  httpRequest: "http-request",
  imageCompress: "image-compress",
  imageConvert: "image-convert",
  imageOverlay: "image-overlay",
  imageResize: "image-resize",
  imageStripExif: "image-strip-exif",
  input: "input",
  loop: "loop",
  output: "output",
  parallel: "parallel",
  shellCommand: "shell-command",
  spreadsheetClean: "spreadsheet-clean",
  spreadsheetConvert: "spreadsheet-convert",
  spreadsheetMerge: "spreadsheet-merge",
  spreadsheetRename: "spreadsheet-rename",
  transform: "transform",
} as const;

/** All node type names as a readonly array. */
export const NODE_TYPE_NAMES: readonly NodeTypeName[] = Object.values(NODE_TYPES) as NodeTypeName[];

/**
 * Metadata for all registered node types.
 * Maps node type name → info.
 */
export const NODE_TYPE_INFO: Record<NodeTypeName, NodeTypeInfo> = {
  "edit-fields": editFieldsNodeType,
  "file-rename": fileRenameNodeType,
  "group": groupNodeType,
  "http-request": httpRequestNodeType,
  "image-compress": imageCompressNodeType,
  "image-convert": imageConvertNodeType,
  "image-overlay": imageOverlayNodeType,
  "image-resize": imageResizeNodeType,
  "image-strip-exif": imageStripExifNodeType,
  "input": inputNodeType,
  "loop": loopNodeType,
  "output": outputNodeType,
  "parallel": parallelNodeType,
  "shell-command": shellCommandNodeType,
  "spreadsheet-clean": spreadsheetCleanNodeType,
  "spreadsheet-convert": spreadsheetConvertNodeType,
  "spreadsheet-merge": spreadsheetMergeNodeType,
  "spreadsheet-rename": spreadsheetRenameNodeType,
  "transform": transformNodeType,
} as const satisfies Record<NodeTypeName, NodeTypeInfo>;
