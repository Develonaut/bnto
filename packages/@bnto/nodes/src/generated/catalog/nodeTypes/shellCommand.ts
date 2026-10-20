/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for shell-command. */
export const shellCommandNodeType: NodeTypeInfo = {
  name: "shell-command",
  label: "Run Shell Command",
  description:
    "Run shell commands with stall detection, retry, and streaming output. Still runs under bnto run --dry-run, so a recipe run dry uses it only to read.",
  category: "system",
  isContainer: false,
  platforms: ["server"] as const,
  icon: "terminal",
};
