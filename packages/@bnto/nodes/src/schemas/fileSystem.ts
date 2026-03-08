/**
 * File System node schema — parameters for file operations.
 *
 * Engine-sourced operation: "rename" with find/replace/case/prefix/suffix/pattern.
 * Legacy Go-era operations (read/write/copy/move/delete) kept for compat.
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";
import { PROCESSOR_MAP } from "../generated/catalog";

/** Valid file system operations (engine + legacy). */
export const FILE_OPERATIONS = [
  "rename",
  "read",
  "write",
  "copy",
  "move",
  "delete",
  "mkdir",
  "exists",
  "list",
] as const;

// Pull case options from engine catalog
const renameProc = PROCESSOR_MAP.get("file-system:rename");
const caseParam = renameProc?.parameters.find((p) => p.name === "case");
const CASE_OPTIONS = (caseParam?.options ?? ["lower", "upper", "title"]) as readonly string[];

/** Zod schema for file-system node parameters. */
export const fileSystemParamsSchema = z.object({
  operation: z.enum(FILE_OPERATIONS),
  // Engine-implemented rename params
  find: z.string().optional(),
  replace: z.string().optional(),
  case: z.enum(CASE_OPTIONS as [string, ...string[]]).optional(),
  prefix: z.string().optional(),
  suffix: z.string().optional(),
  pattern: z.string().optional(),
  // Legacy Go-era params (not in engine)
  path: z.string().optional(),
  content: z.string().optional(),
  source: z.string().optional(),
  dest: z.string().optional(),
});

/** Inferred TypeScript type for file-system node parameters. */
export type FileSystemParams = z.infer<typeof fileSystemParamsSchema>;

/** Full schema definition for the file-system node type. */
export const fileSystemNodeSchema: NodeSchemaDefinition = {
  nodeType: "file-system",
  schemaVersion: 1,
  schema: fileSystemParamsSchema,
  params: {
    operation: {
      label: "Operation",
      description: "The file system operation to perform.",
    },
    // Engine rename params
    find: {
      label: "Find",
      description: "Text or regex pattern to search for in the filename.",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    replace: {
      label: "Replace",
      description: "Replacement text (used with Find).",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    case: {
      label: "Case",
      description: "Transform the filename to a specific case.",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    prefix: {
      label: "Prefix",
      description: "Text to prepend to the filename.",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    suffix: {
      label: "Suffix",
      description: "Text to append before the file extension.",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    pattern: {
      label: "Pattern",
      description: "Template for the output filename.",
      visibleWhen: { param: "operation", equals: "rename" },
    },
    // Legacy params
    path: {
      label: "Path",
      description: "File or directory path.",
      placeholder: "/path/to/file.txt",
      visibleWhen: [
        { param: "operation", equals: "read" },
        { param: "operation", equals: "write" },
        { param: "operation", equals: "delete" },
        { param: "operation", equals: "mkdir" },
        { param: "operation", equals: "exists" },
        { param: "operation", equals: "list" },
      ],
    },
    content: {
      label: "Content",
      description: "Content to write to the file.",
      visibleWhen: { param: "operation", equals: "write" },
      requiredWhen: { param: "operation", equals: "write" },
    },
    source: {
      label: "Source",
      description: "Source file path for copy or move operations.",
      visibleWhen: [
        { param: "operation", equals: "copy" },
        { param: "operation", equals: "move" },
      ],
      requiredWhen: [
        { param: "operation", equals: "copy" },
        { param: "operation", equals: "move" },
      ],
    },
    dest: {
      label: "Destination",
      description: "Destination file path for copy or move operations.",
      visibleWhen: [
        { param: "operation", equals: "copy" },
        { param: "operation", equals: "move" },
      ],
      requiredWhen: [
        { param: "operation", equals: "copy" },
        { param: "operation", equals: "move" },
      ],
    },
  },
};
