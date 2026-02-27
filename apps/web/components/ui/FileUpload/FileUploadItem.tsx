"use client";

import { useMemo, type ComponentProps } from "react";

import { cn } from "@/lib/cn";
import { Animate } from "@/components/ui/Animate";

import { FileUploadItemContext } from "./context";

export function FileUploadItem({
  value,
  index,
  className,
  children,
  ...props
}: ComponentProps<"div"> & { value: File; index?: number }) {
  const ctx = useMemo(() => ({ file: value }), [value]);

  return (
    <FileUploadItemContext.Provider value={ctx}>
      <Animate.SlideUp index={index} easing="spring-bouncier">
        <div
          role="listitem"
          data-slot="file-upload-item"
          {...props}
          className={cn("relative", className)}
        >
          {children}
        </div>
      </Animate.SlideUp>
    </FileUploadItemContext.Provider>
  );
}
