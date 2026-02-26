"use client";

import * as React from "react";
import { useDropzone, type DropzoneOptions } from "react-dropzone";
import { createCn } from "./createCn";
import { Button } from "./Button";
import { Card } from "./Card";

const dropzoneCn = createCn({
  base: "h-auto w-full flex-col rounded-xl outline-2 outline-dashed outline-offset-[-8px] motion-safe:transition-all motion-safe:duration-fast",
  variants: {
    size: {
      default: "gap-3 px-6 py-10",
      sm: "gap-2 px-4 py-6",
    },
    state: {
      idle: "outline-border",
      active: "outline-primary bg-primary/5",
    },
  },
  defaultVariants: {
    size: "default",
    state: "idle",
  },
});

interface DropzoneProps extends DropzoneOptions {
  className?: string;
  size?: "default" | "sm";
  children: (props: { isDragActive: boolean }) => React.ReactNode;
}

function Dropzone({ className, size, children, ...options }: DropzoneProps) {
  const { getRootProps, getInputProps, isDragActive } = useDropzone(options);

  return (
    <Button asChild variant="outline" size="md">
      <Card
        depth="md"
        className={dropzoneCn(
          {
            size,
            state: isDragActive ? "active" : "idle",
          },
          className,
        )}
        {...getRootProps({
          "data-slot": "dropzone",
        })}
      >
        <input data-slot="dropzone-input" {...getInputProps()} />
        {children({ isDragActive })}
      </Card>
    </Button>
  );
}

export { Dropzone };
