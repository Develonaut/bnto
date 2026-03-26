"use client";

import { useCallback, type ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { ScaleIn } from "../../animation/Animate";
import { Button } from "../Button";
import { Card } from "../../surface/Card";

import { useFileUploadContext } from "./context";

interface FileUploadDropzoneProps extends ComponentProps<typeof Card> {
  /** Skip the ScaleIn entrance animation. */
  disableAnimation?: boolean;
}

const DROPZONE_CN = [
  "flex h-auto w-full flex-col items-center justify-center rounded-xl",
  "outline-border [outline-style:dashed] [outline-width:3px] [outline-offset:-8px]",
  "focus-visible:[outline-style:solid] focus-visible:[outline-color:var(--focus-ring)] focus-visible:[outline-width:2px]",
  "data-disabled:pointer-events-none",
].join(" ");

export function FileUploadDropzone({
  disableAnimation,
  className,
  children,
  ...props
}: FileUploadDropzoneProps) {
  const { getRootProps, getInputProps, isDragActive, open, disabled } =
    useFileUploadContext("FileUpload.Dropzone");

  const handleClick = useCallback(() => {
    if (!disabled) open();
  }, [disabled, open]);

  const card = (
    <Button asChild hovered={isDragActive}>
      <Card
        {...getRootProps()}
        role="button"
        tabIndex={disabled ? undefined : 0}
        data-slot="file-upload-dropzone"
        data-disabled={disabled ? "" : undefined}
        data-dragging={isDragActive ? "" : undefined}
        {...props}
        className={cn(DROPZONE_CN, className)}
        onClick={handleClick}
      >
        <input {...getInputProps()} aria-label="File upload" data-testid="file-input" />
        {children}
      </Card>
    </Button>
  );

  if (disableAnimation) return card;
  return <ScaleIn>{card}</ScaleIn>;
}
