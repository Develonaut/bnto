/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for shell-run node parameters. */
export const shellRunParamsSchema = z.object({
  command: z.string().optional(),
  script: z.string().optional(),
  shell: z.string().optional().default("sh"),
  args: z
    .array(
      z.union([z.string(), z.array(z.string()), z.object({ each: z.array(z.string()) }).strict()]),
    )
    .optional(),
  outputMode: z.string().optional().default("stdout"),
  timeout: z.number().optional().default(300),
  maxOutputSize: z.number().optional().default(500),
  env: z.record(z.unknown()).optional(),
  searchPath: z.record(z.unknown()).optional(),
  cwd: z.string().optional(),
});

/** Inferred TypeScript type for shell-run node parameters. */
export type ShellRunParams = z.infer<typeof shellRunParamsSchema>;

/** Full schema definition for the shell-run node type. */
export const shellRunNodeSchema: NodeSchema = {
  nodeType: "shell-run",
  schemaVersion: 1,
  schema: shellRunParamsSchema,
  params: {
    command: {
      label: "Command",
      description:
        "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH and written literally. Set this or 'script', not both.",
      placeholder: "ffmpeg",
    },
    script: {
      label: "Script",
      description:
        "A shell script to run instead of 'command', written literally — no {{templates}}. Values from 'args' arrive as \"$1\", \"$2\"… (\"$0\" is 'bnto'). Quote every expansion, avoid eval, and don't splice values into nested -c / -e code; put -- before values a tool could read as options. A PowerShell script reads them as $env:BNTO_ARG_1… ($env:BNTO_ARG_COUNT holds how many); never pass one to Invoke-Expression. The shell starts with a cleared environment and no startup files.",
      surfaceable: false,
    },
    shell: {
      label: "Shell",
      description:
        "Which shell runs 'script': sh, bash, zsh, dash or ksh, powershell (Windows only) or pwsh (PowerShell 7), by its fixed path (never PATH). Default: sh.",
      surfaceable: false,
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
    searchPath: {
      label: "Search Path",
      description:
        "Folders to find the command in, in order, instead of PATH. The command is still a bare name and runs by its full path; relative folders are under the working directory.",
      surfaceable: false,
    },
    cwd: {
      label: "Working Folder",
      description:
        "Folder to run the command in, under the working directory (e.g. 'Tools'). Relative args are relative to it; search_path and {{output_dir}} are not. Absolute paths and \"..\" are refused.",
      placeholder: "Tools",
      surfaceable: false,
    },
  },
};
