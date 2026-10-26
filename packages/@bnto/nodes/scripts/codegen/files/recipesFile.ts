import { generatedHeader } from "../generatedHeader";
import type { CatalogEnvelope, RawRecipe } from "../types";

const TYPES = `import type { Definition } from "../definition";

/** An operating system a recipe can run on (what \`{{ctx.platform}}\` reads). */
export type RecipeOs = "macos" | "linux" | "windows";

/** A generated recipe from the engine's built-in catalog. */
export interface GeneratedRecipe {
  readonly slug: string;
  readonly name: string;
  readonly description: string;
  readonly category: string;
  readonly tags: readonly string[];
  /** The OSes it has steps for, worked out by the engine from its platform guards. */
  readonly os: readonly RecipeOs[];
  readonly definition: Definition;
}`;

function recipeEntry(r: RawRecipe): string {
  const definition = JSON.stringify(r.definition, null, 4)
    .split("\n")
    .map((line, i) => (i === 0 ? line : "    " + line))
    .join("\n");
  return `  {
    slug: ${JSON.stringify(r.slug)},
    name: ${JSON.stringify(r.name)},
    description: ${JSON.stringify(r.description)},
    category: ${JSON.stringify(r.category)},
    tags: ${JSON.stringify(r.tags ?? [])} as const,
    os: ${JSON.stringify(r.os)} as const,
    definition: ${definition},
  }`;
}

/** `recipes.ts`: the engine's built-in recipes, or null when the catalog has none. */
export function recipesFile(catalog: CatalogEnvelope): string | null {
  const recipes = catalog.recipes;
  if (!recipes || recipes.length === 0) return null;

  return `${generatedHeader(catalog.version)}

${TYPES}

/** All engine-owned built-in recipes. */
export const GENERATED_RECIPES: readonly GeneratedRecipe[] = [
${recipes.map(recipeEntry).join(",\n")},
] as const;
`;
}
