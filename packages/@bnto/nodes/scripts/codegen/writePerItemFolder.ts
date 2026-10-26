import { resolve } from "node:path";
import { cleanStaleFiles } from "./cleanStaleFiles";
import type { Writer } from "./createWriter";
import { toCamelCase } from "./toCamelCase";

/** A generated file named after a node type (`image-resize` → `imageResize.ts`). */
export interface PerItemFile {
  name: string;
  content: string;
}

/** One file per node type plus `index.ts`; anything else in the folder is removed. */
export function writePerItemFolder(
  out: Writer,
  dir: string,
  files: PerItemFile[],
  barrel: string,
): void {
  const expected = new Set<string>(["index.ts"]);
  for (const file of files) {
    const filename = `${toCamelCase(file.name)}.ts`;
    expected.add(filename);
    out.write(resolve(dir, filename), file.content);
  }
  out.write(resolve(dir, "index.ts"), barrel);
  cleanStaleFiles(dir, expected);
}
