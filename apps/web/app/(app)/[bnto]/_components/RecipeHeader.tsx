import { Heading } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";

/** Recipe heading + description — shared between bento and classic layouts. */
export function RecipeHeader({ entry }: { entry: BntoEntry }) {
  return (
    <div className="space-y-2 text-center">
      <Heading level={1} data-testid="recipe-heading">
        {entry.h1}
      </Heading>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>
    </div>
  );
}
