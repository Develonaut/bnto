import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShellContent, Container, Divider, StepperContent } from "@bnto/ui";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { BntoHero } from "./_components/BntoHero";
import {
  RecipeStepper,
  RecipeStepperIndicator,
  RecipeStepperDropzone,
  RecipeStepperToolbar,
  RecipeStepperBackButton,
  RecipeStepperConfigButton,
  RecipeStepperBanner,
  RecipeStepperActions,
  RunRecipeButton,
  RecipeStepperResultList,
} from "./_components/RecipeStepper";
import { OpenInEditorLink } from "./_components/OpenInEditorLink";

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
          <RecipeStepper key={slug} entry={entry}>
            <RecipeStepperIndicator />
            <BntoHero h1={entry.h1} description={entry.description} />
            <div>
              <OpenInEditorLink slug={slug} />
            </div>

            <StepperContent value="1">
              <RecipeStepperDropzone />
            </StepperContent>

            <StepperContent value="2">
              <RecipeStepperToolbar>
                <RecipeStepperActions className="shrink-0">
                  <RecipeStepperBackButton />
                </RecipeStepperActions>
                <RecipeStepperBanner />
                <RecipeStepperActions className="ml-auto shrink-0">
                  <RecipeStepperConfigButton />
                  <RunRecipeButton />
                </RecipeStepperActions>
              </RecipeStepperToolbar>
              <Divider />
              <RecipeStepperResultList />
            </StepperContent>

            <StepperContent value="3">
              <RecipeStepperToolbar>
                <RecipeStepperActions className="shrink-0">
                  <RecipeStepperBackButton />
                </RecipeStepperActions>
                <RecipeStepperBanner />
                <RecipeStepperActions className="ml-auto shrink-0">
                  <RecipeStepperConfigButton />
                  <RunRecipeButton />
                </RecipeStepperActions>
              </RecipeStepperToolbar>
              <Divider />
              <RecipeStepperResultList />
            </StepperContent>
          </RecipeStepper>
        </Container>
      </AppShellContent>
    </>
  );
}
