import { Heading } from "@bnto/ui";

export function BntoHero({ h1, description }: { h1: string; description: string }) {
  return (
    <>
      <Heading level={1} data-testid="recipe-heading">
        {h1}
      </Heading>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {description}
      </p>
    </>
  );
}
