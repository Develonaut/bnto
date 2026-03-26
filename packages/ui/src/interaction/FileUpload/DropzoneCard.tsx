"use client";

import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { Button } from "../Button";
import { Card } from "../../surface/Card";

import type { FileUploadContextValue } from "./context";

interface DropzoneCardProps extends ComponentProps<typeof Card> {
  ctx: Pick<FileUploadContextValue, "getRootProps" | "isDragActive" | "disabled" | "getInputProps">;
}

const DROPZONE_CN = [
  "flex h-auto w-full flex-col items-center justify-center rounded-xl",
  "outline-border [outline-style:dashed] [outline-width:3px] [outline-offset:-8px]",
  "focus-visible:[outline-style:solid] focus-visible:[outline-color:var(--focus-ring)] focus-visible:[outline-width:2px]",
  "data-disabled:pointer-events-none",
].join(" ");

export function DropzoneCard({ ctx, onClick, className, children, ...props }: DropzoneCardProps) {
  return (
    <Button asChild hovered={ctx.isDragActive}>
      <Card
        {...ctx.getRootProps()}
        role="button"
        tabIndex={ctx.disabled ? undefined : 0}
        data-slot="file-upload-dropzone"
        data-disabled={ctx.disabled ? "" : undefined}
        data-dragging={ctx.isDragActive ? "" : undefined}
        {...props}
        className={cn(DROPZONE_CN, className)}
        onClick={onClick}
      >
        <input {...ctx.getInputProps()} aria-label="File upload" data-testid="file-input" />
        {children}
      </Card>
    </Button>
  );
}
