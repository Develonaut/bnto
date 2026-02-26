/**
 * File System node schema — parameters for file operations.
 *
 * Go source: engine/pkg/node/library/filesystem/filesystem.go
 * Validator: engine/pkg/validator/validators.go -> validateFileSystem
 */

import type { NodeSchema } from "./types";

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

export const fileSystemSchema: NodeSchema = {
  nodeType: "file-system",
  parameters: [
    {
      name: "operation",
      type: "enum",
      required: true,
      label: "Operation",
      description: "The file system operation to perform.",
      enumValues: FILE_OPERATIONS,
    },
    {
      name: "path",
      type: "string",
      required: false,
      label: "Path",
      description:
        "File or directory path. Supports glob patterns for list and delete.",
      placeholder: "/path/to/file.txt",
    },
    {
      name: "content",
      type: "string",
      required: false,
      label: "Content",
      description: "Content to write to the file.",
      visibleWhen: { param: "operation", equals: "write" },
      requiredWhen: { param: "operation", equals: "write" },
    },
    {
      name: "source",
      type: "string",
      required: false,
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
    {
      name: "dest",
      type: "string",
      required: false,
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
  ],
} as const;
