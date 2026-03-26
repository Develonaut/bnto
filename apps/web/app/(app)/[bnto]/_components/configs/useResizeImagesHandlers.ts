import type { ChangeEvent } from "react";
import { useCallback } from "react";
import type { ResizeImagesConfig as Config } from "./types";

/** Handlers for ResizeImagesConfig inputs. */
export function useResizeImagesHandlers(value: Config, onChange: (config: Config) => void) {
  const handleWidthChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const width = parseInt(e.target.value, 10);
      if (!isNaN(width) && width > 0) {
        onChange({ ...value, width });
      }
    },
    [onChange, value],
  );

  const handleAspectRatioChange = useCallback(
    (maintainAspectRatio: boolean) =>
      onChange({ ...value, maintainAspectRatio: !!maintainAspectRatio }),
    [onChange, value],
  );

  return { handleWidthChange, handleAspectRatioChange };
}
