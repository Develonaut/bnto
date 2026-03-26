import type { ChangeEvent } from "react";
import { useCallback } from "react";
import type { GenerateThumbnailsConfig as Config } from "./types";

/** Handlers for GenerateThumbnailsConfig inputs. */
export function useGenerateThumbnailsHandlers(value: Config, onChange: (config: Config) => void) {
  const handleWidthChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const width = parseInt(e.target.value, 10);
      if (!isNaN(width) && width > 0) {
        onChange({ ...value, width });
      }
    },
    [onChange, value],
  );

  const handleFormatChange = useCallback(
    (format: string) => onChange({ ...value, format: format as Config["format"] }),
    [onChange, value],
  );

  const handlePrefixChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => onChange({ ...value, prefix: e.target.value }),
    [onChange, value],
  );

  return { handleWidthChange, handleFormatChange, handlePrefixChange };
}
