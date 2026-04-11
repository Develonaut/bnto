import { FadeIn, Badge } from "@bnto/ui";

/** Below-fold SEO content — description, feature badges, and trust line. */
export function RecipeDetailsSection({
  description,
  features,
}: {
  description: string;
  features: string[];
}) {
  return (
    <FadeIn>
      <div className="mx-auto max-w-xl space-y-4 pt-8 text-center">
        <p className="text-muted-foreground leading-snug text-balance">{description}</p>
        <div className="flex flex-wrap justify-center gap-2">
          {features.map((f) => (
            <Badge key={f} variant="secondary">
              {f}
            </Badge>
          ))}
        </div>
        <p className="text-muted-foreground text-xs">
          Free. Open source. Files never leave your browser.
        </p>
      </div>
    </FadeIn>
  );
}
