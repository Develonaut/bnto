"use client";

import { Badge, FileListMeta, FileListName } from "@bnto/ui";

interface CompletedFileMetaProps {
  filename: string;
  extension: string | null;
  originalSize: string | undefined;
  savings: string | undefined;
  outputSize: string;
}

/** Name with extension badge, and size stats with savings. */
export function CompletedFileMeta({
  filename,
  extension,
  originalSize,
  savings,
  outputSize,
}: CompletedFileMetaProps) {
  return (
    <>
      <span className="flex items-center gap-1.5">
        <FileListName>{filename}</FileListName>
        {extension && (
          <Badge variant="outline" size="sm" className="shrink-0 uppercase">
            {extension}
          </Badge>
        )}
      </span>
      <FileListMeta>
        <SizeDisplay originalSize={originalSize} savings={savings} outputSize={outputSize} />
      </FileListMeta>
    </>
  );
}

export function SizeDisplay({
  originalSize,
  savings,
  outputSize,
}: {
  originalSize: string | undefined;
  savings: string | undefined;
  outputSize: string;
}) {
  if (originalSize != null && savings != null) {
    return (
      <>
        <span className="line-through">{originalSize}</span>{" "}
        <span className="font-semibold text-primary">{savings}</span> {outputSize}
      </>
    );
  }
  return <>{outputSize}</>;
}
