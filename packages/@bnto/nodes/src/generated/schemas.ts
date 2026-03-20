/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 *
 * Zod schemas and UI metadata for engine-backed node types.
 * Run `task nodes:generate` to regenerate after engine changes.
 *
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../schemas/types";

/** Zod schema for file-rename node parameters (auto-generated from engine). */
export const fileRenameParamsSchema = z.object({
  find: z.string().optional(),
  replace: z.string().optional(),
  case: z.enum(["lower", "upper", "title"] as const).optional(),
  prefix: z.string().optional(),
  suffix: z.string().optional(),
  pattern: z.string().optional(),
});

/** Inferred TypeScript type for file-rename node parameters. */
export type FileRenameParams = z.infer<typeof fileRenameParamsSchema>;

/** Full schema definition for the file-rename node type (auto-generated from engine). */
export const fileRenameNodeSchema: NodeSchema = {
  nodeType: "file-rename",
  schemaVersion: 1,
  schema: fileRenameParamsSchema,
  params: {
    find: {
      label: "Find",
      description: "Text or regex pattern to search for in the filename",
    },
    replace: {
      label: "Replace",
      description: "Replacement text (used with Find)",
    },
    case: {
      label: "Case",
      description: "Transform the filename to a specific case",
    },
    prefix: {
      label: "Prefix",
      description: "Text to prepend to the filename",
    },
    suffix: {
      label: "Suffix",
      description: "Text to append before the file extension",
    },
    pattern: {
      label: "Pattern",
      description:
        "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}})",
      placeholder: "{{name}}-compressed.{{ext}}",
    },
  },
};

/** Zod schema for image-compress node parameters (auto-generated from engine). */
export const imageCompressParamsSchema = z.object({
  quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-compress node parameters. */
export type ImageCompressParams = z.infer<typeof imageCompressParamsSchema>;

/** Full schema definition for the image-compress node type (auto-generated from engine). */
export const imageCompressNodeSchema: NodeSchema = {
  nodeType: "image-compress",
  schemaVersion: 1,
  schema: imageCompressParamsSchema,
  params: {
    quality: {
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};

/** Zod schema for image-convert node parameters (auto-generated from engine). */
export const imageConvertParamsSchema = z.object({
  format: z.enum(["jpeg", "png", "webp"] as const).default("jpeg"),
  quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-convert node parameters. */
export type ImageConvertParams = z.infer<typeof imageConvertParamsSchema>;

/** Full schema definition for the image-convert node type (auto-generated from engine). */
export const imageConvertNodeSchema: NodeSchema = {
  nodeType: "image-convert",
  schemaVersion: 1,
  schema: imageConvertParamsSchema,
  params: {
    format: {
      label: "Output Format",
      description: "The target image format to convert to",
    },
    quality: {
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};

/** Zod schema for image-resize node parameters (auto-generated from engine). */
export const imageResizeParamsSchema = z.object({
  width: z.number().min(1).optional(),
  height: z.number().min(1).optional(),
  maintainAspect: z.boolean().optional().default(true),
  quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-resize node parameters. */
export type ImageResizeParams = z.infer<typeof imageResizeParamsSchema>;

/** Full schema definition for the image-resize node type (auto-generated from engine). */
export const imageResizeNodeSchema: NodeSchema = {
  nodeType: "image-resize",
  schemaVersion: 1,
  schema: imageResizeParamsSchema,
  params: {
    width: {
      label: "Width",
      description: "Target width in pixels",
    },
    height: {
      label: "Height",
      description: "Target height in pixels",
    },
    maintainAspect: {
      label: "Maintain Aspect Ratio",
      description: "Keep the original width-to-height ratio when resizing",
    },
    quality: {
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};

/** Zod schema for spreadsheet-clean node parameters (auto-generated from engine). */
export const spreadsheetCleanParamsSchema = z.object({
  trimWhitespace: z.boolean().optional().default(true),
  removeEmptyRows: z.boolean().optional().default(true),
  removeDuplicates: z.boolean().optional().default(true),
});

/** Inferred TypeScript type for spreadsheet-clean node parameters. */
export type SpreadsheetCleanParams = z.infer<typeof spreadsheetCleanParamsSchema>;

/** Full schema definition for the spreadsheet-clean node type (auto-generated from engine). */
export const spreadsheetCleanNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-clean",
  schemaVersion: 1,
  schema: spreadsheetCleanParamsSchema,
  params: {
    trimWhitespace: {
      label: "Trim Whitespace",
      description: "Remove leading and trailing whitespace from every cell",
    },
    removeEmptyRows: {
      label: "Remove Empty Rows",
      description: "Skip rows where every cell is blank",
    },
    removeDuplicates: {
      label: "Remove Duplicates",
      description: "Remove duplicate rows, keeping the first occurrence",
    },
  },
};

/** Zod schema for spreadsheet-rename node parameters (auto-generated from engine). */
export const spreadsheetRenameParamsSchema = z.object({
  columns: z.record(z.string()).optional(),
});

/** Inferred TypeScript type for spreadsheet-rename node parameters. */
export type SpreadsheetRenameParams = z.infer<typeof spreadsheetRenameParamsSchema>;

/** Full schema definition for the spreadsheet-rename node type (auto-generated from engine). */
export const spreadsheetRenameNodeSchema: NodeSchema = {
  nodeType: "spreadsheet-rename",
  schemaVersion: 1,
  schema: spreadsheetRenameParamsSchema,
  params: {
    columns: {
      label: "Column Mapping",
      description: 'Map of old column names to new names (e.g., {"Name": "full_name"})',
    },
  },
};
