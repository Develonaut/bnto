"use client";

import { BookOpen, FolderOpen, Inbox, Plus, Search } from "lucide-react";

import {
  Stagger,
  ScaleIn,
  FadeIn,
  Button,
  Card,
  CardContent,
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  EmptyStateAction,
  EmptyStateSkeleton,
  Grid,
  GridItem,
  Heading,
  Stack,
  Text,
} from "@bnto/ui";

export function EmptyStateShowcase() {
  return (
    <Stack gap="lg">
      {/* Size variants */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">
            Size Variants
          </Heading>
          <Text size="sm" color="muted">
            Small, medium (default), and large — controls padding, icon size, and spacing.
          </Text>
        </div>
        <Stagger asChild>
          <Grid cols={3} gap="md" animated>
            {(["sm", "md", "lg"] as const).map((size, i) => (
              <GridItem key={size}>
                <ScaleIn index={i} from={0.85} easing="spring-bouncy">
                  <Card className="h-full">
                    <CardContent>
                      <EmptyState size={size}>
                        <EmptyStateIcon size={size}>
                          <Inbox />
                        </EmptyStateIcon>
                        <EmptyStateTitle>No items yet</EmptyStateTitle>
                        <EmptyStateDescription>
                          Get started by creating your first item.
                        </EmptyStateDescription>
                      </EmptyState>
                      <Text size="xs" color="muted" className="text-center">
                        size=&quot;{size}&quot;
                      </Text>
                    </CardContent>
                  </Card>
                </ScaleIn>
              </GridItem>
            ))}
          </Grid>
        </Stagger>
      </Stack>

      {/* With action */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">
            With Action
          </Heading>
          <Text size="sm" color="muted">
            Add a call-to-action button to guide users forward.
          </Text>
        </div>
        <Stagger asChild>
          <Grid cols={2} gap="md" animated>
            <GridItem>
              <ScaleIn index={0} from={0.85} easing="spring-bouncy">
                <Card className="h-full">
                  <CardContent>
                    <EmptyState>
                      <EmptyStateIcon>
                        <BookOpen />
                      </EmptyStateIcon>
                      <EmptyStateTitle>No recipes yet</EmptyStateTitle>
                      <EmptyStateDescription>
                        Create your first recipe to automate repetitive tasks.
                      </EmptyStateDescription>
                      <EmptyStateAction>
                        <Button>
                          <Plus className="size-4" />
                          New Recipe
                        </Button>
                      </EmptyStateAction>
                    </EmptyState>
                  </CardContent>
                </Card>
              </ScaleIn>
            </GridItem>
            <GridItem>
              <ScaleIn index={1} from={0.85} easing="spring-bouncy">
                <Card className="h-full">
                  <CardContent>
                    <EmptyState>
                      <EmptyStateIcon>
                        <Search />
                      </EmptyStateIcon>
                      <EmptyStateTitle>No results found</EmptyStateTitle>
                      <EmptyStateDescription>
                        Try adjusting your search terms or clearing filters.
                      </EmptyStateDescription>
                      <EmptyStateAction>
                        <Button variant="secondary">Clear Filters</Button>
                      </EmptyStateAction>
                    </EmptyState>
                  </CardContent>
                </Card>
              </ScaleIn>
            </GridItem>
          </Grid>
        </Stagger>
      </Stack>

      {/* Skeleton */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">
            Skeleton
          </Heading>
          <Text size="sm" color="muted">
            Placeholder while content state is loading.
          </Text>
        </div>
        <ScaleIn from={0.85} easing="spring-bouncy">
          <Card className="max-w-sm">
            <CardContent>
              <EmptyStateSkeleton />
            </CardContent>
          </Card>
        </ScaleIn>
      </Stack>

      {/* Inline (no card) */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">
            Inline (No Card)
          </Heading>
          <Text size="sm" color="muted">
            EmptyState works without a wrapping Card — useful for inline content areas.
          </Text>
        </div>
        <FadeIn>
          <EmptyState size="sm">
            <EmptyStateIcon size="sm">
              <FolderOpen />
            </EmptyStateIcon>
            <EmptyStateTitle>Empty folder</EmptyStateTitle>
            <EmptyStateDescription>Drop files here or click to upload.</EmptyStateDescription>
          </EmptyState>
        </FadeIn>
      </Stack>
    </Stack>
  );
}
