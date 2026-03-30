"use client";

import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { BouncyStagger, Grid } from "@bnto/ui";
import { RecipeFileGridItem } from "./RecipeFileGridItem";

interface RecipeFileGridProps {
  files: File[];
  activeStep: 1 | 2 | 3;
  isBrowserPath: boolean;
  browserExec: BrowserExecution;
  onDeleteFile: (index: number) => () => void;
  onDownload: (result: BrowserFileResult) => void;
}

/**
 * Persistent file grid shown in Steps 2 and 3.
 *
 * Columns adapt to file count: 1 file = full width,
 * 2 files = 2 cols on md+, 3+ files = 2 cols on md / 3 cols on lg.
 */
export function RecipeFileGrid(props: RecipeFileGridProps) {
  const { files, activeStep, isBrowserPath, browserExec, onDeleteFile, onDownload } = props;

  return (
    <BouncyStagger asChild>
      <Grid
        cols={1}
        gap="sm"
        role="list"
        aria-label="Selected files"
        className={gridColsClass(files.length)}
      >
        {files.map((file, i) => (
          <RecipeFileGridItem
            key={`${file.name}-${file.size}-${file.lastModified}`}
            file={file}
            index={i}
            activeStep={activeStep}
            isBrowserPath={isBrowserPath}
            browserExec={browserExec}
            onDelete={onDeleteFile(i)}
            onDownload={onDownload}
          />
        ))}
      </Grid>
    </BouncyStagger>
  );
}

/** Derive responsive grid column classes from file count. */
const gridColsClass = (count: number) => {
  if (count >= 3) return "md:grid-cols-2 lg:grid-cols-3";
  if (count === 2) return "md:grid-cols-2";
  return;
};
