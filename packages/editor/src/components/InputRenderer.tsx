"use client";

import type { Definition } from "@bnto/nodes";
import { deriveAcceptedTypes } from "@bnto/core";
import { FileUpload, toDropzoneAccept } from "@bnto/ui";
import { DropzoneContent } from "./DropzoneContent";

interface InputRendererProps {
  definition: Definition;
  files: File[];
  onFilesChange: (files: File[]) => void;
  disabled?: boolean;
}

/**
 * Generic input renderer — reads the input node from a recipe definition
 * and renders the appropriate input widget.
 *
 * Phase 1: Only `file-upload` mode is implemented. `text` and `url` modes
 * render placeholder UI for forward compatibility.
 */
export function InputRenderer({
  definition,
  files,
  onFilesChange,
  disabled,
}: InputRendererProps) {
  const { accept, label } = deriveAcceptedTypes(definition);
  const inputNode = definition.nodes?.find((n) => n.type === "input");
  const mode = (inputNode?.parameters?.mode as string) ?? "file-upload";

  if (mode === "text") {
    return (
      <div className="rounded-lg border border-dashed border-border p-8 text-center text-muted-foreground">
        Text input mode coming soon
      </div>
    );
  }

  if (mode === "url") {
    return (
      <div className="rounded-lg border border-dashed border-border p-8 text-center text-muted-foreground">
        URL input mode coming soon
      </div>
    );
  }

  // file-upload mode (default)
  const dropzoneAccept = toDropzoneAccept(accept);

  return (
    <FileUpload
      value={files}
      onValueChange={onFilesChange}
      accept={dropzoneAccept}
      multiple
      disabled={disabled}
    >
      <FileUpload.Dropzone className="gap-3 px-4 py-8 sm:px-6 sm:py-10">
        <DropzoneContent label={label} />
      </FileUpload.Dropzone>
    </FileUpload>
  );
}
