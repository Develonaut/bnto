import { Card } from "@/components/ui/Card";
import { Grid } from "@/components/ui/Grid";

export function CardShowcase() {
  return (
    <Grid cols={6} rows={3} gap="md" animated>
      <Grid.Item colSpan={2} rowSpan={1}>
        <Card
          elevation="none"
          className="flex h-full min-h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid="elevation-card-none"
        >
          none
        </Card>
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={3}>
        <Card
          elevation="sm"
          className="flex h-full min-h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid="elevation-card-sm"
        >
          sm
        </Card>
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={3} colStart={5}>
        <Card
          elevation="lg"
          className="flex h-full min-h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid="elevation-card-lg"
        >
          lg
        </Card>
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} rowStart={2}>
        <Card
          elevation="md"
          className="flex h-full min-h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid="elevation-card-md"
        >
          md (default)
        </Card>
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={1} colStart={3} rowStart={3}>
        <Card
          elevation="sm"
          className="flex h-full min-h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid="elevation-card-sm-2"
        >
          sm
        </Card>
      </Grid.Item>
    </Grid>
  );
}
