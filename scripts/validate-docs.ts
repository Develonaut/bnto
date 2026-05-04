// validate-docs.ts — documentation completeness gate.
//
// Reads engine/catalog.snapshot.json and verifies:
// 1. Every workspace crate has a README.md
// 2. Processor-owning crate READMEs mention all their processors
// 3. bnto-engine README mentions all registered processors
// 4. Every processor node type has a doc file in packages/@bnto/nodes/docs/

import { readFileSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, "..");

// --- Catalog snapshot types ---

interface NodeType {
  name: string;
  label: string;
  category: string;
}

interface CatalogSnapshot {
  nodeTypes: NodeType[];
}

// --- Category-to-crate mapping ---
// Only categories that own processor crates. Control, io, data, and network
// node types are metadata-only in bnto-core (no dedicated processor crate).

const CATEGORY_TO_CRATE: Record<string, string> = {
  image: "bnto-image",
  spreadsheet: "bnto-spreadsheet",
  file: "bnto-file",
  vector: "bnto-vector",
  system: "bnto-shell",
};

// Categories that don't have dedicated processor crates
const SKIP_CATEGORIES = new Set([
  "control",
  "io",
  "data",
  "network",
  "transform",
]);

// --- Helpers ---

function readCatalog(): CatalogSnapshot {
  const path = resolve(ROOT, "engine/catalog.snapshot.json");
  if (!existsSync(path)) {
    console.error(`ERROR: catalog snapshot not found at ${path}`);
    console.error("       Run 'task wasm:snapshot' to generate it.");
    process.exit(1);
  }
  return JSON.parse(readFileSync(path, "utf-8"));
}

function readWorkspaceMembers(): string[] {
  const cargoToml = readFileSync(
    resolve(ROOT, "engine/Cargo.toml"),
    "utf-8",
  );
  const members: string[] = [];
  const memberRegex = /"crates\/([^"]+)"/g;
  let match;
  while ((match = memberRegex.exec(cargoToml)) !== null) {
    members.push(match[1]);
  }
  return members;
}

function getProcessorNodeTypes(catalog: CatalogSnapshot): NodeType[] {
  return catalog.nodeTypes.filter((nt) => !SKIP_CATEGORIES.has(nt.category));
}

// --- Checks ---

function checkCrateReadmes(members: string[]): string[] {
  const errors: string[] = [];
  for (const crate of members) {
    const readmePath = resolve(ROOT, `engine/crates/${crate}/README.md`);
    if (!existsSync(readmePath)) {
      errors.push(
        `ERROR: engine/crates/${crate}/README.md does not exist.\n` +
          `       Every workspace crate must have a README.`,
      );
    }
  }
  return errors;
}

function checkProcessorCoverage(
  processorTypes: NodeType[],
): string[] {
  const errors: string[] = [];

  // Group processors by crate
  const crateToProcTypes = new Map<string, string[]>();
  for (const nt of processorTypes) {
    const crate = CATEGORY_TO_CRATE[nt.category];
    if (!crate) continue;
    const existing = crateToProcTypes.get(crate) ?? [];
    existing.push(nt.name);
    crateToProcTypes.set(crate, existing);
  }

  // Check each crate README mentions all its processors
  for (const [crate, nodeTypes] of crateToProcTypes) {
    const readmePath = resolve(ROOT, `engine/crates/${crate}/README.md`);
    if (!existsSync(readmePath)) continue; // already reported by checkCrateReadmes

    const readme = readFileSync(readmePath, "utf-8");
    const missing = nodeTypes.filter((nt) => !readme.includes(nt));
    if (missing.length > 0) {
      errors.push(
        `ERROR: engine/crates/${crate}/README.md is missing processors: ${missing.join(", ")}\n` +
          `       Add them to the Processors table.`,
      );
    }
  }

  return errors;
}

function checkEngineRegistry(processorTypes: NodeType[]): string[] {
  const readmePath = resolve(
    ROOT,
    "engine/crates/bnto-engine/README.md",
  );
  if (!existsSync(readmePath)) return []; // already reported

  const readme = readFileSync(readmePath, "utf-8");
  const missing = processorTypes
    .map((nt) => nt.name)
    .filter((name) => !readme.includes(name));

  if (missing.length > 0) {
    return [
      `ERROR: engine/crates/bnto-engine/README.md is missing processors: ${missing.join(", ")}\n` +
        `       The engine README must list all registered processors.`,
    ];
  }
  return [];
}

function checkNodeDocs(catalog: CatalogSnapshot): string[] {
  const errors: string[] = [];
  const docsDir = resolve(ROOT, "packages/@bnto/nodes/docs");

  // Check processor node types only (skip control/io/data/network — those are
  // also checked but separately since they have docs too)
  for (const nt of catalog.nodeTypes) {
    const docPath = resolve(docsDir, `${nt.name}.md`);
    if (!existsSync(docPath)) {
      errors.push(
        `ERROR: packages/@bnto/nodes/docs/${nt.name}.md does not exist.\n` +
          `       Every node type needs a documentation file.`,
      );
    }
  }

  return errors;
}

// --- Main ---

function main(): void {
  const catalog = readCatalog();
  const members = readWorkspaceMembers();
  const processorTypes = getProcessorNodeTypes(catalog);

  const allErrors = [
    ...checkCrateReadmes(members),
    ...checkProcessorCoverage(processorTypes),
    ...checkEngineRegistry(processorTypes),
    ...checkNodeDocs(catalog),
  ];

  if (allErrors.length > 0) {
    console.error("\nDocumentation validation failed:\n");
    for (const err of allErrors) {
      console.error(err);
      console.error();
    }
    console.error(`${allErrors.length} error(s) found.`);
    process.exit(1);
  }

  console.log("Documentation validation passed.");
}

main();
