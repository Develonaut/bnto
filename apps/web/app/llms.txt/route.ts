import { getBrowserRecipes, getAllRecipes, isRecipeBrowserCapable } from "@bnto/registry";
import { BASE_URL } from "@/lib/constants";

export const dynamic = "force-static";

function generateLlmsTxt() {
  const browserTools = getBrowserRecipes()
    .map((r) => `- [${r.name}](${BASE_URL}/${r.slug}): ${r.description}`)
    .join("\n");

  const cliTools = getAllRecipes()
    .filter((r) => !isRecipeBrowserCapable(r))
    .map((r) => `- [${r.name}](${BASE_URL}/${r.slug}): ${r.description} (CLI only)`)
    .join("\n");

  const sections = [
    "# bnto",
    "",
    "> Free tools that run in your browser. Compress images, clean CSVs, rename files, convert formats, and build custom recipes. Powered by Rust & WebAssembly. No signup, no upload. Open source.",
    "",
    "## Browser Recipes",
    "",
    browserTools,
  ];

  if (cliTools) {
    sections.push("", "## CLI-Only Recipes", "", cliTools);
  }

  sections.push("");
  return sections.join("\n");
}

export function GET() {
  return new Response(generateLlmsTxt(), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}
