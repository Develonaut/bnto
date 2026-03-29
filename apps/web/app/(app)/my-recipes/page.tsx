import { AppShellContent, Heading, Stack, Text } from "@bnto/ui";

/**
 * My Recipes — placeholder until Sprint 8.5b (Favorites).
 *
 * Will be rebuilt as a favorites-backed page: favorited recipes
 * resolved via the registry, rendered as cards linking to /{slug}.
 */
export default function MyRecipesPage() {
  return (
    <AppShellContent>
      <Stack className="gap-4">
        <Heading level={1}>My Recipes</Heading>
        <Text color="muted">
          Favorites are coming soon. Browse available recipes on the Explore page.
        </Text>
      </Stack>
    </AppShellContent>
  );
}
