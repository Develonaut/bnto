import { writeIfChanged } from "./writeIfChanged";

/** Writes generated files, counting how many changed for the summary. */
export interface Writer {
  write(path: string, content: string): void;
  readonly counts: { written: number; unchanged: number };
}

/** A fresh `Writer` with both counts at zero. */
export function createWriter(): Writer {
  const counts = { written: 0, unchanged: 0 };
  return {
    counts,
    write(path, content) {
      if (writeIfChanged(path, content)) {
        counts.written++;
      } else {
        counts.unchanged++;
      }
    },
  };
}
