"use client";

import { useCallback, useState } from "react";
import { validateDefinition } from "@bnto/core";
import type { Definition } from "@bnto/core";
import { FileUpload, FileUploadDropzone, FileIcon, Text, UploadIcon } from "@bnto/ui";

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
  const [error, setError] = useState<string | null>(null);
  const [files, setFiles] = useState<File[]>([]);

  const handleFiles = useCallback(
    (incoming: File[]) => {
      setError(null);
      setFiles(incoming);

      const file = incoming[0];
      if (!file) return;

      if (!file.name.endsWith(".json") && !file.name.endsWith(".bnto.json")) {
        setError("Only .bnto.json or .json files are accepted.");
        return;
      }

      const reader = new FileReader();
      reader.onload = () => {
        try {
          const parsed = JSON.parse(reader.result as string) as Definition;
          const errors = validateDefinition(parsed);
          if (errors.length > 0) {
            setError(
              `Invalid definition: ${errors[0].message}${errors.length > 1 ? ` (+${errors.length - 1} more)` : ""}`,
            );
            return;
          }
          onImport(parsed);
        } catch {
          setError("Invalid JSON — could not parse the file.");
        }
      };
      reader.readAsText(file);
    },
    [onImport],
  );

  return (
    <div className="flex shrink-0 flex-col gap-2">
      <FileUpload
        value={files}
        onValueChange={handleFiles}
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
      {error && (
        <div className="flex items-start gap-2 rounded-md bg-destructive/10 px-3 py-2">
          <FileIcon className="mt-0.5 size-4 shrink-0 text-destructive" />
          <Text size="xs" className="text-destructive">
            {error}
          </Text>
        </div>
      )}
    </div>
  );
}

export { FileImportDropzone };
