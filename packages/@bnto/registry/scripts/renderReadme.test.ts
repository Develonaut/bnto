import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { RECIPES } from "../src/recipesCatalog";
import { renderReadme } from "./renderReadme";

const README_PATH = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../README.md");

const TEMPLATE = [
  "# Demo",
  "",
  "<!-- BEGIN AUTO-GENERATED RECIPES TABLE -->",
  "<!-- END AUTO-GENERATED RECIPES TABLE -->",
  "",
  "**Today:** <!-- recipe-count -->0<!-- /recipe-count --> predefined recipes.",
  "",
].join("\n");

describe("renderReadme", () => {
  it("fills in the recipe count", async () => {
    const out = await renderReadme(TEMPLATE, RECIPES.slice(0, 2), README_PATH);
    expect(out).toContain("**Today:** <!-- recipe-count -->2<!-- /recipe-count --> predefined");
  });

  it("lists every recipe in the table", async () => {
    const out = await renderReadme(TEMPLATE, RECIPES, README_PATH);
    for (const recipe of RECIPES) expect(out).toContain(`| ${recipe.name} `);
  });

  it("shows a CLI-only recipe's command instead of a bnto.io link", async () => {
    const out = await renderReadme(TEMPLATE, RECIPES, README_PATH);
    expect(out).toContain("`bnto run keep-awake`");
    expect(out).not.toContain("bnto.io/keep-awake");
    expect(out).toContain("[bnto.io/compress-images](https://bnto.io/compress-images)");
  });

  it("refuses a README without the markers", async () => {
    await expect(renderReadme("# Demo\n", RECIPES, README_PATH)).rejects.toThrow("recipe-count");
  });
});

describe("README.md", () => {
  it("matches the registry (run `task readme:generate` if this fails)", async () => {
    const readme = readFileSync(README_PATH, "utf-8");
    expect(await renderReadme(readme, RECIPES, README_PATH)).toBe(readme);
  });
});
