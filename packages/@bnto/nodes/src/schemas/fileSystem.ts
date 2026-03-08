/**
 * File System node schema — parameters for file operations.
 *
 * Operations are derived from the engine catalog — only engine-backed
 * operations are valid. Currently: ["rename"].
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";
import { PROCESSOR_MAP } from "../generated/catalog";
import { getEngineOperations } from "./deriveOperations";

/** Valid file system operations — derived from engine PROCESSORS. */
export const FILE_OPERATIONS = getEngineOperations("file-system");

// Pull case options from engine catalog
const renameProc = PROCESSOR_MAP.get("file-system:rename");
const caseParam = renameProc?.parameters.find((p) => p.name === "case");
const CASE_OPTIONS = (caseParam?.options ?? ["lower", "upper", "title"]) as readonly string[];

/** Zod schema for file-system node parameters. */
export const fileSystemParamsSchema = z.object({
  operation: z.enum(FILE_OPERATIONS as [string, ...string[]]),
  find: z.string().optional(),
  replace: z.string().optional(),
  case: z.enum(CASE_OPTIONS as [string, ...string[]]).optional(),
  prefix: z.string().optional(),
  suffix: z.string().optional(),
  pattern: z.string().optional(),
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
  },
};
