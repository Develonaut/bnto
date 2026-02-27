"use client";

import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";
import { Animate } from "@/components/ui/Animate";

export function FileUploadList({
  className,
  children,
  ...props
}: ComponentProps<"div">) {
  return (
    <Animate.Stagger
      interval={60}
      role="list"
      data-slot="file-upload-list"
      {...props}
      className={cn("flex flex-col gap-2", className)}
    >
      {children}
    </Animate.Stagger>
  );
}
