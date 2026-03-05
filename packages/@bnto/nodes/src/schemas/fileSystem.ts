/**
 * File System node schema — parameters for file operations.
 *
 * Go source: engine/pkg/node/library/filesystem/filesystem.go
 * Validator: engine/pkg/validator/validators.go -> validateFileSystem
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Valid file system operations. */
export const FILE_OPERATIONS = [
  "read",
  "write",
  "copy",
  "move",
  "delete",
  "mkdir",
  "exists",
  "list",
] as const;

/** Zod schema for file-system node parameters. */
export const fileSystemParamsSchema = z.object({
  operation: z.enum(FILE_OPERATIONS),
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
    path: {
      label: "Path",
      description: "File or directory path. Supports glob patterns for list and delete.",
      placeholder: "/path/to/file.txt",
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
