/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeName, NodeTypeInfo } from "../types";

import { assertNodeType } from "./assert";
import { editFieldsNodeType } from "./editFields";
import { fileCollectNodeType } from "./fileCollect";
import { fileCopyNodeType } from "./fileCopy";
import { fileFilterNodeType } from "./fileFilter";
import { fileListNodeType } from "./fileList";
import { fileMetadataNodeType } from "./fileMetadata";
import { fileMoveNodeType } from "./fileMove";
import { fileRenameNodeType } from "./fileRename";
import { fileRenderNodeType } from "./fileRender";
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
import { reportNodeType } from "./report";
import { shellCommandNodeType } from "./shellCommand";
import { spreadsheetCleanNodeType } from "./spreadsheetClean";
import { spreadsheetConvertNodeType } from "./spreadsheetConvert";
import { spreadsheetExportNodeType } from "./spreadsheetExport";
import { spreadsheetLookupNodeType } from "./spreadsheetLookup";
import { spreadsheetMergeNodeType } from "./spreadsheetMerge";
import { spreadsheetReadNodeType } from "./spreadsheetRead";
import { spreadsheetRenameNodeType } from "./spreadsheetRename";
import { transformNodeType } from "./transform";
import { vectorOptimizeNodeType } from "./vectorOptimize";
import { vectorRasterizeNodeType } from "./vectorRasterize";

/**
 * All registered node type names.
 * Maps camelCase keys to kebab-case type name strings.
 */
export const NODE_TYPES = {
  assert: "assert",
  editFields: "edit-fields",
  fileCollect: "file-collect",
  fileCopy: "file-copy",
  fileFilter: "file-filter",
  fileList: "file-list",
  fileMetadata: "file-metadata",
  fileMove: "file-move",
  fileRename: "file-rename",
  fileRender: "file-render",
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
  report: "report",
  shellCommand: "shell-command",
  spreadsheetClean: "spreadsheet-clean",
  spreadsheetConvert: "spreadsheet-convert",
  spreadsheetExport: "spreadsheet-export",
  spreadsheetLookup: "spreadsheet-lookup",
  spreadsheetMerge: "spreadsheet-merge",
  spreadsheetRead: "spreadsheet-read",
  spreadsheetRename: "spreadsheet-rename",
  transform: "transform",
  vectorOptimize: "vector-optimize",
  vectorRasterize: "vector-rasterize",
} as const;

/** All node type names as a readonly array. */
export const NODE_TYPE_NAMES: readonly NodeTypeName[] = Object.values(NODE_TYPES) as NodeTypeName[];

/**
 * Metadata for all registered node types.
 * Maps node type name → info.
 */
export const NODE_TYPE_INFO: Record<NodeTypeName, NodeTypeInfo> = {
  assert: assertNodeType,
  "edit-fields": editFieldsNodeType,
  "file-collect": fileCollectNodeType,
  "file-copy": fileCopyNodeType,
  "file-filter": fileFilterNodeType,
  "file-list": fileListNodeType,
  "file-metadata": fileMetadataNodeType,
  "file-move": fileMoveNodeType,
  "file-rename": fileRenameNodeType,
  "file-render": fileRenderNodeType,
  group: groupNodeType,
  "http-request": httpRequestNodeType,
  "image-compress": imageCompressNodeType,
  "image-convert": imageConvertNodeType,
  "image-overlay": imageOverlayNodeType,
  "image-resize": imageResizeNodeType,
  "image-strip-exif": imageStripExifNodeType,
  input: inputNodeType,
  loop: loopNodeType,
  output: outputNodeType,
  parallel: parallelNodeType,
  report: reportNodeType,
  "shell-command": shellCommandNodeType,
  "spreadsheet-clean": spreadsheetCleanNodeType,
  "spreadsheet-convert": spreadsheetConvertNodeType,
  "spreadsheet-export": spreadsheetExportNodeType,
  "spreadsheet-lookup": spreadsheetLookupNodeType,
  "spreadsheet-merge": spreadsheetMergeNodeType,
  "spreadsheet-read": spreadsheetReadNodeType,
  "spreadsheet-rename": spreadsheetRenameNodeType,
  transform: transformNodeType,
  "vector-optimize": vectorOptimizeNodeType,
  "vector-rasterize": vectorRasterizeNodeType,
} as const satisfies Record<NodeTypeName, NodeTypeInfo>;
