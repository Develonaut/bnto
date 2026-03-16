import { RECIPES } from "@bnto/nodes";
import { BASE_URL } from "@/lib/constants";

export const dynamic = "force-static";

function formatAccepts(r: (typeof RECIPES)[number]) {
  return r.accept.extensions.map((e) => e.replace(".", "").toUpperCase()).join(", ");
}

function generateLlmsFullTxt() {
  const sections = RECIPES.map((r) =>
    [
      `### ${r.name}`,
      `- URL: ${BASE_URL}/${r.slug}`,
      `- Description: ${r.description}`,
      `- Category: ${r.category}`,
      `- Accepts: ${formatAccepts(r)}`,
      `- Features: ${r.features.join(", ")}`,
      "- Cost: Free",
    ].join("\n"),
  ).join("\n\n");

  return [
    "# bnto",
    "",
    "> Compress images, clean CSVs, rename files, and convert formats. Free, instant, 100% in your browser. No signup, no upload. Open source.",
    "",
    "## Tools",
    "",
    sections,
    "",
  ].join("\n");
}

export function GET() {
  return new Response(generateLlmsFullTxt(), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}
