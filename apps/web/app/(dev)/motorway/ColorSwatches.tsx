import { cn, Card, Grid } from "@bnto/ui";

function Swatch({
  name,
  bg,
  fg,
  surface,
}: {
  name: string;
  bg: string;
  fg: string;
  surface: string;
}) {
  return (
    <Card
      className={cn("flex h-full min-h-20 items-center justify-center rounded-xl", bg, fg, surface)}
    >
      <span className="text-sm font-semibold">{name}</span>
    </Card>
  );
}

export function ColorSwatches() {
  return (
    <Grid cols={6} rows={4} gap="md" animated>
      <Grid.Item colSpan={2} rowSpan={2}>
        <Swatch
          name="Primary"
          bg="bg-primary"
          fg="text-primary-foreground"
          surface="surface-primary"
        />
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={3}>
        <Swatch
          name="Secondary"
          bg="bg-secondary"
          fg="text-secondary-foreground"
          surface="surface-secondary"
        />
      </Grid.Item>
      <Grid.Item colSpan={2} colStart={5}>
        <Swatch name="Accent" bg="bg-accent" fg="text-accent-foreground" surface="surface-accent" />
      </Grid.Item>
      <Grid.Item colSpan={2} colStart={5} rowStart={2}>
        <Swatch name="Muted" bg="bg-muted" fg="text-muted-foreground" surface="surface-muted" />
      </Grid.Item>
      <Grid.Item rowSpan={2} rowStart={3}>
        <Swatch
          name="Destructive"
          bg="bg-destructive"
          fg="text-destructive-foreground"
          surface="surface-destructive"
        />
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={2} rowStart={3}>
        <Swatch
          name="Success"
          bg="bg-success"
          fg="text-success-foreground"
          surface="surface-success"
        />
      </Grid.Item>
      <Grid.Item rowSpan={2} colStart={4} rowStart={3}>
        <Swatch
          name="Warning"
          bg="bg-warning"
          fg="text-warning-foreground"
          surface="surface-warning"
        />
      </Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={5} rowStart={3}>
        <Swatch name="Card" bg="bg-card" fg="text-card-foreground" surface="surface-outline" />
      </Grid.Item>
    </Grid>
  );
}
