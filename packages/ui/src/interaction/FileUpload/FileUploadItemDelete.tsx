"use client";

import { useCallback, type ComponentProps, type MouseEvent } from "react";
import { Slot } from "@radix-ui/react-slot";

import { useFileUploadContext, useFileUploadItemContext } from "./context";

interface FileUploadItemDeleteProps extends ComponentProps<"button"> {
  asChild?: boolean;
}

export function FileUploadItemDelete({
  asChild,
  onClick: onClickProp,
  ...props
}: FileUploadItemDeleteProps) {
  const { removeFile } = useFileUploadContext("FileUpload.ItemDelete");
  const { file } = useFileUploadItemContext("FileUpload.ItemDelete");

  const onClick = useCallback(
    (e: MouseEvent<HTMLButtonElement>) => {
      onClickProp?.(e);
      if (!e.defaultPrevented) removeFile(file);
    },
    [onClickProp, removeFile, file],
  );

  const Comp = asChild ? Slot : "button";

  return (
    <Comp
      type="button"
      data-slot="file-upload-item-delete"
      {...props}
      onClick={onClick}
    />
  );
}
