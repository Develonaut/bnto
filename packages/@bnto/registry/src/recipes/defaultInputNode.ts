/** Factory for input node definitions used in predefined recipes. */

import type { Definition } from "@bnto/nodes";
import { CURRENT_FORMAT_VERSION } from "@bnto/nodes";

interface InputNodeOptions {
  /** MIME types to accept (e.g., ["image/jpeg", "image/png"]). */
  accept: readonly string[];
  /** File extensions to accept (e.g., [".jpg", ".png"]). */
  extensions: readonly string[];
  /** Human-readable label for the file drop zone. */
  label: string;
  /** Whether to accept multiple files. Defaults to true. */
  multiple?: boolean;
  /** Input mode: "file-upload" (default), "url", or "text". */
  mode?: "file-upload" | "url" | "text";
  /** Placeholder text for the input field. */
  placeholder?: string;
}

function defaultInputNode(options: InputNodeOptions): Definition {
  const params: Record<string, unknown> = {
    mode: options.mode ?? "file-upload",
    accept: [...options.accept],
    extensions: [...options.extensions],
    label: options.label,
    multiple: options.multiple ?? true,
  };
  if (options.placeholder) {
    params.placeholder = options.placeholder;
  }
  return {
    id: "input",
    type: "input",
    version: CURRENT_FORMAT_VERSION,
    name: "Input",
    position: { x: 0, y: 100 },
    metadata: {},
    parameters: params,
    inputPorts: [],
    outputPorts: [{ id: "out-1", name: "files" }],
  };
}

export { defaultInputNode };
export type { InputNodeOptions };
