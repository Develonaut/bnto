/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeSchema } from "../schemas/types";
import type { NodeParamFields } from "../schemas/types";

import { editFieldsNodeSchema } from "./schemas/editFields";
import { fileRenameNodeSchema } from "./schemas/fileRename";
import { groupNodeSchema } from "./schemas/group";
import { imageCompressNodeSchema } from "./schemas/imageCompress";
import { imageConvertNodeSchema } from "./schemas/imageConvert";
import { imageOverlayNodeSchema } from "./schemas/imageOverlay";
import { imageResizeNodeSchema } from "./schemas/imageResize";
import { imageStripExifNodeSchema } from "./schemas/imageStripExif";
import { inputNodeSchema } from "./schemas/input";
import { loopNodeSchema } from "./schemas/loop";
import { outputNodeSchema } from "./schemas/output";
import { parallelNodeSchema } from "./schemas/parallel";
import { spreadsheetCleanNodeSchema } from "./schemas/spreadsheetClean";
import { spreadsheetConvertNodeSchema } from "./schemas/spreadsheetConvert";
import { spreadsheetMergeNodeSchema } from "./schemas/spreadsheetMerge";
import { spreadsheetRenameNodeSchema } from "./schemas/spreadsheetRename";
import { transformNodeSchema } from "./schemas/transform";
import { vectorOptimizeNodeSchema } from "./schemas/vectorOptimize";
import { vectorRasterizeNodeSchema } from "./schemas/vectorRasterize";
import { videoDownloadNodeSchema } from "./schemas/videoDownload";

/**
 * Schema entries for ALL node types — auto-assembled from the engine catalog.
 * The engine is the single source of truth. No hand-written overrides needed.
 */
export const ENGINE_NODE_SCHEMAS: Record<string, NodeSchema> = {
  "edit-fields": editFieldsNodeSchema,
  "file-rename": fileRenameNodeSchema,
  group: groupNodeSchema,
  "image-compress": imageCompressNodeSchema,
  "image-convert": imageConvertNodeSchema,
  "image-overlay": imageOverlayNodeSchema,
  "image-resize": imageResizeNodeSchema,
  "image-strip-exif": imageStripExifNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  "spreadsheet-clean": spreadsheetCleanNodeSchema,
  "spreadsheet-convert": spreadsheetConvertNodeSchema,
  "spreadsheet-merge": spreadsheetMergeNodeSchema,
  "spreadsheet-rename": spreadsheetRenameNodeSchema,
  transform: transformNodeSchema,
  "vector-optimize": vectorOptimizeNodeSchema,
  "vector-rasterize": vectorRasterizeNodeSchema,
  "video-download": videoDownloadNodeSchema,
};

/**
 * UI field presentation metadata for all node types.
 * Generated from the engine catalog's nodeTypes[].params presentation fields.
 */
export const ENGINE_NODE_PARAM_FIELDS: Record<string, NodeParamFields> = {
  "edit-fields": {
    values: { control: "keyValue" },
    keepOnlySet: { control: "switch" },
  },
  "file-rename": {
    case: {
      options: [
        { value: "lower", label: "lowercase" },
        { value: "upper", label: "UPPERCASE" },
        { value: "title", label: "Title Case" },
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
        { value: "download", label: "Download" },
        { value: "display", label: "Display" },
        { value: "preview", label: "Preview" },
      ],
    },
    filename: { visibleWhen: { param: "mode", equals: "download" } },
    zip: { control: "switch", visibleWhen: { param: "mode", equals: "download" } },
    autoDownload: { control: "switch", visibleWhen: { param: "mode", equals: "download" } },
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
  "spreadsheet-merge": {
    headerHandling: {
      options: [
        { value: "first-file", label: "First File" },
        { value: "union", label: "Union" },
      ],
    },
  },
  "spreadsheet-rename": {
    columns: { control: "keyValue" },
  },
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
  "video-download": {
    format: {
      options: [
        { value: "mp4", label: "MP4" },
        { value: "webm", label: "WebM" },
        { value: "mkv", label: "MKV" },
        { value: "mp3", label: "MP3" },
        { value: "m4a", label: "M4A" },
        { value: "wav", label: "WAV" },
        { value: "flac", label: "FLAC" },
      ],
    },
    quality: {
      options: [
        { value: "best", label: "Best Available" },
        { value: "1080", label: "1080p" },
        { value: "720", label: "720p" },
        { value: "480", label: "480p" },
        { value: "360", label: "360p" },
      ],
    },
  },
};
