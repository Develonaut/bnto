import { MyRecipesTabs } from "./_components/MyRecipesTabs";
import { AppShell, Heading, Stack, Text } from "@bnto/ui";

/**
 * My Recipes dashboard — server component.
 *
 * Static content (heading, description) renders on the server for faster
 * first paint. The interactive part (tabs + data leaves) lives in
 * MyRecipesTabs — the client boundary is pushed to the smallest leaf.
 */
export default function MyRecipesPage() {
  return (
    <AppShell.Content>
      <Stack className="gap-8">
        <Stack className="gap-1">
          <Heading level={1} size="lg">
            My Recipes
          </Heading>
          <Text color="muted">
            Your saved recipes and recent activity.
          </Text>
        </Stack>

        <MyRecipesTabs />
      </Stack>
    </AppShell.Content>
  );
}
