/**
 * Derive output delivery config from a definition's output node.
 *
 * Reads the output node's parameters (mode, zip, autoDownload, label)
 * and returns them for result presentation. Falls back to defaults
 * when no output node is found.
 */

import { getOutputNode } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";

interface OutputConfig {
  /** How results are delivered. */
  mode: "download" | "display" | "preview";
  /** Whether to auto-zip multiple output files. */
  zip: boolean;
  /** Whether to auto-download on completion. */
  autoDownload: boolean;
  /** Label for the download button or display section. */
  label: string;
  /** Filename template for output files. */
  filename?: string;
}

const DEFAULTS: OutputConfig = {
  mode: "download",
  zip: true,
  autoDownload: false,
  label: "Results",
};

/**
 * Derives output delivery config from a definition's output node.
 *
 * Returns the output mode, zip preference, auto-download flag, and label.
 * If no output node is found, returns sensible defaults (download mode,
 * zip enabled, no auto-download).
 */
export function deriveOutputConfig(definition: Definition): OutputConfig {
  const outputNode = getOutputNode(definition);
  if (!outputNode) return DEFAULTS;

  const params = outputNode.parameters;
  const mode = (params.mode as OutputConfig["mode"] | undefined) ?? "download";
  const zip = (params.zip as boolean | undefined) ?? true;
  const autoDownload = (params.autoDownload as boolean | undefined) ?? false;
  const label = (params.label as string | undefined) ?? "Results";
  const filename = params.filename as string | undefined;

  return { mode, zip, autoDownload, label, filename };
}
