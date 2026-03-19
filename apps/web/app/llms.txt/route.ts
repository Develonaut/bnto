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
    "> Compress images, clean CSVs, rename files, and convert formats. Free, instant, 100% in your browser. No signup, no upload. Open source.",
    "",
    "## Tools",
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
