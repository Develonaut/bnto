"use client";

import type { Definition } from "@bnto/core";
import { FileUpload, FileUploadDropzone, FileIcon, Text, UploadIcon } from "@bnto/ui";
import { useFileImport } from "./useFileImport";

/**
 * FileImportDropzone — drop or pick a `.bnto.json` file to import.
 *
 * Uses the @bnto/ui FileUpload compound component for drag-and-drop.
 * Reads the file, parses JSON, validates as a Definition, and calls
 * `onImport` on success. Shows validation errors inline.
 */

interface FileImportDropzoneProps {
  onImport: (definition: Definition) => void;
}

function FileImportDropzone({ onImport }: FileImportDropzoneProps) {
  const { error, files, handleFiles } = useFileImport(onImport);

  return (
    <div className="flex shrink-0 flex-col gap-2">
      <DropzoneUpload files={files} onFiles={handleFiles} />
      {error && <ImportError message={error} />}
    </div>
  );
}

function DropzoneUpload({ files, onFiles }: { files: File[]; onFiles: (f: File[]) => void }) {
  return (
    <FileUpload
      value={files}
      onValueChange={onFiles}
      accept={{ "application/json": [".json"] }}
      maxFiles={1}
    >
      <FileUploadDropzone className="gap-3 px-4 py-6">
        <div className="rounded-full bg-muted p-3 text-muted-foreground">
          <UploadIcon className="size-6" />
        </div>
        <div className="text-center">
          <Text size="sm" className="font-medium">
            Drop a .bnto.json file here
          </Text>
          <Text size="xs" className="mt-1 text-muted-foreground">
            or click to browse
          </Text>
        </div>
      </FileUploadDropzone>
    </FileUpload>
  );
}

function ImportError({ message }: { message: string }) {
  return (
    <div className="flex items-start gap-2 rounded-md bg-destructive/10 px-3 py-2">
      <FileIcon className="mt-0.5 size-4 shrink-0 text-destructive" />
      <Text size="xs" className="text-destructive">
        {message}
      </Text>
    </div>
  );
}

export { FileImportDropzone };
