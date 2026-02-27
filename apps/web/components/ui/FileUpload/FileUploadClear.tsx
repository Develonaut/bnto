"use client";

import { useCallback, type ComponentProps, type MouseEvent } from "react";
import { Slot } from "@radix-ui/react-slot";

import { useFileUploadContext } from "./context";

interface FileUploadClearProps extends ComponentProps<"button"> {
  asChild?: boolean;
}

export function FileUploadClear({
  asChild,
  onClick: onClickProp,
  ...props
}: FileUploadClearProps) {
  const { clearFiles, disabled } = useFileUploadContext("FileUpload.Clear");

  const onClick = useCallback(
    (e: MouseEvent<HTMLButtonElement>) => {
      onClickProp?.(e);
      if (!e.defaultPrevented) clearFiles();
    },
    [onClickProp, clearFiles],
  );

  const Comp = asChild ? Slot : "button";

  return (
    <Comp
      type="button"
      data-slot="file-upload-clear"
      data-disabled={disabled ? "" : undefined}
      {...props}
      disabled={disabled}
      onClick={onClick}
    />
  );
}
