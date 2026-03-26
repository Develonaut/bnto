import type { ChangeEvent } from "react";
import { useCallback, useMemo } from "react";
import type { OptimizeImagesForWebConfig as Config } from "./types";

/** Handlers and derived state for OptimizeImagesForWebConfig inputs. */
export function useOptimizeImagesHandlers(value: Config, onChange: (config: Config) => void) {
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

  const qualityValue = useMemo(() => [value.quality], [value.quality]);

  const handleQualityChange = useCallback(
    ([quality]: number[]) => onChange({ ...value, quality: quality ?? value.quality }),
    [onChange, value],
  );

  return { handleWidthChange, handleFormatChange, qualityValue, handleQualityChange };
}
