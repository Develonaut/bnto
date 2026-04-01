"use client";

import { useCallback, useRef } from "react";
import { Button, Text, Row } from "@bnto/ui";
import type { ControlProps } from "./types";
import { readAsBase64 } from "./readAsBase64";

/**
 * FileControl — file picker that stores the selected file as a base64 data URI.
 *
 * Reads `fieldConfig.accept` for MIME type filtering on the file input.
 * The value stored in the form is the full data URI string
 * (e.g., "data:image/png;base64,...").
 */
function FileControl({ id, fieldConfig, value, onChange }: ControlProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const accept = fieldConfig?.accept?.join(",") ?? "image/*";
  const hasFile = typeof value === "string" && value.length > 0;

  const handleClick = useCallback(() => inputRef.current?.click(), []);

  const handleChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;
      const dataUri = await readAsBase64(file);
      onChange(dataUri);
    },
    [onChange],
  );

  const handleClear = useCallback(() => {
    onChange("");
    if (inputRef.current) inputRef.current.value = "";
  }, [onChange]);

  return (
    <Row className="items-center gap-2" data-testid={`control-file-${id}`}>
      <input
        ref={inputRef}
        id={id}
        type="file"
        accept={accept}
        onChange={handleChange}
        className="hidden"
      />
      <Button variant="outline" size="sm" onClick={handleClick}>
        {hasFile ? "Replace" : "Choose file"}
      </Button>
      {hasFile && (
        <>
          <Text size="xs" color="muted" className="truncate max-w-32">
            Selected
          </Text>
          <Button variant="ghost" size="sm" onClick={handleClear}>
            Clear
          </Button>
        </>
      )}
    </Row>
  );
}

export { FileControl };
