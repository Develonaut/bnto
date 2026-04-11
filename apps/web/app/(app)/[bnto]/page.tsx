import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShellContent, Container } from "@bnto/ui";
import { getRecipeBySlug, isRecipeBrowserCapable } from "@bnto/registry";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { BntoHero } from "./_components/BntoHero";
import { BntoRunStepper } from "./_components/BntoRunStepper";
import { RecipeHeroMascot } from "./_components/RecipeHeroMascot";
import { CliPromo } from "./_components/CliPromo";

/** Only slugs from generateStaticParams are valid — everything else is 404. */
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
    openGraph: { title: entry.title, description: entry.description },
  };
}

export default async function BntoPage({ params }: { params: Promise<{ bnto: string }> }) {
  const { bnto: slug } = await params;
  const entry = getBntoBySlug(slug);
  if (!entry) notFound();

  const recipe = getRecipeBySlug(slug);
  const browserCapable = recipe ? isRecipeBrowserCapable(recipe) : true;

  return (
    <>
      <BntoJsonLd entry={entry} />
      <AppShellContent>
        <Container size="md" className="space-y-6 text-center">
          {browserCapable ? (
            <BntoRunStepper entry={entry} />
          ) : (
            <>
              <RecipeHeroMascot category={entry.category} />
              <BntoHero h1={entry.h1} description={entry.description} />
              <CliPromo slug={slug} />
            </>
          )}
        </Container>
      </AppShellContent>
    </>
  );
}
