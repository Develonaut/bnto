import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { DOCS_DIR } from "../paths";
import type { CatalogEnvelope, RawProcessor } from "../types";
import { nodeReadme } from "./nodeReadme";

/** Write `docs/<type>.md` for every node type. */
export function writeNodeReadmes(catalog: CatalogEnvelope): void {
  mkdirSync(DOCS_DIR, { recursive: true });

  const processorByType = new Map<string, RawProcessor>();
  for (const p of catalog.processors) {
    processorByType.set(p.nodeType, p);
  }

  for (const nodeType of catalog.nodeTypes) {
    const readme = nodeReadme(nodeType, processorByType.get(nodeType.name), catalog.version);
    writeFileSync(resolve(DOCS_DIR, `${nodeType.name}.md`), readme, "utf-8");
  }

  console.log(`Generated ${catalog.nodeTypes.length} node READMEs in ${DOCS_DIR}`);
}
