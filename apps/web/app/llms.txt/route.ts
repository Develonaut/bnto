import { getAllRecipes } from "@bnto/registry";
import { BASE_URL } from "@/lib/constants";

export const dynamic = "force-static";

function generateLlmsTxt() {
  const tools = getAllRecipes()
    .map((r) => `- [${r.name}](${BASE_URL}/${r.slug}): ${r.description}`)
    .join("\n");

  return [
    "# bnto",
    "",
    "> Free tools that run in your browser. Compress images, clean CSVs, rename files, convert formats, and build custom recipes. Powered by Rust & WebAssembly. No signup, no upload. Open source.",
    "",
    "## Recipes",
    "",
    tools,
    "",
  ].join("\n");
}

export function GET() {
  return new Response(generateLlmsTxt(), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}
