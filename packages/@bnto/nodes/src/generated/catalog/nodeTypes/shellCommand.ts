/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for shell-command. */
export const shellCommandNodeType: NodeTypeInfo = {
  name: "shell-command",
  label: "Shell Command",
  description: "Execute shell commands with stall detection, retry, and streaming output.",
  category: "system",
  isContainer: false,
  platforms: ["server"] as const,
  icon: "terminal",
};
