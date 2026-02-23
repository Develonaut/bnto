"use client";

import * as React from "react";
import { useDropzone, type DropzoneOptions } from "react-dropzone";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const dropzoneVariants = cva(
  [
    "flex cursor-pointer flex-col items-center justify-center rounded-xl border-2 border-dashed",
    "motion-safe:transition-all motion-safe:duration-fast",
    "outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
  ],
  {
    variants: {
      size: {
        default: "gap-3 px-6 py-10",
        sm: "gap-2 px-4 py-6",
      },
      state: {
        idle: "border-border bg-muted/30 hover:border-muted-foreground/40 hover:bg-muted/50",
        active: "border-primary bg-primary/5 shadow-md",
      },
    },
    defaultVariants: {
      size: "default",
      state: "idle",
    },
  },
);

interface DropzoneProps
  extends DropzoneOptions,
    Omit<VariantProps<typeof dropzoneVariants>, "state"> {
  className?: string;
  children: (props: { isDragActive: boolean }) => React.ReactNode;
}

function Dropzone({ className, size, children, ...options }: DropzoneProps) {
  const { getRootProps, getInputProps, isDragActive } = useDropzone(options);

  return (
    <div
      {...getRootProps({
        role: "button",
        "data-slot": "dropzone",
        className: cn(
          dropzoneVariants({
            size,
            state: isDragActive ? "active" : "idle",
          }),
          className,
        ),
      })}
    >
      <input data-slot="dropzone-input" {...getInputProps()} />
      {children({ isDragActive })}
    </div>
  );
}

export { Dropzone, dropzoneVariants };
