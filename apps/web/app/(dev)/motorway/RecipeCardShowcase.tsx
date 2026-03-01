import { Animate } from "@/components/ui/Animate";
import { Grid } from "@/components/ui/Grid";
import { RecipeCard, RecipeCardSkeleton } from "@/components/blocks/RecipeCard";
import type { WorkflowListItem } from "@bnto/core";

/* ── Mock data ───────────────────────────────────────────────── */

const NOW = Date.now();
const HOUR = 3_600_000;
const DAY = 86_400_000;

const MOCK_RECIPES: Array<{
  recipe: WorkflowListItem;
  lastStatus?: "pending" | "running" | "completed" | "failed";
}> = [
  {
    recipe: { id: "1", name: "Compress & Resize", nodeCount: 3, updatedAt: NOW - 2 * HOUR },
    lastStatus: "completed",
  },
  {
    recipe: { id: "2", name: "Clean CSV Pipeline", nodeCount: 5, updatedAt: NOW - 3 * DAY },
    lastStatus: "failed",
  },
  {
    recipe: { id: "3", name: "Batch Rename", nodeCount: 2, updatedAt: NOW - 15 * DAY },
    lastStatus: "running",
  },
  {
    recipe: { id: "4", name: "Image Format Converter", nodeCount: 4, updatedAt: NOW - 45 * DAY },
  },
];

/* ── Showcase ────────────────────────────────────────────────── */

export function RecipeCardShowcase() {
  return (
    <Animate.Stagger asChild>
      <Grid cols={4} gap="md" animated>
        {MOCK_RECIPES.map((item, i) => (
          <Grid.Item key={item.recipe.id}>
            <Animate.ScaleIn index={i} from={0.85} easing="spring-bouncy">
              <RecipeCard recipe={item.recipe} lastStatus={item.lastStatus} />
            </Animate.ScaleIn>
          </Grid.Item>
        ))}
        <Grid.Item>
          <RecipeCardSkeleton />
        </Grid.Item>
      </Grid>
    </Animate.Stagger>
  );
}
