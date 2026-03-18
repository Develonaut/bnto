import { MyRecipesTabs } from "./_components/MyRecipesTabs";
import { AppShellContent, Heading, Stack, Text } from "@bnto/ui";

/**
 * My Recipes dashboard — server component.
 *
 * Static content (heading, description) renders on the server for faster
 * first paint. The interactive RecipeGrid lives in MyRecipesTabs — the
 * client boundary is pushed to the smallest leaf.
 */
export default function MyRecipesPage() {
  return (
    <AppShellContent>
      <Stack className="gap-8">
        <Stack className="gap-1">
          <Heading level={1} size="lg" data-testid="my-recipes-heading">
            My Recipes
          </Heading>
          <Text color="muted">Your saved recipes.</Text>
        </Stack>

        <MyRecipesTabs />
      </Stack>
    </AppShellContent>
  );
}
