import { useMemo, useCallback } from "react";
import type { BrowserFileResult } from "../types/browser";
import { core } from "../core";
import { deriveFileResultProps } from "../utils/deriveFileResultProps";
import type { FileResultDisplay } from "../utils/deriveFileResultProps";

interface FileResultDisplayProps extends FileResultDisplay {
  download: () => void;
}

/**
 * Derives display props from a BrowserFileResult for use with FileList components.
 *
 * Wraps the pure `deriveFileResultProps` utility with a memoized download callback.
 */
function useFileResultProps(result: BrowserFileResult): FileResultDisplayProps {
  const download = useCallback(() => {
    core.executions.downloadResult(result);
  }, [result]);

  return useMemo(() => {
    const derived = deriveFileResultProps(result);
    return { ...derived, download };
  }, [result, download]);
}

export { useFileResultProps };
export type { FileResultDisplayProps };
