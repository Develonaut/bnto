/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeSchema } from "../schemas/types";
import type { NodeParamFields } from "../schemas/types";

import { assertNodeSchema } from "./schemas/assert";
import { editFieldsNodeSchema } from "./schemas/editFields";
import { fileCollectNodeSchema } from "./schemas/fileCollect";
import { fileCopyNodeSchema } from "./schemas/fileCopy";
import { fileDecodeNodeSchema } from "./schemas/fileDecode";
import { fileDeleteNodeSchema } from "./schemas/fileDelete";
import { fileFilterNodeSchema } from "./schemas/fileFilter";
import { fileInspectNodeSchema } from "./schemas/fileInspect";
import { fileJoinNodeSchema } from "./schemas/fileJoin";
import { fileListNodeSchema } from "./schemas/fileList";
import { fileMoveNodeSchema } from "./schemas/fileMove";
import { fileRenameNodeSchema } from "./schemas/fileRename";
import { fileRenderNodeSchema } from "./schemas/fileRender";
import { fileSortNodeSchema } from "./schemas/fileSort";
import { groupNodeSchema } from "./schemas/group";
import { imageCompressNodeSchema } from "./schemas/imageCompress";
import { imageConvertNodeSchema } from "./schemas/imageConvert";
import { imageOverlayNodeSchema } from "./schemas/imageOverlay";
import { imagePadNodeSchema } from "./schemas/imagePad";
import { imageResizeNodeSchema } from "./schemas/imageResize";
import { imageStripExifNodeSchema } from "./schemas/imageStripExif";
import { inputNodeSchema } from "./schemas/input";
import { loopNodeSchema } from "./schemas/loop";
import { outputNodeSchema } from "./schemas/output";
import { parallelNodeSchema } from "./schemas/parallel";
import { reportNodeSchema } from "./schemas/report";
import { shellRunNodeSchema } from "./schemas/shellRun";
import { spreadsheetCleanNodeSchema } from "./schemas/spreadsheetClean";
import { spreadsheetConvertNodeSchema } from "./schemas/spreadsheetConvert";
import { spreadsheetExportNodeSchema } from "./schemas/spreadsheetExport";
import { spreadsheetLookupNodeSchema } from "./schemas/spreadsheetLookup";
import { spreadsheetMergeNodeSchema } from "./schemas/spreadsheetMerge";
import { spreadsheetReadNodeSchema } from "./schemas/spreadsheetRead";
import { spreadsheetRenameNodeSchema } from "./schemas/spreadsheetRename";
import { summarizeNodeSchema } from "./schemas/summarize";
import { transformNodeSchema } from "./schemas/transform";
import { vectorOptimizeNodeSchema } from "./schemas/vectorOptimize";
import { vectorRasterizeNodeSchema } from "./schemas/vectorRasterize";

/**
 * Schema entries for ALL node types — auto-assembled from the engine catalog.
 * The engine is the single source of truth. No hand-written overrides needed.
 */
export const ENGINE_NODE_SCHEMAS: Record<string, NodeSchema> = {
  assert: assertNodeSchema,
  "edit-fields": editFieldsNodeSchema,
  "file-collect": fileCollectNodeSchema,
  "file-copy": fileCopyNodeSchema,
  "file-decode": fileDecodeNodeSchema,
  "file-delete": fileDeleteNodeSchema,
  "file-filter": fileFilterNodeSchema,
  "file-inspect": fileInspectNodeSchema,
  "file-join": fileJoinNodeSchema,
  "file-list": fileListNodeSchema,
  "file-move": fileMoveNodeSchema,
  "file-rename": fileRenameNodeSchema,
  "file-render": fileRenderNodeSchema,
  "file-sort": fileSortNodeSchema,
  group: groupNodeSchema,
  "image-compress": imageCompressNodeSchema,
  "image-convert": imageConvertNodeSchema,
  "image-overlay": imageOverlayNodeSchema,
  "image-pad": imagePadNodeSchema,
  "image-resize": imageResizeNodeSchema,
  "image-strip-exif": imageStripExifNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  report: reportNodeSchema,
  "shell-run": shellRunNodeSchema,
  "spreadsheet-clean": spreadsheetCleanNodeSchema,
  "spreadsheet-convert": spreadsheetConvertNodeSchema,
  "spreadsheet-export": spreadsheetExportNodeSchema,
  "spreadsheet-lookup": spreadsheetLookupNodeSchema,
  "spreadsheet-merge": spreadsheetMergeNodeSchema,
  "spreadsheet-read": spreadsheetReadNodeSchema,
  "spreadsheet-rename": spreadsheetRenameNodeSchema,
  summarize: summarizeNodeSchema,
  transform: transformNodeSchema,
  "vector-optimize": vectorOptimizeNodeSchema,
  "vector-rasterize": vectorRasterizeNodeSchema,
};

