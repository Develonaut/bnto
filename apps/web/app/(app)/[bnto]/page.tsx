import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShellContent, Container, Divider, Heading } from "@bnto/ui";
import { getRecipeBySlug, isRecipeBrowserCapable } from "@bnto/registry";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { BntoRunStepper } from "./_components/BntoRunStepper";
import { RecipeHeroMascot } from "./_components/RecipeHeroMascot";
import { CliPromo } from "./_components/CliPromo";
import { HowItWorksSection } from "./_components/HowItWorksSection";
import { WhyBntoSection } from "./_components/WhyBntoSection";
import { CategoryFeaturesSection } from "./_components/CategoryFeaturesSection";
import { RelatedRecipesSection } from "./_components/RelatedRecipesSection";
import { RecipeFaqSection } from "./_components/RecipeFaqSection";

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
      <BntoJsonLd entry={entry} recipe={recipe ?? undefined} />
      <AppShellContent>
        <Container size="lg" className="space-y-6 text-center">
          {browserCapable ? (
            <BntoRunStepper entry={entry} />
          ) : (
            <>
              <Heading level={1} data-testid="recipe-heading">
                {entry.h1}
              </Heading>
              <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
                {entry.description}
              </p>
              <div className="relative">
                <CliPromo slug={slug} />
                <RecipeHeroMascot category={entry.category} />
              </div>
            </>
          )}
        </Container>

        {/* Below-the-fold SEO content */}
        {recipe && (
          <>
            <Divider label="How it works" />
            <HowItWorksSection recipe={recipe} />
          </>
        )}

        <Divider label="No catches" />
        <WhyBntoSection />

        <Divider label="Features" />
        <CategoryFeaturesSection category={entry.category} />

        <RelatedRecipesSection category={entry.category} currentSlug={entry.slug} />

        <Divider label="FAQ" />
        <RecipeFaqSection slug={entry.slug} category={entry.category} />
      </AppShellContent>
    </>
  );
}
