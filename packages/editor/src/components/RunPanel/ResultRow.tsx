"use client";

import { Button, DownloadIcon, ResultFileCard } from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { useFileResultProps } from "@bnto/core";
import { useEditor } from "../../context";

/** Single result row using the shared ResultFileCard. */
function ResultRow({ result }: { result: BrowserFileResult }) {
  const editor = useEditor();
  const props = useFileResultProps(result);

  return (
    <ResultFileCard
      filename={props.filename}
      extension={props.extension}
      outputSize={props.outputSize}
      originalSize={props.originalSize}
      savings={props.savings}
      action={
        <Button
          variant="outline"
          size="sm"
          icon={<DownloadIcon />}
          onClick={() => editor.execution.downloadResult(result)}
          aria-label={`Download ${result.filename}`}
        />
      }
    />
  );
}

export { ResultRow };
