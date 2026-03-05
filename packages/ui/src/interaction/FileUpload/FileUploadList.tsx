"use client";

import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { Stagger } from "../../animation/Animate";

export function FileUploadList({ className, children, ...props }: ComponentProps<"div">) {
  return (
    <Stagger
      interval={60}
      role="list"
      data-slot="file-upload-list"
      {...props}
      className={cn("flex flex-col gap-2", className)}
    >
      {children}
    </Stagger>
  );
}
