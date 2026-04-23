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
  args: z.string().optional(),
  timeout: z.number().optional().default(300),
  env: z.record(z.unknown()).optional(),
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
      description: "Command arguments as an array of strings.",
    },
    timeout: {
      label: "Timeout",
      description: "Maximum execution time in seconds. Default: 300.",
    },
    env: {
      label: "Environment",
      description: "Additional environment variables for the command.",
      surfaceable: false,
    },
  },
};
