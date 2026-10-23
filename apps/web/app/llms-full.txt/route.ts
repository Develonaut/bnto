import {
  getAllRecipes,
  getBrowserRecipes,
  getCliRunCommand,
  isRecipeBrowserCapable,
  recipeOsLabels,
} from "@bnto/registry";
import type { Recipe } from "@bnto/registry";
import { BASE_URL } from "@/lib/constants";

export const dynamic = "force-static";

function formatAccepts(r: Recipe) {
  return r.accept.extensions.map((e) => e.replace(".", "").toUpperCase()).join(", ");
}

/** Browser recipes list the formats they accept; CLI-only ones, their command. */
function formatInput(r: Recipe) {
  return isRecipeBrowserCapable(r)
    ? `- Accepts: ${formatAccepts(r)}`
    : `- Run: ${getCliRunCommand(r)}`;
}

/** The OSes a CLI-only recipe runs on; a browser recipe runs in any browser. */
function formatOs(r: Recipe) {
  const labels = recipeOsLabels(r.os);
  if (isRecipeBrowserCapable(r) || labels.length === 0) return [];
  return [`- Runs on: ${labels.join(", ")}`];
}

function formatRecipe(r: Recipe, platform: string) {
  return [
    `### ${r.name}`,
    `- URL: ${BASE_URL}/${r.slug}`,
    `- Description: ${r.description}`,
    `- Category: ${r.category}`,
    formatInput(r),
    ...formatOs(r),
    `- Features: ${r.features.join(", ")}`,
    `- Platform: ${platform}`,
    "- Cost: Free",
  ].join("\n");
}

function generateLlmsFullTxt() {
  const browserSections = getBrowserRecipes()
    .map((r) => formatRecipe(r, "Browser"))
    .join("\n\n");

  const cliRecipes = getAllRecipes().filter((r) => !isRecipeBrowserCapable(r));
  const cliSections = cliRecipes.map((r) => formatRecipe(r, "CLI only")).join("\n\n");

  const parts = [
    "# bnto",
    "",
    "> Free tools that run in your browser. Compress images, clean CSVs, rename files, convert formats, and build custom recipes. Powered by Rust & WebAssembly. No signup, no upload. Open source.",
    "",
    "## Browser Recipes",
    "",
    browserSections,
  ];

  if (cliSections) {
    parts.push("", "## CLI-Only Recipes", "", cliSections);
  }

  parts.push("");
  return parts.join("\n");
}

export function GET() {
  return new Response(generateLlmsFullTxt(), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}
