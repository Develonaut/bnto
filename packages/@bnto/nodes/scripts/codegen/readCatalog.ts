import { readFileSync } from "node:fs";
import { CATALOG_PATH } from "./paths";
import type { CatalogEnvelope } from "./types";

/** Parse `engine/catalog.snapshot.json`. */
export function readCatalog(): CatalogEnvelope {
  return JSON.parse(readFileSync(CATALOG_PATH, "utf-8"));
}
