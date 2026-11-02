/**
 * The units a cargo build is made of, from its `--message-format=json`
 * output (ISS-120). Cargo reports every artifact and build script run, fresh
 * ones too, so a no-op rebuild lists the whole build without compiling anything.
 */

import { unitKey } from "./unitKey.ts";

interface CargoMessage {
  reason?: string;
  filenames?: string[];
  out_dir?: string;
}

function parse(line: string): CargoMessage | null {
  try {
    return JSON.parse(line) as CargoMessage;
  } catch {
    return null;
  }
}

// A build script's files have no hash of their own: their folder carries it
// (`build/x-<hash>/build-script-build`, `build/x-<hash>/out`).
function keyOf(file: string): string | null {
  const parts = file.split(/[\\/]/);
  return unitKey(parts[parts.length - 1]) ?? unitKey(parts[parts.length - 2] ?? "");
}

function filesOf(message: CargoMessage | null): string[] {
  if (message?.reason === "compiler-artifact") return message.filenames ?? [];
  if (message?.reason === "build-script-executed" && message.out_dir) return [message.out_dir];
  return [];
}

/** The unit hash of every artifact and build script run in the output. */
export function artifactKeys(output: string): Set<string> {
  const files = output.split("\n").map(parse).flatMap(filesOf);
  return new Set(files.map(keyOf).filter((key): key is string => key !== null));
}
