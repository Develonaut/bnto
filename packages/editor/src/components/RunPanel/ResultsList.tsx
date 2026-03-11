"use client";

import { useEditor } from "../../context";
import { ResultRow } from "./ResultRow";

/** File result cards — only renders when there are results. */
function ResultsList() {
  const editor = useEditor();
  const { results } = editor.execution.useExecution();
  if (results.length === 0) return null;

  return (
    <div className="flex flex-col gap-1.5 p-2">
      {results.map((file, i) => (
        <ResultRow key={`${file.filename}-${i}`} result={file} />
      ))}
    </div>
  );
}

export { ResultsList };
