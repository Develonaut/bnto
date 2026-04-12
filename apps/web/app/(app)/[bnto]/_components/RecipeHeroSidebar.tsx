import { Badge, FadeIn, Heading, Text } from "@bnto/ui";
import { getCategoryMascot } from "../_utils/categoryMascot";

/**
 * Left sidebar for Steps 2 & 3 — H1, description, mascot, badges.
 *
 * Mobile: full-width banner — H1 left, mascot right, description below, tags hidden.
 * Desktop: vertical sidebar column with all elements stacked.
 */
export function RecipeHeroSidebar({
  h1,
  description,
  features,
  category,
}: {
  h1: string;
  description: string;
  features: string[];
  category: string;
}) {
  const src = getCategoryMascot(category);
  return (
    <div className="flex flex-col gap-3 text-left lg:gap-6">
      <div className="flex items-center gap-4">
        {/* Mobile mascot — inline with H1 + description */}
        <FadeIn className="shrink-0 lg:hidden">
          {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
          <img src={src} alt="" aria-hidden className="h-20 w-auto" />
        </FadeIn>
        <div className="min-w-0 flex-1">
          <Heading level={1} size="lg" data-testid="recipe-heading">
            {h1}
          </Heading>
          <Text size="sm" color="muted" leading="snug" className="mt-1.5">
            {description}
          </Text>
        </div>
      </div>
      <div className="hidden flex-wrap gap-2 lg:flex">
        {features.map((f) => (
          <Badge key={f} variant="secondary" size="sm">
            {f}
          </Badge>
        ))}
      </div>
      {/* Desktop mascot — large, centered */}
      <FadeIn className="hidden lg:block lg:self-center">
        {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
        <img src={src} alt="" aria-hidden className="h-56 w-auto" />
      </FadeIn>
    </div>
  );
}
