"use client";

import { Card, CardContent, Heading, Text } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";

interface RecipeInfoCardProps {
  entry: BntoEntry;
}

/** Title + description compartment — read-only in run mode. */
export function RecipeInfoCard({ entry }: RecipeInfoCardProps) {
  return (
    <Card elevation="sm" className="flex flex-col justify-center p-5">
      <CardContent className="space-y-2 p-0">
        <Heading level={2} className="text-lg">
          {entry.h1}
        </Heading>
        <Text size="sm" color="muted" className="leading-snug text-balance">
          {entry.description}
        </Text>
      </CardContent>
    </Card>
  );
}
