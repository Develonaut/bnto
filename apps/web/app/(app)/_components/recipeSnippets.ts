import { getRecipeBySlug } from "@bnto/registry";

import type { TerminalLine } from "@/components/blocks/Terminal";

/* ── Syntax-coloring class names ─────────────────────────────── */

const key = "text-primary";
const str = "text-secondary-foreground";
const num = "text-accent-foreground";

/* ── Types ────────────────────────────────────────────────────── */

export interface RecipeSnippet {
  id: string;
  label: string;
  filename: string;
  lines: TerminalLine[];
  mascot: string;
}

/* ── Helpers ──────────────────────────────────────────────────── */

function ln(text: string, opts?: Partial<TerminalLine>): TerminalLine {
  return { text, delay: 80, ...opts };
}

/** Verbose params to strip — noise that doesn't help the reader. */
const STRIP_PARAMS = new Set(["extensions", "label", "multiple", "zip", "autoDownload"]);

/** Keep only the meaningful parameters for display. */
function pickDisplayParams(params: Record<string, unknown>): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(params)) {
    if (!STRIP_PARAMS.has(k)) result[k] = v;
  }
  return result;
}

/** Color a value based on its JS type. */
function valueColor(v: unknown): string {
  if (typeof v === "number" || typeof v === "boolean") return num;
  return str;
}

/**
 * Build TerminalLine[] from a real recipe definition.
 *
 * Pretty-prints each node across multiple lines — one line per param —
 * so the JSON fits comfortably in a fixed-width editor.
 */
function buildLines(slug: string): TerminalLine[] {
  const recipe = getRecipeBySlug(slug);
  if (!recipe) throw new Error(`Recipe "${slug}" not found in registry`);

  const { definition } = recipe;
  const nodes = definition.nodes ?? [];

  const result: TerminalLine[] = [
    ln("{", { typing: true, speed: 30, delay: 500 }),
    ln(`  "id": "${definition.id}",`, { className: key }),
    ln(`  "name": "${definition.name}",`, { className: str }),
    ln('  "nodes": [', { className: key }),
  ];

  nodes.forEach((node, i) => {
    const isLast = i === nodes.length - 1;
    const params = pickDisplayParams(node.parameters);
    const entries = Object.entries(params);

    result.push(ln("    {"));
    result.push(ln(`      "type": "${node.type}",`, { className: str }));

    entries.forEach(([k, v], j) => {
      const comma = j < entries.length - 1 ? "," : "";
      result.push(ln(`      "${k}": ${JSON.stringify(v)}${comma}`, { className: valueColor(v) }));
    });

    result.push(ln(`    }${isLast ? "" : ","}`));
  });

  result.push(ln("  ]"));
  result.push(ln("}"));

  return result;
}

/* ── Showcase recipes ─────────────────────────────────────────── */

const SHOWCASE_SLUGS = [
  { slug: "compress-images", label: "Compress Images", mascot: "/mascots/salmon-chopstick.svg" },
  { slug: "csv-to-json", label: "CSV to JSON", mascot: "/mascots/sushi-onigiri.svg" },
  { slug: "rename-files", label: "Rename Files", mascot: "/mascots/sushi-shoyu.svg" },
  {
    slug: "optimize-images-for-web",
    label: "Optimize for Web",
    mascot: "/mascots/sushi-shoyu-alt.svg",
  },
] as const;

export const RECIPE_SNIPPETS: RecipeSnippet[] = SHOWCASE_SLUGS.map(({ slug, label, mascot }) => ({
  id: slug,
  label,
  filename: `${slug}.bnto.json`,
  lines: buildLines(slug),
  mascot,
}));
