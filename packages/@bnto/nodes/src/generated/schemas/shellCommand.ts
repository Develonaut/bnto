/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for shell-command node parameters. */
export const shellCommandParamsSchema = z.object({
  command: z.string().optional(),
  args: z
    .array(
      z.union([z.string(), z.array(z.string()), z.object({ each: z.array(z.string()) }).strict()]),
    )
    .optional(),
  outputMode: z.string().optional().default("stdout"),
  timeout: z.number().optional().default(300),
  maxOutputSize: z.number().optional().default(500),
  env: z.record(z.unknown()).optional(),
  search_path: z.record(z.unknown()).optional(),
});

/** Inferred TypeScript type for shell-command node parameters. */
export type ShellCommandParams = z.infer<typeof shellCommandParamsSchema>;

/** Full schema definition for the shell-command node type. */
export const shellCommandNodeSchema: NodeSchema = {
  nodeType: "shell-command",
  schemaVersion: 1,
  schema: shellCommandParamsSchema,
  params: {
    command: {
      label: "Command",
      description: "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH.",
      placeholder: "ffmpeg",
    },
    args: {
      label: "Arguments",
      description:
        'Command arguments: text, conditional groups (nested lists, dropped when any part is empty) and {"each": [...]} groups, repeated once per incoming file with {{path}}, {{file}} and {{item.KEY}} filled.',
    },
    outputMode: {
      label: "Output Mode",
      description:
        "How to collect output. 'stdout' captures command output. 'file' reads files written by the command to a temp directory (use {{output_dir}} in args to inject the path).",
    },
    timeout: {
      label: "Timeout",
      description: "Maximum execution time in seconds. Default: 300.",
    },
    maxOutputSize: {
      label: "Max Output Size",
      description: "Maximum size per output file in megabytes. Default: 100 MB.",
    },
    env: {
      label: "Environment",
      description: "Additional environment variables for the command.",
      surfaceable: false,
    },
    search_path: {
      label: "Search Path",
      description:
        "Folders to find the command in, in order, instead of PATH. The command                       is still a bare name and runs by its full path; relative folders are                       under the working directory.",
      surfaceable: false,
    },
  },
};
