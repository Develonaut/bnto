/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for shell-run. */
export const shellRunProcessor: ProcessorDef = {
  nodeType: "shell-run",
  name: "Run Shell Command",
  description: "Run external CLI tools with security validation.",
  category: "system",
  accepts: [] as const,
  platforms: ["cli", "server", "desktop"] as const,
  parameters: [
    {
      name: "command",
      label: "Command",
      description:
        "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH and written literally. Set this or 'script', not both.",
      type: "string" as const,
      placeholder: "ffmpeg",
    },
    {
      name: "script",
      label: "Script",
      description:
        "A shell script to run instead of 'command', written literally — no {{templates}}. Values from 'args' arrive as \"$1\", \"$2\"… (\"$0\" is 'bnto'). Quote every expansion, avoid eval, and don't splice values into nested -c / -e code; put -- before values a tool could read as options. The shell starts with a cleared environment and no startup files.",
      type: "string" as const,
      surfaceable: false,
    },
    {
      name: "shell",
      label: "Shell",
      description:
        "Which shell runs 'script': sh, bash, zsh, dash or ksh, by its fixed path (never PATH). Default: sh.",
      type: "string" as const,
      default: "sh",
      surfaceable: false,
    },
    {
      name: "args",
      label: "Arguments",
      description:
        'Command arguments: text, conditional groups (nested lists, dropped when any part is empty) and {"each": [...]} groups, repeated once per incoming file with {{path}}, {{file}} and {{item.KEY}} filled.',
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
      name: "maxOutputSize",
      label: "Max Output Size",
      description: "Maximum size per output file in megabytes. Default: 100 MB.",
      type: "number" as const,
      default: 500,
    },
    {
      name: "env",
      label: "Environment",
      description: "Additional environment variables for the command.",
      type: "object" as const,
      surfaceable: false,
    },
    {
      name: "searchPath",
      label: "Search Path",
      description:
        "Folders to find the command in, in order, instead of PATH. The command is still a bare name and runs by its full path; relative folders are under the working directory.",
      type: "object" as const,
      surfaceable: false,
    },
    {
      name: "cwd",
      label: "Working Folder",
      description:
        "Folder to run the command in, under the working directory (e.g. 'Tools'). Relative args are relative to it; search_path and {{output_dir}} are not. Absolute paths and \"..\" are refused.",
      type: "string" as const,
      placeholder: "Tools",
      surfaceable: false,
    },
  ],
  inputCardinality: "batch" as const,
};
