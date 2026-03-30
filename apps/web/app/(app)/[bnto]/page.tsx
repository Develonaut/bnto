import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { Heading } from "@bnto/ui";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { RecipeShell } from "./_components/RecipeShell";

/** Only slugs from generateStaticParams are valid — everything else is 404
 * at the routing level (no component code runs for unknown slugs). */
export const dynamicParams = false;

/** Pre-render all registered slugs at build time. */
export function generateStaticParams() {
  return BNTO_REGISTRY.map((bnto) => ({ bnto: bnto.slug }));
}

/** Per-slug metadata -- resolved at build time for static pages. */
export async function generateMetadata({
  params,
}: {
  params: Promise<{ bnto: string }>;
}): Promise<Metadata> {
  const { bnto } = await params;
  const entry = getBntoBySlug(bnto);
  if (!entry) return {};
  return {
    title: { absolute: entry.title },
    description: entry.description,
    openGraph: {
      title: entry.title,
      description: entry.description,
    },
  };
}

export default async function BntoPage({ params }: { params: Promise<{ bnto: string }> }) {
  const { bnto: slug } = await params;
  const entry = getBntoBySlug(slug);
  if (!entry) notFound();

  return (
    <>
      <BntoJsonLd entry={entry} />
      <div className="min-h-[80svh] flex-1 py-12">
        <div className="mx-auto grid min-h-[70svh] max-w-[1800px] grid-cols-1 items-center gap-10 px-6 lg:grid-cols-[2fr_3fr] lg:gap-16 lg:pl-[max(2rem,calc((100vw-1220px)/2))] lg:pr-24">
          {/* Left column — static header, server-rendered */}
          <div className="space-y-3 text-left">
            <Heading level={1} data-testid="recipe-heading">
              {entry.h1}
            </Heading>
            <p className="text-muted-foreground leading-snug text-balance">{entry.description}</p>
          </div>

          {/* Right column — interactive recipe flow */}
          <RecipeShell key={slug} entry={entry} />
        </div>
      </div>
    </>
  );
}
