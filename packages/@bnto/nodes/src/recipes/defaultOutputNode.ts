/** Factory for output node definitions used in predefined recipes. */

import type { Definition } from "../definition";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";

interface OutputNodeOptions {
  /** Label for the download button or display section. */
  label: string;
  /** Whether to auto-zip multiple output files. Defaults to true. */
  zip?: boolean;
  /** Whether to auto-download results. Defaults to true. */
  autoDownload?: boolean;
}

function defaultOutputNode(options: OutputNodeOptions): Definition {
  return {
    id: "output",
    type: "output",
    version: CURRENT_FORMAT_VERSION,
    name: "Output",
    position: { x: 500, y: 100 },
    metadata: {},
    parameters: {
      mode: "download",
      label: options.label,
      zip: options.zip ?? true,
      autoDownload: options.autoDownload ?? true,
    },
    inputPorts: [{ id: "in-1", name: "files" }],
    outputPorts: [],
  };
}

export { defaultOutputNode };
export type { OutputNodeOptions };
