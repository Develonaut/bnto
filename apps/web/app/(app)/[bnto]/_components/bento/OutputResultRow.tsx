"use client";

import { useCallback } from "react";
import { Text, Row, Button, DownloadIcon } from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";

interface OutputResultRowProps {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}

/** Single result file row with download button. */
export function OutputResultRow({ result, onDownload }: OutputResultRowProps) {
  const handleDownload = useCallback(() => onDownload(result), [onDownload, result]);

  return (
    <Row className="items-center gap-2">
      <Text size="sm" className="flex-1 truncate">
        {result.filename}
      </Text>
      <Button size="sm" variant="ghost" onClick={handleDownload}>
        <DownloadIcon className="size-3.5" />
      </Button>
    </Row>
  );
}
