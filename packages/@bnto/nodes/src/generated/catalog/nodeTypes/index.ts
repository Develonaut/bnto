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
import { fileDecodeNodeType } from "./fileDecode";
import { fileDeleteNodeType } from "./fileDelete";
import { fileFilterNodeType } from "./fileFilter";
import { fileJoinNodeType } from "./fileJoin";
import { fileListNodeType } from "./fileList";
import { fileMetadataNodeType } from "./fileMetadata";
import { fileMoveNodeType } from "./fileMove";
import { fileRenameNodeType } from "./fileRename";
import { fileRenderNodeType } from "./fileRender";
import { fileSortNodeType } from "./fileSort";
import { groupNodeType } from "./group";
import { httpRequestNodeType } from "./httpRequest";
import { imageCompressNodeType } from "./imageCompress";
import { imageConvertNodeType } from "./imageConvert";
import { imageOverlayNodeType } from "./imageOverlay";
import { imagePadNodeType } from "./imagePad";
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
import { summarizeNodeType } from "./summarize";
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
  fileDecode: "file-decode",
  fileDelete: "file-delete",
  fileFilter: "file-filter",
  fileJoin: "file-join",
  fileList: "file-list",
  fileMetadata: "file-metadata",
  fileMove: "file-move",
  fileRename: "file-rename",
  fileRender: "file-render",
  fileSort: "file-sort",
  group: "group",
  httpRequest: "http-request",
  imageCompress: "image-compress",
  imageConvert: "image-convert",
  imageOverlay: "image-overlay",
  imagePad: "image-pad",
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
  summarize: "summarize",
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
  "file-decode": fileDecodeNodeType,
  "file-delete": fileDeleteNodeType,
  "file-filter": fileFilterNodeType,
  "file-join": fileJoinNodeType,
  "file-list": fileListNodeType,
  "file-metadata": fileMetadataNodeType,
  "file-move": fileMoveNodeType,
  "file-rename": fileRenameNodeType,
  "file-render": fileRenderNodeType,
  "file-sort": fileSortNodeType,
  group: groupNodeType,
  "http-request": httpRequestNodeType,
  "image-compress": imageCompressNodeType,
  "image-convert": imageConvertNodeType,
  "image-overlay": imageOverlayNodeType,
  "image-pad": imagePadNodeType,
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
  summarize: summarizeNodeType,
  transform: transformNodeType,
  "vector-optimize": vectorOptimizeNodeType,
  "vector-rasterize": vectorRasterizeNodeType,
} as const satisfies Record<NodeTypeName, NodeTypeInfo>;
