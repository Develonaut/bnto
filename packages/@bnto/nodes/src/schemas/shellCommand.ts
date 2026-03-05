/**
 * Shell Command node schema — parameters for executing shell commands.
 *
 * Go source: engine/pkg/node/library/shellcommand/shellcommand.go
 * Validator: engine/pkg/validator/validators.go → validateShellCommand
 */

import type { NodeSchema } from "./types";

export const shellCommandSchema: NodeSchema = {
  nodeType: "shell-command",
  schemaVersion: 1,
  parameters: [
    {
      name: "command",
      type: "string",
      required: true,
      label: "Command",
      description: "The shell command to execute.",
      placeholder: "ls",
    },
    {
      name: "args",
      type: "array",
      required: false,
      label: "Arguments",
      description: "Command-line arguments (each element is a separate arg).",
    },
    {
      name: "timeout",
      type: "number",
      required: false,
      label: "Timeout",
      description: "Maximum execution time in seconds.",
      default: 120,
      min: 1,
      max: 86400,
    },
    {
      name: "stream",
      type: "boolean",
      required: false,
      label: "Stream Output",
      description: "Enable line-by-line output streaming for progress feedback.",
      default: false,
    },
    {
      name: "retry",
      type: "number",
      required: false,
      label: "Retry Attempts",
      description: "Number of retry attempts on failure (0 = no retries).",
      default: 0,
      min: 0,
      max: 100,
    },
    {
      name: "retryDelay",
      type: "number",
      required: false,
      label: "Retry Delay",
      description: "Seconds to wait between retry attempts.",
      default: 5,
      min: 0,
      max: 3600,
    },
    {
      name: "stallTimeout",
      type: "number",
      required: false,
      label: "Stall Timeout",
      description:
        "Kill the process if no output is received for this many seconds (0 = disabled).",
      default: 0,
      min: 0,
      max: 86400,
    },
  ],
} as const;
