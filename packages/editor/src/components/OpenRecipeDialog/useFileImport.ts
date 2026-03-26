"use client";

import { useCallback, useState } from "react";
import { validateDefinition } from "@bnto/core";
import type { Definition } from "@bnto/core";

/** Hook for file import logic -- reads, parses, validates a .bnto.json file. */
function useFileImport(onImport: (definition: Definition) => void) {
  const [error, setError] = useState<string | null>(null);
  const [files, setFiles] = useState<File[]>([]);

  const handleFiles = useCallback(
    (incoming: File[]) => {
      setError(null);
      setFiles(incoming);
      const file = incoming[0];
      if (!file) return;
      if (!isJsonFile(file.name)) {
        setError("Only .bnto.json or .json files are accepted.");
        return;
      }
      readAndValidate(file, onImport, setError);
    },
    [onImport],
  );

  return { error, files, handleFiles };
}

function isJsonFile(name: string) {
  return name.endsWith(".json") || name.endsWith(".bnto.json");
}

function readAndValidate(
  file: File,
  onImport: (d: Definition) => void,
  setError: (e: string) => void,
) {
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
}

export { useFileImport };
