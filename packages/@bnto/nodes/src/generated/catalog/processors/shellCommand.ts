/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for shell-command. */
export const shellCommandProcessor: ProcessorDef = {
  nodeType: "shell-command",
  name: "Shell Command",
  description: "Execute external CLI tools with security validation.",
  category: "system",
  accepts: [] as const,
  platforms: ["cli", "server", "desktop"] as const,
  parameters: [
    {
      name: "command",
      label: "Command",
      description: "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH.",
      type: "string" as const,
      placeholder: "ffmpeg",
    },
    {
      name: "args",
      label: "Arguments",
      description: "Command arguments as an array of strings.",
      type: "string" as const,
    },
    {
      name: "outputMode",
      label: "Output Mode",
      description:
        "How to collect output. 'stdout' captures command output. 'file' reads files written by the command to a temp directory (use {{output_dir}} in args to inject the path).",
      type: "string" as const,
      default: "stdout",
    },
    {
      name: "timeout",
      label: "Timeout",
      description: "Maximum execution time in seconds. Default: 300.",
      type: "number" as const,
      default: 300,
    },
    {
      name: "env",
      label: "Environment",
      description: "Additional environment variables for the command.",
      type: "object" as const,
      surfaceable: false,
    },
  ],
  inputCardinality: "source" as const,
};
