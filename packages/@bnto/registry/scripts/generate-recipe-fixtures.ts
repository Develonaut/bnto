/**
 * Codegen: TypeScript recipes → JSON fixtures.
 *
 * Reads all predefined recipes from `@bnto/registry` and serializes their
 * definitions to `.bnto.json` files in `src/recipes/generated/`.
 * Test-only recipes go into `src/recipes/generated/testing/`.
 *
 * This ensures CLI, WASM, and E2E tests all consume the same fixtures
 * generated from the single source of truth (TypeScript recipe definitions).
 *
 * Run via: `task recipes:generate`
 */

import { writeFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import type { Recipe } from "../src/recipe";

// --- Paths ---

const SCRIPT_DIR = dirname(import.meta.url.replace("file://", ""));
const GENERATED_DIR = resolve(SCRIPT_DIR, "../src/recipes/generated");
const TESTING_DIR = resolve(GENERATED_DIR, "testing");

// --- Import recipes ---

// Predefined (public) recipes
import { RECIPES } from "../src/recipesCatalog";

// Test-only recipes
import { allOperations } from "../src/recipes/testing/index";

const TEST_RECIPES: readonly Recipe[] = [allOperations];

// --- Generate ---

function writeFixture(recipe: Recipe, dir: string): void {
  const filename = `${recipe.slug}.bnto.json`;
  const filepath = resolve(dir, filename);
  const json = JSON.stringify(recipe.definition, null, 2) + "\n";
  writeFileSync(filepath, json, "utf-8");
  console.log(`  ${filepath}`);
}

console.log(`Generating ${RECIPES.length} predefined recipe fixtures:`);
for (const recipe of RECIPES) {
  writeFixture(recipe, GENERATED_DIR);
}

console.log(`\nGenerating ${TEST_RECIPES.length} test-only recipe fixtures:`);
for (const recipe of TEST_RECIPES) {
  writeFixture(recipe, TESTING_DIR);
}

console.log(`\nDone — ${RECIPES.length + TEST_RECIPES.length} fixtures written.`);
