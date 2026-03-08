/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 *
 * Zod schemas and UI metadata for engine-backed node types.
 * Run `task nodes:generate` to regenerate after engine changes.
 *
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "../schemas/types";

/** Valid file-system operations — derived from engine processors. */
export const FILE_SYSTEM_OPERATIONS = ["rename"] as const;

/** Zod schema for file-system node parameters (auto-generated from engine). */
export const fileSystemParamsSchema = z.object({
  operation: z.enum(FILE_SYSTEM_OPERATIONS as unknown as [string, ...string[]]),
  find: z.string().optional(),
  replace: z.string().optional(),
  case: z.enum(["lower", "upper", "title"] as const).optional(),
  prefix: z.string().optional(),
  suffix: z.string().optional(),
  pattern: z.string().optional(),
});

/** Inferred TypeScript type for file-system node parameters. */
export type FileSystemParams = z.infer<typeof fileSystemParamsSchema>;

/** Full schema definition for the file-system node type (auto-generated from engine). */
export const fileSystemNodeSchema: NodeSchemaDefinition = {
  nodeType: "file-system",
  schemaVersion: 1,
  schema: fileSystemParamsSchema,
  params: {
    operation: {
      label: "Operation",
      description: "The file-system operation to perform.",
    },
    find: {
      label: "Find",
      description: "Text or regex pattern to search for in the filename",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    replace: {
      label: "Replace",
      description: "Replacement text (used with Find)",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    case: {
      label: "Case",
      description: "Transform the filename to a specific case",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    prefix: {
      label: "Prefix",
      description: "Text to prepend to the filename",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    suffix: {
      label: "Suffix",
      description: "Text to append before the file extension",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    pattern: {
      label: "Pattern",
      description:
        "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}})",
      placeholder: "{{name}}-compressed.{{ext}}",
      visibleWhen: { param: "operation", equals: "rename" },
    },
  },
};

/** Valid image operations — derived from engine processors. */
export const IMAGE_OPERATIONS = ["compress", "convert", "resize"] as const;

/** Zod schema for image node parameters (auto-generated from engine). */
export const imageParamsSchema = z.object({
  operation: z.enum(IMAGE_OPERATIONS as unknown as [string, ...string[]]),
  quality: z.number().min(1).max(100).optional().default(80),
  format: z.enum(["jpeg", "png", "webp"] as const).optional(),
  width: z.number().min(1).optional(),
  height: z.number().min(1).optional(),
  maintainAspect: z.boolean().optional().default(true),
});

/** Inferred TypeScript type for image node parameters. */
export type ImageParams = z.infer<typeof imageParamsSchema>;

/** Full schema definition for the image node type (auto-generated from engine). */
export const imageNodeSchema: NodeSchemaDefinition = {
  nodeType: "image",
  schemaVersion: 1,
  schema: imageParamsSchema,
  params: {
    operation: {
      label: "Operation",
      description: "The image operation to perform.",
    },
    quality: {
      label: "Quality",
      description: "Compression quality (1 = smallest file, 100 = best quality)",
    },
    format: {
      label: "Output Format",
      description: "The target image format to convert to",
      visibleWhen: { param: "operation", equals: "convert" },
    },
    width: {
      label: "Width",
      description: "Target width in pixels",
      visibleWhen: { param: "operation", equals: "resize" },
    },
    height: {
      label: "Height",
      description: "Target height in pixels",
      visibleWhen: { param: "operation", equals: "resize" },
    },
    maintainAspect: {
      label: "Maintain Aspect Ratio",
      description: "Keep the original width-to-height ratio when resizing",
      visibleWhen: { param: "operation", equals: "resize" },
    },
  },
};

/** Valid spreadsheet operations — derived from engine processors. */
export const SPREADSHEET_OPERATIONS = ["clean", "rename"] as const;

/** Zod schema for spreadsheet node parameters (auto-generated from engine). */
export const spreadsheetParamsSchema = z.object({
  operation: z.enum(SPREADSHEET_OPERATIONS as unknown as [string, ...string[]]),
  trimWhitespace: z.boolean().optional().default(true),
  removeEmptyRows: z.boolean().optional().default(true),
  removeDuplicates: z.boolean().optional().default(true),
  columns: z.record(z.string()).optional(),
});

/** Inferred TypeScript type for spreadsheet node parameters. */
export type SpreadsheetParams = z.infer<typeof spreadsheetParamsSchema>;

/** Full schema definition for the spreadsheet node type (auto-generated from engine). */
export const spreadsheetNodeSchema: NodeSchemaDefinition = {
  nodeType: "spreadsheet",
  schemaVersion: 1,
  schema: spreadsheetParamsSchema,
  params: {
    operation: {
      label: "Operation",
      description: "The spreadsheet operation to perform.",
    },
    trimWhitespace: {
      label: "Trim Whitespace",
      description: "Remove leading and trailing whitespace from every cell",
      visibleWhen: { param: "operation", equals: "clean" },
    },
    removeEmptyRows: {
      label: "Remove Empty Rows",
      description: "Skip rows where every cell is blank",
      visibleWhen: { param: "operation", equals: "clean" },
    },
    removeDuplicates: {
      label: "Remove Duplicates",
      description: "Remove duplicate rows, keeping the first occurrence",
      visibleWhen: { param: "operation", equals: "clean" },
    },
    columns: {
      label: "Column Mapping",
      description: 'Map of old column names to new names (e.g., {"Name": "full_name"})',
      visibleWhen: { param: "operation", equals: "rename" },
    },
  },
};
