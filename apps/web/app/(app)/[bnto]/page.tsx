import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShellContent, Container } from "@bnto/ui";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { RecipeHeader } from "./_components/RecipeHeader";
import { RecipeShell } from "./_components/RecipeShell";
import { BentoRecipeShell } from "./_components/bento/BentoRecipeShell";

const useBentoGrid = process.env.NEXT_PUBLIC_BENTO_GRID === "1";

export const dynamicParams = false;

export function generateStaticParams() {
  return BNTO_REGISTRY.map((bnto) => ({ bnto: bnto.slug }));
}

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

  return (
    <>
      <BntoJsonLd entry={entry} />
      <AppShellContent>
        {useBentoGrid ? (
          <Container size="lg" className="space-y-6">
            <RecipeHeader entry={entry} />
            <BentoRecipeShell key={slug} entry={entry} />
          </Container>
        ) : (
          <Container size="md" className="space-y-6 text-center">
            <RecipeShell key={slug} entry={entry}>
              <RecipeHeader entry={entry} />
            </RecipeShell>
          </Container>
        )}
      </AppShellContent>
    </>
  );
}
