"use client";

import { Stack } from "@bnto/ui";
import type { OutputFileUrl } from "@bnto/core";
import { OutputFileRow } from "./OutputFileRow";

interface OutputFileListProps {
  files: { key: string; name: string; sizeBytes: number }[];
  urls: OutputFileUrl[];
  onDownloadFile: (url: OutputFileUrl) => () => void;
}

/** List of output files with download buttons. */
export function OutputFileList({ files, urls, onDownloadFile }: OutputFileListProps) {
  return (
    <Stack as="ul" className="gap-2">
      {files.map((file) => (
        <OutputFileRow
          key={file.key}
          name={file.name}
          sizeBytes={file.sizeBytes}
          downloadUrl={urls.find((u) => u.key === file.key)}
          onDownload={onDownloadFile}
        />
      ))}
    </Stack>
  );
}
