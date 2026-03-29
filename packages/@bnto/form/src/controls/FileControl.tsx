"use client";

import { useCallback, useRef, useState } from "react";
import { Button, Input, Row, UploadIcon, XIcon } from "@bnto/ui";
import type { ControlProps } from "./types";

/** Read a File as a base64-encoded data URI string. */
function readAsBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error("Failed to read file"));
    reader.readAsDataURL(file);
  });
}

function FileControl({ id, fieldConfig, value, onChange }: ControlProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const [fileName, setFileName] = useState("");
  const accept = fieldConfig?.accept?.join(",") ?? "image/*";

  const handleFileChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;
      setFileName(file.name);
      onChange(await readAsBase64(file));
    },
    [onChange],
  );

  const handleClear = useCallback(() => {
    onChange("");
    setFileName("");
    if (inputRef.current) inputRef.current.value = "";
  }, [onChange]);

  const handleChoose = useCallback(() => inputRef.current?.click(), []);

  const hasValue = typeof value === "string" && value.length > 0;
  const isImage = hasValue && (value as string).startsWith("data:image/");

  return (
    <div className="flex flex-col gap-2" data-testid={`control-file-${id}`}>
      <input
        ref={inputRef}
        id={id}
        type="file"
        accept={accept}
        onChange={handleFileChange}
        className="hidden"
      />
      <Row className="gap-2">
        <Input
          readOnly
          value={fileName}
          placeholder="No file chosen"
          wrapperClassName="min-w-0 flex-1"
          onClick={handleChoose}
          className="cursor-pointer"
        />
        {hasValue ? (
          <Button
            type="button"
            variant="outline"
            size="icon"
            onClick={handleClear}
            aria-label="Remove file"
          >
            <XIcon />
          </Button>
        ) : (
          <Button
            type="button"
            variant="outline"
            size="icon"
            onClick={handleChoose}
            aria-label="Choose file"
          >
            <UploadIcon />
          </Button>
        )}
      </Row>
      {isImage && (
        <img
          src={value as string}
          alt="Preview"
          className="max-h-16 max-w-32 rounded border border-border object-contain"
        />
      )}
    </div>
  );
}

export { FileControl };
