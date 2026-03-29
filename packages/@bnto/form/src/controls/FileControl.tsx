"use client";

import { useCallback, useRef, useState } from "react";
import { Button, FileUpIcon, Input, Row, XIcon } from "@bnto/ui";
import type { ControlProps } from "./types";
import { readAsBase64 } from "./readAsBase64";

function FileActionButton({ hasValue, onChoose, onClear }: FileActionButtonProps) {
  if (hasValue) {
    return (
      <Button
        type="button"
        variant="outline"
        size="icon"
        onClick={onClear}
        aria-label="Remove file"
      >
        <XIcon />
      </Button>
    );
  }
  return (
    <Button type="button" variant="outline" size="icon" onClick={onChoose} aria-label="Choose file">
      <FileUpIcon />
    </Button>
  );
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

  return (
    <div data-testid={`control-file-${id}`}>
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
        <FileActionButton hasValue={hasValue} onChoose={handleChoose} onClear={handleClear} />
      </Row>
    </div>
  );
}

interface FileActionButtonProps {
  hasValue: boolean;
  onChoose: () => void;
  onClear: () => void;
}

export { FileControl };
