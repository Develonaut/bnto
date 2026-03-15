import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShellContent, Container, Heading } from "@bnto/ui";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { OpenInEditorLink } from "./_components/OpenInEditorLink";
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
      <AppShellContent>
        <Container size="md" className="space-y-6 text-center">
          {/* Interactive recipe flow — PhaseIndicator is at the top, then
              static header content, then the file upload / execution flow */}
          <RecipeShell key={slug} entry={entry}>
            {/* Static header — server-rendered, zero JS.
                Passed as children so it renders between PhaseIndicator
                and the interactive flow inside RecipeShell. */}
            <Heading level={1}>{entry.h1}</Heading>
            <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
              {entry.description}
            </p>
            <div>
              <OpenInEditorLink slug={entry.slug} />
            </div>
          </RecipeShell>
        </Container>
      </AppShellContent>
    </>
  );
}
