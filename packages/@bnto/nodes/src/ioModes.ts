/**
 * I/O node mode metadata — single source of truth for mode → icon and label.
 *
 * Consumed by getNodeIcon (icon field) and getNodeSublabel (label field).
 */

import type { InputParams } from "./generated/schemas/input";
import type { OutputParams } from "./generated/schemas/output";

interface IoModeInfo {
  icon: string;
  label: string;
}

const INPUT_MODES: Record<InputParams["mode"], IoModeInfo> = {
  "file-upload": { icon: "file-up", label: "File Upload" },
  text: { icon: "text-cursor-input", label: "Text Input" },
  url: { icon: "link", label: "URL" },
};

const OUTPUT_MODES: Record<OutputParams["mode"], IoModeInfo> = {
  download: { icon: "download", label: "Download" },
  display: { icon: "monitor", label: "Display" },
  preview: { icon: "eye", label: "Preview" },
};

export { INPUT_MODES, OUTPUT_MODES };
export type { IoModeInfo };
