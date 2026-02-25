"use client";

import { useState, useCallback } from "react";
import { FileIcon, TrashIcon, UploadIcon } from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
import { Dropzone } from "@/components/ui/Dropzone";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/tabs";
import { Text } from "@/components/ui/Text";

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

interface FileListShowcaseProps {
  /** Pre-populate with files — skips the dropzone and shows the list directly. */
  initialFiles?: File[];
}

export function FileListShowcase({ initialFiles }: FileListShowcaseProps) {
  const hasInitial = initialFiles && initialFiles.length > 0;
  const [files, setFiles] = useState<File[]>(hasInitial ? initialFiles : []);
  const [tab, setTab] = useState(hasInitial ? "list" : "dropzone");

  const onDrop = useCallback((accepted: File[]) => {
    setFiles(accepted);
    if (accepted.length > 0) setTab("list");
  }, []);

  const handleClear = useCallback(() => {
    setFiles([]);
    setTab("dropzone");
  }, []);

  return (
    <Tabs value={tab} onValueChange={setTab}>
      {/* Tabs.List is hidden — state drives the view, not user tab clicks */}
      <Tabs.List className="sr-only">
        <Tabs.Trigger value="dropzone">Drop</Tabs.Trigger>
        <Tabs.Trigger value="list">List</Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="dropzone">
        <Dropzone accept={{ "image/*": [] }} onDrop={onDrop} multiple>
          {({ isDragActive }) => (
            <Stack gap="sm" align="center" className="text-muted-foreground">
              <UploadIcon className="size-8" />
              <Text size="sm" weight="medium" color="inherit" as="span">
                {isDragActive ? "Drop files here" : "Drag images here or click to browse"}
              </Text>
              <Text size="xs" color="inherit" as="span">PNG, JPEG, WebP up to 10MB</Text>
            </Stack>
          )}
        </Dropzone>
      </Tabs.Content>

      <Tabs.Content value="list">
        <Stack className="gap-3">
          <div className="flex justify-end">
            <Button variant="outline" size="sm" onClick={handleClear}>
              <TrashIcon />
              Clear
            </Button>
          </div>

          <Animate.Stagger interval={60} className="flex flex-col gap-3">
            {files.map((file, i) => (
              <Animate.SlideUp key={file.name} index={i} easing="spring-bouncier">
                <Card className="flex items-center gap-4 p-4">
                  <div className="bg-primary/10 text-primary flex size-10 items-center justify-center rounded-full">
                    <FileIcon className="size-5" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <Text size="sm" weight="semibold" as="span" className="truncate">
                      {file.name}
                    </Text>
                    <Text size="xs" color="muted">
                      {file.type || "unknown"} &middot; {formatFileSize(file.size)}
                    </Text>
                  </div>
                  <Text size="xs" color="muted" mono as="span">
                    {formatFileSize(file.size)}
                  </Text>
                </Card>
              </Animate.SlideUp>
            ))}
          </Animate.Stagger>
        </Stack>
      </Tabs.Content>
    </Tabs>
  );
}
