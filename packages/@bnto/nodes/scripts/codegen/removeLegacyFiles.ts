import { rmSync } from "node:fs";
import { LEGACY_FILES } from "./paths";

/** Remove the monolithic files the per-file layout replaced, when they're still there. */
export function removeLegacyFiles(): void {
  for (const legacy of LEGACY_FILES) {
    try {
      rmSync(legacy);
      console.log(`Removed legacy ${legacy}`);
    } catch {
      // Doesn't exist, that's fine
    }
  }
}
