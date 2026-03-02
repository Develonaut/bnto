"use client";

import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";
import { Animate } from "@/components/ui/Animate";
import { Card } from "@/components/ui/Card";
import { Pressable } from "@/components/ui/Pressable";

import { useFileUploadContext } from "./context";

export function FileUploadDropzone({
  className,
  children,
  ...props
}: ComponentProps<typeof Card>) {
  const { getRootProps, getInputProps, isDragActive, open, disabled } =
    useFileUploadContext("FileUpload.Dropzone");

  const handleClick = () => {
    if (!disabled) open();
  };

  return (
    <Animate.ScaleIn>
      <Pressable asChild hovered={isDragActive}>
        <Card
          elevation="md"
          {...getRootProps()}
          role="button"
          tabIndex={disabled ? undefined : 0}
          data-slot="file-upload-dropzone"
          data-disabled={disabled ? "" : undefined}
          data-dragging={isDragActive ? "" : undefined}
          {...props}
          className={cn(
            "flex h-auto w-full flex-col items-center justify-center rounded-xl",
            "outline-border [outline-style:dashed] [outline-width:3px] [outline-offset:-8px]",
            "data-disabled:pointer-events-none",
            className,
          )}
          onClick={handleClick}
        >
          <input {...getInputProps()} aria-label="File upload" />
          {children}
        </Card>
      </Pressable>
    </Animate.ScaleIn>
  );
}
