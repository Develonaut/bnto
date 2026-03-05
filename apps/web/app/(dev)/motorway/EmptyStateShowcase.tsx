import { BookOpen, FolderOpen, Inbox, Plus, Search } from "lucide-react";

import { Animate, Button, Card, EmptyState, Grid, Heading, Stack, Text } from "@bnto/ui";

export function EmptyStateShowcase() {
  return (
    <Stack gap="lg">
      {/* Size variants */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">Size Variants</Heading>
          <Text size="sm" color="muted">
            Small, medium (default), and large — controls padding, icon size, and spacing.
          </Text>
        </div>
        <Animate.Stagger asChild>
          <Grid cols={3} gap="md" animated>
            {(["sm", "md", "lg"] as const).map((size, i) => (
              <Grid.Item key={size}>
                <Animate.ScaleIn index={i} from={0.85} easing="spring-bouncy">
                  <Card className="h-full">
                    <Card.Content>
                      <EmptyState size={size}>
                        <EmptyState.Icon size={size}>
                          <Inbox />
                        </EmptyState.Icon>
                        <EmptyState.Title>No items yet</EmptyState.Title>
                        <EmptyState.Description>
                          Get started by creating your first item.
                        </EmptyState.Description>
                      </EmptyState>
                      <Text size="xs" color="muted" className="text-center">
                        size=&quot;{size}&quot;
                      </Text>
                    </Card.Content>
                  </Card>
                </Animate.ScaleIn>
              </Grid.Item>
            ))}
          </Grid>
        </Animate.Stagger>
      </Stack>

      {/* With action */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">With Action</Heading>
          <Text size="sm" color="muted">
            Add a call-to-action button to guide users forward.
          </Text>
        </div>
        <Animate.Stagger asChild>
          <Grid cols={2} gap="md" animated>
            <Grid.Item>
              <Animate.ScaleIn index={0} from={0.85} easing="spring-bouncy">
                <Card className="h-full">
                  <Card.Content>
                    <EmptyState>
                      <EmptyState.Icon>
                        <BookOpen />
                      </EmptyState.Icon>
                      <EmptyState.Title>No recipes yet</EmptyState.Title>
                      <EmptyState.Description>
                        Create your first recipe to automate repetitive tasks.
                      </EmptyState.Description>
                      <EmptyState.Action>
                        <Button>
                          <Plus className="size-4" />
                          New Recipe
                        </Button>
                      </EmptyState.Action>
                    </EmptyState>
                  </Card.Content>
                </Card>
              </Animate.ScaleIn>
            </Grid.Item>
            <Grid.Item>
              <Animate.ScaleIn index={1} from={0.85} easing="spring-bouncy">
                <Card className="h-full">
                  <Card.Content>
                    <EmptyState>
                      <EmptyState.Icon>
                        <Search />
                      </EmptyState.Icon>
                      <EmptyState.Title>No results found</EmptyState.Title>
                      <EmptyState.Description>
                        Try adjusting your search terms or clearing filters.
                      </EmptyState.Description>
                      <EmptyState.Action>
                        <Button variant="secondary">Clear Filters</Button>
                      </EmptyState.Action>
                    </EmptyState>
                  </Card.Content>
                </Card>
              </Animate.ScaleIn>
            </Grid.Item>
          </Grid>
        </Animate.Stagger>
      </Stack>

      {/* Skeleton */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">Skeleton</Heading>
          <Text size="sm" color="muted">
            Placeholder while content state is loading.
          </Text>
        </div>
        <Animate.ScaleIn from={0.85} easing="spring-bouncy">
          <Card className="max-w-sm">
            <Card.Content>
              <EmptyState.Skeleton />
            </Card.Content>
          </Card>
        </Animate.ScaleIn>
      </Stack>

      {/* Inline (no card) */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">Inline (No Card)</Heading>
          <Text size="sm" color="muted">
            EmptyState works without a wrapping Card — useful for inline content areas.
          </Text>
        </div>
        <Animate.FadeIn>
          <EmptyState size="sm">
            <EmptyState.Icon size="sm">
              <FolderOpen />
            </EmptyState.Icon>
            <EmptyState.Title>Empty folder</EmptyState.Title>
            <EmptyState.Description>
              Drop files here or click to upload.
            </EmptyState.Description>
          </EmptyState>
        </Animate.FadeIn>
      </Stack>
    </Stack>
  );
}
