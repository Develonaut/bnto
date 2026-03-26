import { useCallback, useMemo } from "react";
import type { ConvertFormatConfig as Config } from "./types";

/** Handlers and derived state for ConvertFormatConfig inputs. */
export function useConvertFormatHandlers(value: Config, onChange: (config: Config) => void) {
  const handleFormatChange = useCallback(
    (format: string) => onChange({ ...value, format: format as Config["format"] }),
    [onChange, value],
  );

  const qualityValue = useMemo(() => [value.quality], [value.quality]);

  const handleQualityChange = useCallback(
    ([quality]: number[]) => onChange({ ...value, quality: quality ?? value.quality }),
    [onChange, value],
  );

  return { handleFormatChange, qualityValue, handleQualityChange };
}
