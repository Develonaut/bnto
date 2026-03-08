/**
 * Shell Command node schema — parameters for executing shell commands.
 *
 * Server-only — not available in browser execution.
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Zod schema for shell-command node parameters. */
export const shellCommandParamsSchema = z.object({
  command: z.string(),
  args: z.array(z.string()).optional(),
  timeout: z.number().min(1).max(86400).optional().default(120),
  stream: z.boolean().optional().default(false),
  retry: z.number().min(0).max(100).optional().default(0),
  retryDelay: z.number().min(0).max(3600).optional().default(5),
  stallTimeout: z.number().min(0).max(86400).optional().default(0),
});

/** Inferred TypeScript type for shell-command node parameters. */
export type ShellCommandParams = z.infer<typeof shellCommandParamsSchema>;

/** Full schema definition for the shell-command node type. */
export const shellCommandNodeSchema: NodeSchemaDefinition = {
  nodeType: "shell-command",
  schemaVersion: 1,
  schema: shellCommandParamsSchema,
  params: {
    command: {
      label: "Command",
      description: "The shell command to execute.",
      placeholder: "ls",
    },
    args: {
      label: "Arguments",
      description: "Command-line arguments (each element is a separate arg).",
    },
    timeout: {
      label: "Timeout",
      description: "Maximum execution time in seconds.",
    },
    stream: {
      label: "Stream Output",
      description: "Enable line-by-line output streaming for progress feedback.",
    },
    retry: {
      label: "Retry Attempts",
      description: "Number of retry attempts on failure (0 = no retries).",
    },
    retryDelay: {
      label: "Retry Delay",
      description: "Seconds to wait between retry attempts.",
    },
    stallTimeout: {
      label: "Stall Timeout",
      description:
        "Kill the process if no output is received for this many seconds (0 = disabled).",
    },
  },
};
