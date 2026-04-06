"use client";

import { useState } from "react";
import { Button, Card, Heading, Row, Skeleton, Stack, Text } from "@bnto/ui";

export function LoadingCardShowcase() {
  const [dormant, setDormant] = useState(true);

  return (
    <Stack gap="md">
      <Row gap="sm" className="items-center">
        <Button variant={dormant ? "secondary" : "outline"} onClick={() => setDormant((d) => !d)}>
          {dormant ? "Load Content" : "Reset to Dormant"}
        </Button>
        <Text size="sm" color="muted">
          {"<Card dormant>"} — bounciest spring by default
        </Text>
      </Row>

      <div className="grid grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <Card key={i} dormant={dormant} className="flex h-48 flex-col justify-between p-5">
            {dormant ? (
              <Stack gap="sm">
                <Skeleton className="h-5 w-2/3 rounded" />
                <Skeleton className="h-3 w-full rounded" />
                <Skeleton className="h-3 w-4/5 rounded" />
              </Stack>
            ) : (
              <Stack gap="sm">
                <Heading level={3} size="xs">
                  Card {i}
                </Heading>
                <Text size="sm" color="muted">
                  Content loaded. Sprung up from the ground plane.
                </Text>
              </Stack>
            )}
            <Text size="xs" color="muted" className="font-mono uppercase tracking-wider">
              {dormant ? "dormant" : "raised"}
            </Text>
          </Card>
        ))}
      </div>
    </Stack>
  );
}
