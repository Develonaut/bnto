"use client";

import { useState } from "react";

import {
  Button,
  Card,
  FileUpload,
  FileUploadDropzone,
  FileUploadClear,
  FileUploadList,
  FileUploadItem,
  FileUploadItemMetadata,
  FileUploadItemActions,
  FileUploadItemDelete,
  UploadIcon,
  XIcon,
} from "@bnto/ui";

export function FileListShowcase() {
  const [files, setFiles] = useState<File[]>([]);

  return (
    <FileUpload value={files} onValueChange={setFiles} accept={{ "image/*": [] }}>
      {files.length === 0 && (
        <FileUploadDropzone className="gap-3 px-6 py-10">
          <div className="rounded-full bg-muted p-3 text-muted-foreground">
            <UploadIcon className="size-6" />
          </div>
          <div className="text-center">
            <p className="text-sm font-medium text-foreground">Drag & drop files here</p>
            <p className="mt-1 text-xs text-muted-foreground">
              or click to browse &middot; accepts images
            </p>
          </div>
        </FileUploadDropzone>
      )}

      {files.length > 0 && (
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <p className="text-sm font-medium text-foreground">
              {files.length} {files.length === 1 ? "file" : "files"} selected
            </p>
            <FileUploadClear asChild>
              <Button variant="outline" elevation="md">
                Clear all
              </Button>
            </FileUploadClear>
          </div>

          <FileUploadList>
            {files.map((file, i) => (
              <FileUploadItem
                key={`${file.name}-${file.size}-${file.lastModified}`}
                value={file}
                index={i}
              >
                <Card className="flex items-center gap-3 rounded-lg px-4 py-3" elevation="sm">
                  <FileUploadItemMetadata />
                  <FileUploadItemActions>
                    <FileUploadItemDelete asChild>
                      <Button variant="outline" size="icon" elevation="sm">
                        <XIcon className="size-4" />
                      </Button>
                    </FileUploadItemDelete>
                  </FileUploadItemActions>
                </Card>
              </FileUploadItem>
            ))}
          </FileUploadList>
        </div>
      )}
    </FileUpload>
  );
}
