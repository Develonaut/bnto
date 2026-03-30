import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShellContent, Container, StepperIndicator } from "@bnto/ui";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { BntoHero } from "./_components/BntoHero";
import { BntoToolbar } from "./_components/BntoToolbar";
import { SessionMarker } from "./_components/SessionMarker";
import {
  RecipeFlow,
  RecipeFlowStepper,
  RecipeFlowFileUpload,
  RecipeFlowDropzone,
  RecipeFlowResults,
  RecipeFilesGrid,
} from "./_components/RecipeFlow";

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

  return (
    <>
      <BntoJsonLd entry={entry} />
      <AppShellContent>
        <Container size="md" className="space-y-6 text-center">
          <RecipeFlow key={slug} entry={entry}>
            <RecipeFlowStepper>
              <RecipeFlowFileUpload>
                <SessionMarker />
                <StepperIndicator />
                <BntoHero h1={entry.h1} description={entry.description} />
                <RecipeFlowDropzone />
                <BntoToolbar />
                <RecipeFlowResults />
                <RecipeFilesGrid />
              </RecipeFlowFileUpload>
            </RecipeFlowStepper>
          </RecipeFlow>
        </Container>
      </AppShellContent>
    </>
  );
}
