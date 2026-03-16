"use client";

import { useCallback } from "react";
import type { BrowserFileResult } from "@bnto/core";
import { Button, Card, CheckCircle2Icon, DownloadIcon, IconBadge, Row, Stack } from "@bnto/ui";
import { formatFileSize } from "@bnto/ui";

interface OutputFileCardProps {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}

function OutputFileCard({ result, onDownload }: OutputFileCardProps) {
  const { originalSize, compressionRatio } = result.metadata;
  const handleDownload = useCallback(() => onDownload(result), [onDownload, result]);

  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
      data-testid="output-file"
    >
      <Row className="min-w-0 flex-1 gap-3">
        <IconBadge variant="primary" size="lg">
          <CheckCircle2Icon className="size-5" />
        </IconBadge>
        <Stack className="min-w-0 flex-1 gap-0">
          <span className="truncate text-sm font-semibold">{result.filename}</span>
          <span className="truncate text-xs text-muted-foreground">
            {formatFileSize(result.blob.size)}
            {originalSize != null && compressionRatio != null && (
              <span> &middot; {Math.round(compressionRatio)}% smaller</span>
            )}
          </span>
        </Stack>
      </Row>

      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        onClick={handleDownload}
        aria-label={`Download ${result.filename}`}
        data-testid="download-button"
      >
        <DownloadIcon className="size-4" />
      </Button>
    </Card>
  );
}

export { OutputFileCard };
