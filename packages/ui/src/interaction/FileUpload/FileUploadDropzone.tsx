"use client";

import { useCallback, type ComponentProps } from "react";

import { ScaleIn } from "../../animation/Animate";
import type { Card } from "../../surface/Card";

import { useFileUploadContext } from "./context";
import { DropzoneCard } from "./DropzoneCard";

interface FileUploadDropzoneProps extends ComponentProps<typeof Card> {
  /** Skip the ScaleIn entrance animation. */
  disableAnimation?: boolean;
}

export function FileUploadDropzone({
  disableAnimation,
  className,
  children,
  ...props
}: FileUploadDropzoneProps) {
  const ctx = useFileUploadContext("FileUpload.Dropzone");

  const handleClick = useCallback(() => {
    if (!ctx.disabled) ctx.open();
  }, [ctx.disabled, ctx.open]);

  const card = (
    <DropzoneCard ctx={ctx} onClick={handleClick} className={className} {...props}>
      {children}
    </DropzoneCard>
  );

  if (disableAnimation) return card;
  return <ScaleIn>{card}</ScaleIn>;
}