/**
 * UI field presentation metadata for all node types.
 * Generated from the engine catalog's nodeTypes[].params presentation fields.
 */
export const ENGINE_NODE_PARAM_FIELDS: Record<string, NodeParamFields> = {
  assert: {
    op: {
      options: [
        { value: "equals", label: "Equals" },
        { value: "not_equals", label: "Does not equal" },
        { value: "after", label: "Is after (time)" },
        { value: "before", label: "Is before (time)" },
      ],
    },
  },
  "edit-fields": {
    values: { control: "keyValue" },
    keepOnlySet: { control: "switch" },
  },
  "file-collect": {},
  "file-copy": {
    conflict: {
      options: [
        { value: "skip", label: "Skip" },
        { value: "overwrite", label: "Overwrite" },
        { value: "rename", label: "Rename (add suffix)" },
        { value: "newer", label: "Newer only (keep time)" },
      ],
    },
    tolerance_ms: { suffix: "ms", visibleWhen: { param: "conflict", equals: "newer" } },
  },
  "file-decode": {
    encoding: { options: [{ value: "base64", label: "Base64" }] },
  },
  "file-delete": {},
  "file-filter": {
    pattern_mode: {
      options: [
        { value: "glob", label: "Glob (*, ?)" },
        { value: "regex", label: "Regex" },
      ],
    },
    match: {
      options: [
        { value: "name", label: "File name" },
        { value: "path", label: "Relative path" },
      ],
    },
    min_size: { suffix: "bytes" },
    max_size: { suffix: "bytes" },
  },
  "file-inspect": {},
  "file-join": {},
  "file-list": {},
  "file-move": {},
  "file-rename": {
    sanitize: {
      options: [
        { value: "slugify", label: "Slugify (web-safe)" },
        { value: "strip", label: "Strip non-ASCII" },
        { value: "normalize", label: "Normalize Unicode" },
      ],
    },
    case: {
      options: [
        { value: "lower", label: "lowercase" },
        { value: "upper", label: "UPPERCASE" },
        { value: "title", label: "Title Case" },
      ],
    },
  },
  "file-render": {
    header: { control: "textarea" },
    row: { control: "textarea" },
    footer: { control: "textarea" },
    escape: {
      options: [
        { value: "none", label: "None" },
        { value: "markdown", label: "Markdown" },
      ],
    },
  },
  "file-sort": {
    case: {
      options: [
        { value: "sensitive", label: "Case-sensitive" },
        { value: "insensitive", label: "Ignore case" },
      ],
    },
  },
  group: {
    mode: {
      control: "select",
      options: [
        { value: "sequential", label: "Sequential" },
        { value: "parallel", label: "Parallel" },
      ],
    },
  },
  "image-compress": {
    quality: {
      suffix: "%",
      control: "slider",
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
  },
  "image-convert": {
    format: {
      options: [
        { value: "jpeg", label: "JPEG" },
        { value: "png", label: "PNG" },
        { value: "webp", label: "WebP" },
      ],
    },
    quality: {
      suffix: "%",
      control: "slider",
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
  },
  "image-overlay": {
    overlay: { accept: ["image/png", "image/jpeg", "image/webp"] },
    position: {
      control: "watermarkPreview",
      options: [
        { value: "top-left", label: "Top Left" },
        { value: "top-center", label: "Top Center" },
        { value: "top-right", label: "Top Right" },
        { value: "middle-left", label: "Middle Left" },
        { value: "center", label: "Center" },
        { value: "middle-right", label: "Middle Right" },
        { value: "bottom-left", label: "Bottom Left" },
        { value: "bottom-center", label: "Bottom Center" },
        { value: "bottom-right", label: "Bottom Right" },
      ],
    },
    size: { suffix: "%" },
    opacity: { suffix: "%" },
    offsetX: { suffix: "px", group: "offset" },
    offsetY: { suffix: "px", group: "offset" },
    quality: {
      suffix: "%",
      control: "slider",
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
  },
  "image-pad": {
    width: { suffix: "px", group: "canvas" },
    height: { suffix: "px", group: "canvas" },
    quality: {
      suffix: "%",
      control: "slider",
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
  },
  "image-resize": {
    width: { suffix: "px", group: "dimensions" },
    height: { suffix: "px", group: "dimensions" },
    maintainAspect: { group: "dimensions" },
    quality: {
      suffix: "%",
      control: "slider",
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
  },
  "image-strip-exif": {
    quality: {
      suffix: "%",
      control: "slider",
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
  },
  input: {
    mode: {
      control: "select",
      options: [
        { value: "file-upload", label: "File Upload" },
        { value: "text", label: "Text" },
        { value: "url", label: "URL" },
        { value: "none", label: "No Input" },
      ],
    },
    accept: { control: "tagPicker", visibleWhen: { param: "mode", equals: "file-upload" } },
    extensions: {
      control: "tagPicker",
      visibleWhen: { param: "mode", equals: "file-upload" },
      options: [
        { value: ".jpg", label: ".jpg" },
        { value: ".jpeg", label: ".jpeg" },
        { value: ".png", label: ".png" },
        { value: ".webp", label: ".webp" },
        { value: ".gif", label: ".gif" },
        { value: ".svg", label: ".svg" },
        { value: ".csv", label: ".csv" },
        { value: ".json", label: ".json" },
        { value: ".txt", label: ".txt" },
        { value: ".pdf", label: ".pdf" },
        { value: ".xml", label: ".xml" },
        { value: ".zip", label: ".zip" },
        { value: ".mp3", label: ".mp3" },
        { value: ".mp4", label: ".mp4" },
        { value: ".wav", label: ".wav" },
      ],
    },
    label: { visibleWhen: { param: "mode", equals: "file-upload" } },
    multiple: { control: "switch", visibleWhen: { param: "mode", equals: "file-upload" } },
    maxFileSize: { visibleWhen: { param: "mode", equals: "file-upload" } },
    maxFiles: { visibleWhen: { param: "mode", equals: "file-upload" } },
    placeholder: {
      visibleWhen: [
        { param: "mode", equals: "text" },
        { param: "mode", equals: "url" },
      ],
    },
  },
  loop: {
    mode: {
      control: "select",
      options: [
        { value: "forEach", label: "For Each" },
        { value: "times", label: "N Times" },
        { value: "while", label: "While" },
      ],
    },
    items: { visibleWhen: { param: "mode", equals: "forEach" } },
    count: {
      visibleWhen: { param: "mode", equals: "times" },
      requiredWhen: { param: "mode", equals: "times" },
    },
    condition: {
      visibleWhen: { param: "mode", equals: "while" },
      requiredWhen: { param: "mode", equals: "while" },
    },
  },
  output: {
    mode: {
      control: "select",
      options: [
        { value: "write", label: "Write" },
        { value: "overwrite", label: "Overwrite" },
        { value: "message", label: "Message" },
        { value: "none", label: "None" },
      ],
    },
    directory: { visibleWhen: { param: "mode", equals: "write" } },
    filename: { visibleWhen: { param: "mode", equals: "write" } },
    zip: { control: "switch", visibleWhen: { param: "mode", equals: "write" } },
    autoDownload: { control: "switch", visibleWhen: { param: "mode", equals: "write" } },
  },
  parallel: {
    errorStrategy: {
      control: "select",
      options: [
        { value: "failFast", label: "Fail Fast" },
        { value: "collectAll", label: "Collect All" },
      ],
    },
  },
  report: {},
  "shell-run": {
    script: { control: "textarea" },
    args: { control: "tagPicker" },
    timeout: { suffix: "seconds" },
    maxOutputSize: { suffix: "MB" },
  },
  "spreadsheet-clean": {},
  "spreadsheet-convert": {
    delimiter: {
      options: [
        { value: "comma", label: "Comma" },
        { value: "semicolon", label: "Semicolon" },
        { value: "tab", label: "Tab" },
        { value: "pipe", label: "Pipe" },
      ],
    },
  },
  "spreadsheet-export": {},
  "spreadsheet-lookup": {
    match_on: {
      options: [
        { value: "path", label: "Path (relative folders included)" },
        { value: "filename", label: "Filename only" },
      ],
    },
    unmatched: {
      options: [
        { value: "keep", label: "Keep and flag" },
        { value: "drop", label: "Drop" },
        { value: "fail", label: "Fail the run" },
      ],
    },
  },
  "spreadsheet-merge": {
    headerHandling: {
      options: [
        { value: "first-file", label: "First File" },
        { value: "union", label: "Union" },
      ],
    },
  },
  "spreadsheet-read": {
    format: {
      options: [
        { value: "auto", label: "Auto" },
        { value: "csv", label: "CSV" },
        { value: "json", label: "JSON array" },
      ],
    },
    delimiter: {
      options: [
        { value: "comma", label: "Comma" },
        { value: "semicolon", label: "Semicolon" },
        { value: "tab", label: "Tab" },
        { value: "pipe", label: "Pipe" },
      ],
    },
  },
  "spreadsheet-rename": {
    columns: { control: "keyValue" },
  },
  summarize: {},
  transform: {
    expression: { control: "textarea" },
    mappings: { control: "keyValue" },
  },
  "vector-optimize": {},
  "vector-rasterize": {
    format: {
      options: [
        { value: "png", label: "PNG" },
        { value: "jpeg", label: "JPEG" },
        { value: "webp", label: "WebP" },
      ],
    },
  },
};
