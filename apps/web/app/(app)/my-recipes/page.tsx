import { AppShellContent, Stack } from "@bnto/ui";

import { MyRecipesTabs } from "./_components/MyRecipesTabs";

/**
 * My Recipes dashboard — server component.
 *
 * The interactive content (heading + filter + grid) lives in MyRecipesTabs,
 * which is the client boundary. The filter dropdown needs client state,
 * so the heading moved into the client component to compose with the filter.
 */
export default function MyRecipesPage() {
  return (
    <AppShellContent>
      <Stack className="gap-8">
        <MyRecipesTabs />
      </Stack>
    </AppShellContent>
  );
}
