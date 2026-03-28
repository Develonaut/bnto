"use client";

import type { Definition } from "@bnto/core";
import { deriveAcceptedTypes, NODE_TYPE_INFO } from "@bnto/core";
import { FileUpload, FileUploadDropzone, toDropzoneAccept } from "@bnto/ui";
import { DropzoneContent } from "./DropzoneContent";
import { PlaceholderInputMode } from "./PlaceholderInputMode";

interface InputRendererProps {
  definition: Definition;
  files: File[];
  onFilesChange: (files: File[]) => void;
  disabled?: boolean;
}

/**
 * Generic input renderer — reads the input node from a recipe definition
 * and renders the appropriate input widget.
 */
export function InputRenderer({ definition, files, onFilesChange, disabled }: InputRendererProps) {
  const { accept, label } = deriveAcceptedTypes(definition);
  const inputNode = definition.nodes?.find((n) => n.type === NODE_TYPE_INFO.input.name);
  const mode = (inputNode?.parameters?.mode as string) ?? "file-upload";

  if (mode === "text") return <PlaceholderInputMode label="Text input mode coming soon" />;
  if (mode === "url") return <PlaceholderInputMode label="URL input mode coming soon" />;

  return (
    <FileUpload
      value={files}
      onValueChange={onFilesChange}
      accept={toDropzoneAccept(accept)}
      multiple
      disabled={disabled}
    >
      <FileUploadDropzone className="gap-3 px-4 py-8 sm:px-6 sm:py-10">
        <DropzoneContent label={label} />
      </FileUploadDropzone>
    </FileUpload>
  );
}
